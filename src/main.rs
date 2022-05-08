use sqlx::PgPool;
use zero2prod::{run, telemetry};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    telemetry::init("zero2prod");

    let bind = zero2prod::config::must_env("BIND");

    let listener =
        std::net::TcpListener::bind(&bind).unwrap_or_else(|_| panic!("Could not bind to: {}", bind));

    let addr = listener.local_addr().unwrap();

    let pg_pool = PgPool::connect(&zero2prod::config::get_conn_string())
        .await
        .expect("Could not connect to the database!");

    println!("\n--> Starting server on: \x1b[1;34m{}\x1b[1;m", addr);
    return run(listener, pg_pool)?.await;
}
