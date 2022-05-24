use sqlx::PgPool;
use zero2prod::background_jobs::pg_queue::{run_worker_once, PgQueue};
use zero2prod::background_jobs::{Message, Queue};

#[tokio::main]
async fn main() {
    let pg_pool = PgPool::connect(&zero2prod::config::get_conn_string())
        .await
        .expect("Could not connect to the database!");

    let pg_queue = PgQueue::new(pg_pool.clone());

    let job = Message::SendConfirmEmail {
        email: String::from("USE_MY_EMAIL_ADDR_HERE"),
    };

    pg_queue
        .push(job)
        .await
        .expect("could not push to the queue!");

    run_worker_once(pg_queue).await;
}
