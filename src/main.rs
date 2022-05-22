use std::time::Duration;

use sqlx::PgPool;
use zero2prod::background_jobs::pg_queue::PgQueue;
use zero2prod::background_jobs::Queue;
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
    let pg_queue = actix_web::web::Data::new(pg_queue);
    let worker_queue = pg_queue.clone();
    tokio::spawn(async move { run_worker(worker_queue).await });

    println!("\n--> Starting server on: \x1b[1;34m{}\x1b[1;m", addr);
    return run(listener, pg_pool, pg_queue, email_client)?.await;
}

async fn run_worker(pg_queue: actix_web::web::Data<PgQueue>) {
    loop {
        let jobs = match pg_queue.pull(10).await {
            Ok(jobs) => jobs,
            Err(err) => {
                println!("Failed to pull job: {}", err);
                println!("Will retry in 10s");
                std::thread::sleep(Duration::from_millis(10000));
                Vec::new() // do not use early `return` or run_worker will never run again
            }
        };

        // sleep not to overload our database
        println!("Hello from queue");
        for j in jobs {
            println!("> Would work on job #{}", j.id);
        }
        tokio::time::sleep(Duration::from_millis(1000)).await;
    }
}
