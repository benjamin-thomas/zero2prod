use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use sqlx::types::Json;
use sqlx::{Pool, Postgres};

/*
https://kerkour.com/rust-job-queue-with-postgresql
https://www.2ndquadrant.com/en/blog/what-is-select-skip-locked-for-in-postgresql-9-5/
https://www.crunchydata.com/blog/message-queuing-using-native-postgresql
 */

#[derive(Serialize, Deserialize)]
pub enum Message {
    SendConfirmEmail { email: String },
}

#[async_trait]
pub trait Queue: Send + Sync {
    async fn push(&self, job: Message) -> Result<(), sqlx::Error>;
    // async fn pull(&self, number_of_jobs: u32) -> Result<Vec<Job>, Box<dyn Error>>;
    // async fn delete_job(&self, job_id: u32) -> Result<(), Box<dyn Error>>;
    // async fn fail_job(&self, job_id: u32) -> Result<(), Box<dyn Error>>;
    // async fn clear(&self) -> Result<(), Box<dyn Error>>;
}

#[derive(Serialize, Deserialize)]
pub struct Job {
    pub id: u64,
    pub message: Message,
}

// We use a INT as Postgres representation for performance reasons --> FIXME: I'll probably remove this (use a Postgres enum or some FK)
#[repr(i32)]
#[derive(sqlx::Type)]
enum PostgresJobStatus {
    Queued,
    Running,
    Failed,
}

pub struct PostgresQueue {
    pool: Pool<Postgres>,
}

impl PostgresQueue {
    pub fn new(pool: Pool<Postgres>) -> PostgresQueue {
        PostgresQueue { pool }
    }
}

#[async_trait]
impl Queue for PostgresQueue {
    async fn push(&self, job: Message) -> Result<(), sqlx::Error> {
        let message = Json(job);
        let status = PostgresJobStatus::Queued;

        // Not sure why I have to cast to the type I already have
        sqlx::query!(
            r#"INSERT INTO queue (status, message) VALUES ($1, $2)"#,
            status as PostgresJobStatus,
            message as Json<Message>
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
