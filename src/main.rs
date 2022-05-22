use std::time::{Duration, UNIX_EPOCH};

use actix_web::web::Data;
use sqlx::PgPool;
use tokio_stream::StreamExt;
use zero2prod::background_jobs::pg_queue::PgQueue;
use zero2prod::background_jobs::{Job, Message, Queue};
use zero2prod::email_client::EmailClient;
use zero2prod::{run, telemetry};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    telemetry::init("zero2prod");

    let bind = zero2prod::config::must_env("BIND");
    let smtp_host = zero2prod::config::must_env("SMTP_HOST");
    let smtp_sender = zero2prod::config::must_env("SMTP_SENDER");

    let email_client = EmailClient::new(smtp_host, smtp_sender).expect("EmailClient init failed");

    let listener = std::net::TcpListener::bind(&bind)
        .unwrap_or_else(|_| panic!("Could not bind to: {}", bind));

    let addr = listener.local_addr().unwrap();

    let pg_pool = PgPool::connect(&zero2prod::config::get_conn_string())
        .await
        .expect("Could not connect to the database!");

    let pg_queue = PgQueue::new(pg_pool.clone());
    let pg_queue = Data::new(pg_queue);
    let worker_queue = pg_queue.clone();
    tokio::spawn(async move { run_worker(worker_queue).await });

    println!("\n--> Starting server on: \x1b[1;34m{}\x1b[1;m", addr);
    return run(listener, pg_pool, pg_queue, email_client)?.await;
}

// Set to run max 100 jobs per second (so about 8M jobs per day)
async fn run_worker(pg_queue: Data<PgQueue>) {
    loop {
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
            handle_job(pg_queue.clone(), job).await;
        }

        // Poll a batch (100 jobs) every 1s
        tokio::time::sleep(Duration::from_millis(1000)).await;
    }
}

async fn handle_job(pg_queue: Data<PgQueue>, job: Job) {
    match job.message {
        Message::SendConfirmEmail { email } => match send_email_fake(email) {
            Ok(_) => pg_queue.delete_job(job.id).await.unwrap(),
            Err(_) => pg_queue.fail_job(job.id).await.unwrap(),
        },
    }
}

enum SendEmailError {
    BogusError,
}

fn send_email_fake(email: String) -> Result<(), SendEmailError> {
    let now = std::time::SystemTime::now();
    let since_epoch = now.duration_since(UNIX_EPOCH).unwrap();

    if since_epoch.as_secs() % 10 == 0 {
        println!("FAILED to send email to: {}!", email);
        Err(SendEmailError::BogusError)
    } else {
        println!("Sent email to: {}!", email);
        Ok(())
    }
}
