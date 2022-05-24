use std::time::Duration;

use crate::{
    background_jobs::Job, config::must_env, domain::subscriber_email::SubscriberEmail,
    email_client::EmailClient,
};
use async_trait::async_trait;
use serde_json::json;
use sqlx::{types::Json, Pool, Postgres};
use tokio_stream::StreamExt;

use super::{JobStatus, Message, Queue};

#[derive(sqlx::Type)]
#[sqlx(type_name = "QUEUE_STATUS")]
enum PgJobStatus {
    Queued,
    Running,
    Failed,
}

impl From<PgJobStatus> for JobStatus {
    fn from(item: PgJobStatus) -> Self {
        match item {
            PgJobStatus::Queued => JobStatus::Queued,
            PgJobStatus::Running => JobStatus::Running,
            PgJobStatus::Failed => JobStatus::Failed,
        }
    }
}

struct PgJob {
    id: i64,
    status: PgJobStatus,
    message: Json<Message>,
}

impl From<PgJob> for Job {
    fn from(item: PgJob) -> Self {
        Job {
            id: u64::try_from(item.id).expect("number conversion failed!"),
            status: item.status.into(),
            message: item.message.0,
        }
    }
}

pub struct PgQueue {
    pool: Pool<Postgres>,
}

impl PgQueue {
    pub fn new(pool: Pool<Postgres>) -> PgQueue {
        PgQueue { pool }
    }
}

#[async_trait]
impl Queue for PgQueue {
    async fn push(&self, job: Message) -> Result<(), Box<dyn std::error::Error>> {
        let message = json!(job);

        sqlx::query!(
            r#"INSERT INTO queue (status, message) VALUES ($1, $2)"#,
            PgJobStatus::Queued as PgJobStatus,
            message,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn pull(&self, batch_size: u8) -> Result<Vec<Job>, Box<dyn std::error::Error>> {
        // TODO: I'll have to investigate if not dealing with TIMESTAMPTZ makes sens
        //
        // NOTE: I see no reason not to hard-code failed_attempts at this point
        //
        // NOTE: Since the queue is meant to stay mostly empty, it's not clear whether creating indices
        //       would improve or hurt performance.
        let jobs: Vec<PgJob> = sqlx::query_as!(
            PgJob,
            r#"
            UPDATE queue
               SET status = 'Running'
                 , updated_at = current_timestamp AT TIME ZONE 'UTC'

            WHERE id IN (
                SELECT id
                  FROM queue

                 WHERE status = $1
                   AND scheduled_at <= current_timestamp AT TIME ZONE 'UTC'
                   AND failed_attempts <= 3

              ORDER BY
                  priority DESC
                , scheduled_at ASC

              LIMIT $2

                FOR UPDATE SKIP LOCKED
            )
            RETURNING id, message AS "message: Json<Message>", status AS "status: PgJobStatus"
            "#,
            PgJobStatus::Queued as PgJobStatus, // keeping for ref, I could just hard-code the value...
            i64::from(batch_size)
        )
        .fetch_all(&self.pool)
        .await?;

        let jobs = jobs.into_iter().map(Into::into).collect();
        Ok(jobs)
    }

    async fn delete_job(&self, job_id: u64) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query!(
            r#"DELETE FROM queue WHERE id = $1"#,
            i64::try_from(job_id).expect("number conversion failed!!")
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn fail_job(&self, job_id: u64) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query!(
            r#"
            UPDATE queue
               SET status = 'Failed'
                 , failed_attempts = failed_attempts + 1
                 , updated_at = current_timestamp AT TIME ZONE 'UTC'
             WHERE id = $1
            "#,
            i64::try_from(job_id).expect("number conversion failed!!")
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

// Set to run max 100 jobs per second (so about 8M jobs per day)
pub async fn run_worker(pg_queue: PgQueue) {
    let smtp_host = must_env("SMTP_HOST");
    let smtp_sender = must_env("SMTP_SENDER");
    let smtp_password = must_env("SMTP_PASSWORD");

    let email_client = EmailClient::new(smtp_host, smtp_sender, smtp_password);

    loop {
        handle_batch(&pg_queue, &email_client).await;

        // Poll a batch (100 jobs) every 1s
        tokio::time::sleep(Duration::from_millis(1000)).await;
    }
}

pub async fn run_worker_once(pg_queue: PgQueue) {
    let smtp_host = must_env("SMTP_HOST");
    let smtp_sender = must_env("SMTP_SENDER");
    let smtp_password = must_env("SMTP_PASSWORD");

    let email_client = EmailClient::new(smtp_host, smtp_sender, smtp_password);

    handle_batch(&pg_queue, &email_client).await;
}

async fn handle_batch(pg_queue: &PgQueue, email_client: &EmailClient) {
    let jobs = match pg_queue.pull(100).await {
        Ok(jobs) => jobs,
        Err(err) => {
            println!("Failed to pull job: {}", err);
            println!("Will retry in 10s");
            std::thread::sleep(Duration::from_millis(10000));
            Vec::new() // do not use early `return` or run_worker will never run again
        }
    };
    let mut stream = tokio_stream::iter(jobs);
    while let Some(job) = stream.next().await {
        println!("Handling job #{}", job.id);
        match job.message {
            Message::SendConfirmEmail { email } => {
                let email = SubscriberEmail::parse(email).unwrap();
                let res = email_client
                    .send_email(email, "hello", "html bogus", "txt bogus")
                    .await;

                match res {
                    Ok(_) => pg_queue.delete_job(job.id).await.unwrap(),
                    Err(err) => {
                        println!("Failed job #{}! {}", job.id, err);
                        pg_queue.fail_job(job.id).await.unwrap()
                    }
                }
            }
        }
    }
}
