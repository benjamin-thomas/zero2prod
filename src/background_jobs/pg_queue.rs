use crate::background_jobs::Job;
use async_trait::async_trait;
use serde_json::json;
use sqlx::{types::Json, Error, Pool, Postgres};

use super::{Message, Queue, JobStatus};

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
    async fn push(&self, job: Message) -> Result<(), Error> {
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

    async fn pull(&self, _batch_size: u8) -> Result<Vec<Job>, Error> {
        let jobs: Vec<PgJob> = sqlx::query_as!(
            PgJob,
            r#"
            UPDATE queue
            SET status = 'Running'
            WHERE id IN (
                SELECT id
                FROM queue
                WHERE status = $1
                ORDER BY id
                LIMIT 5
                FOR UPDATE SKIP LOCKED
            )
            RETURNING id, message AS "message: Json<Message>", status AS "status: PgJobStatus"
            "#, PgJobStatus::Queued as PgJobStatus
        )
        .fetch_all(&self.pool)
        .await?;

        let jobs = jobs.into_iter().map(Into::into).collect();
        Ok(jobs)
    }
}
