use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub mod pg_queue;

/*
https://kerkour.com/rust-job-queue-with-postgresql
https://www.2ndquadrant.com/en/blog/what-is-select-skip-locked-for-in-postgresql-9-5/
https://www.crunchydata.com/blog/message-queuing-using-native-postgresql
 */

#[derive(Serialize, Deserialize)]
pub enum Message {
    SendConfirmEmail { email: String },
}

#[derive(Serialize, Deserialize)]
pub enum JobStatus {
    Queued,
    Running,
    Failed,
}

#[async_trait]
pub trait Queue: Send + Sync {
    async fn push(&self, job: Message) -> Result<(), sqlx::Error>; // don't know how to return generic errors yet
    async fn pull(&self, batch_size: u8) -> Result<Vec<Job>, sqlx::Error>;

    // async fn delete_job(&self, job_id: u32) -> Result<(), Box<dyn Error>>;
    // async fn fail_job(&self, job_id: u32) -> Result<(), Box<dyn Error>>;
    // async fn clear(&self) -> Result<(), Box<dyn Error>>;
}

#[derive(Serialize, Deserialize)]
pub struct Job {
    pub id: i64,
    pub status: JobStatus,
    pub message: Message,
}
