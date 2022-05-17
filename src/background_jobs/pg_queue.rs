use async_trait::async_trait;
use sqlx::{Postgres, Pool, types::Json};

use super::{Queue, Message};

// We use a INT as Postgres representation for performance reasons --> FIXME: I'll probably remove this (use a Postgres enum or some FK)
#[repr(i32)]
#[derive(sqlx::Type)]
enum PgJobStatus {
    Queued,
    Running,
    Failed,
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
    async fn push(&self, job: Message) -> Result<(), sqlx::Error> {
        let message = Json(job);
        let status = PgJobStatus::Queued;

        // Not sure why I have to cast to the type I already have
        sqlx::query!(
            r#"INSERT INTO queue (status, message) VALUES ($1, $2)"#,
            status as PgJobStatus,
            message as Json<Message>
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
