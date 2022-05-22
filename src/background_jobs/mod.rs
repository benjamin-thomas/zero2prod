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
    async fn push(&self, job: Message) -> Result<(), Box<dyn std::error::Error>>;
    async fn pull(&self, batch_size: u8) -> Result<Vec<Job>, Box<dyn std::error::Error>>;

    async fn delete_job(&self, job_id: u64) -> Result<(), Box<dyn std::error::Error>>;
    async fn fail_job(&self, job_id: u64) -> Result<(), Box<dyn std::error::Error>>;
}

#[derive(Serialize, Deserialize)]
pub struct Job {
    pub id: u64,
    pub status: JobStatus,
    pub message: Message,
}
