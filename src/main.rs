use std::time::Duration;

use actix_web::web::Data;
use sqlx::PgPool;
use tokio_stream::StreamExt;
use zero2prod::background_jobs::pg_queue::PgQueue;
use zero2prod::background_jobs::{Message, Queue};
use zero2prod::domain::subscriber_email::SubscriberEmail;
use zero2prod::email_client::EmailClient;
use zero2prod::{run, telemetry};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    telemetry::init("zero2prod");

    let bind = zero2prod::config::must_env("BIND");

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
    return run(listener, pg_pool, pg_queue)?.await;
}

// Set to run max 100 jobs per second (so about 8M jobs per day)
async fn run_worker(pg_queue: Data<PgQueue>) {
    let smtp_host = zero2prod::config::must_env("SMTP_HOST");
    let smtp_sender = zero2prod::config::must_env("SMTP_SENDER");
    let smtp_password = zero2prod::config::must_env("SMTP_PASSWORD");

    let email_client = EmailClient::new(smtp_host, smtp_sender, smtp_password);

    loop {
        println!("PULLING!");
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
                    match email_client
                        .send_email(email, "hello", "html bogus", "txt bogus")
                        .await
                    {
                        Ok(_) => pg_queue.delete_job(job.id).await.unwrap(),
                        Err(_) => pg_queue.fail_job(job.id).await.unwrap(),
                    }
                }
            }
        }

        // Poll a batch (100 jobs) every 1s
        tokio::time::sleep(Duration::from_millis(1000)).await;
    }
}
