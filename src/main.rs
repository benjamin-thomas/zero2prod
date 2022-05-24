use sqlx::PgPool;
use zero2prod::background_jobs::pg_queue::{run_worker, PgQueue};
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
    tokio::spawn(async move { run_worker(pg_queue).await });

    let pg_queue = PgQueue::new(pg_pool.clone());
    println!("\n--> Starting server on: \x1b[1;34m{}\x1b[1;m", addr);
    return run(listener, pg_pool, pg_queue)?.await;
}
