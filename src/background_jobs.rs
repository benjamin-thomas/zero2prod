#![allow(dead_code)]

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use sqlx::types::Json;
use sqlx::{Pool, Postgres};
use std::fmt::Debug;

/*
https://kerkour.com/rust-job-queue-with-postgresql
https://www.2ndquadrant.com/en/blog/what-is-select-skip-locked-for-in-postgresql-9-5/
https://www.crunchydata.com/blog/message-queuing-using-native-postgresql
 */

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message {
    SendConfirmEmail { email: String },
}

#[async_trait]
pub trait Queue: Send + Sync + Debug {
    async fn push(&self, job: Message) -> Result<(), sqlx::Error>;
    // async fn pull(&self, number_of_jobs: u32) -> Result<Vec<Job>, Box<dyn Error>>;
    // async fn delete_job(&self, job_id: u32) -> Result<(), Box<dyn Error>>;
    // async fn fail_job(&self, job_id: u32) -> Result<(), Box<dyn Error>>;
    // async fn clear(&self) -> Result<(), Box<dyn Error>>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub id: u64,
    pub message: Message,
}

// We use a INT as Postgres representation for performance reasons
#[derive(Debug, Clone, sqlx::Type, PartialEq)]
#[repr(i32)]
enum PostgresJobStatus {
    Queued,
    Running,
    Failed,
}

#[derive(sqlx::FromRow, Debug, Clone)]
struct PostgresJob {
    id: u64,
    status: PostgresJobStatus,
    message: Json<Message>,
}

impl From<PostgresJob> for Job {
    fn from(item: PostgresJob) -> Self {
        Job {
            id: item.id,
            message: item.message.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PostgresQueue {
    pool: Pool<Postgres>,
    max_attempts: u32,
}

impl PostgresQueue {
    pub fn new(pool: Pool<Postgres>) -> PostgresQueue {
        let queue = PostgresQueue {
            pool,
            max_attempts: 5,
        };

        queue
    }
}

#[async_trait]
impl Queue for PostgresQueue {
    async fn push(&self, job: Message) -> Result<(), sqlx::Error> {
        let message = Json(job);
        let status = PostgresJobStatus::Queued;

        // Not sure why I have to double cast
        sqlx::query!(r#"INSERT INTO queue (status, message) VALUES ($1, $2)"#, status as PostgresJobStatus, message as Json<Message>)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
    // async fn push(&self, job: Message)
}
