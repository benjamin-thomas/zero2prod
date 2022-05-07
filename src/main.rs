use env_logger::Env;
use sqlx::PgPool;
use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let listener = std::net::TcpListener::bind("localhost:8000").expect("Could not bind port 8000");
    let addr = listener.local_addr().unwrap();

    let pg_pool = PgPool::connect(&zero2prod::config::get_conn_string())
        .await
        .expect("Could not connect to the database!");

    println!("\n--> Starting server on: \x1b[1;34m{}\x1b[1;m", addr);
    return run(listener, pg_pool)?.await;
}
