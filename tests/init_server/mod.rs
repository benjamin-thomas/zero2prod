use once_cell::sync::Lazy;
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Pool, Postgres};
use std::future::Future;
use std::net::{SocketAddr, TcpListener};
use zero2prod::{config, startup, telemetry};

async fn init_solo_pool() -> Pool<Postgres> {
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&config::get_conn_string())
        .await
        .expect("Could not init pg pool");
    pool
}

async fn init_solo_pool_and_tx_start() -> Pool<Postgres> {
    let pool = init_solo_pool().await;

    sqlx::query("BEGIN")
        .execute(&pool)
        .await
        .expect("BEGIN tx failed");

    pool
}

// Ensure that the `tracing` stack is only initialized once using `once_cell`
static TRACING: Lazy<()> = Lazy::new(|| {
    telemetry::init("test");
});

async fn startup(with_tx: bool) -> (PgPool, SocketAddr) {
    // The first time this function is invoked the code in `TRACING` is executed.
    // All other invocations will instead skip execution.
    Lazy::force(&TRACING);

    let pool = if with_tx {
        init_solo_pool_and_tx_start().await
    } else {
        init_solo_pool().await
    };
    let listener = TcpListener::bind("localhost:0").expect("Failed to create listener");
    let socket = listener.local_addr().unwrap();
    let server = startup::run(listener, pool.clone()).expect("Could not start server");

    tokio::spawn(server);

    (pool, socket)
}

async fn rollback(pool: &PgPool) {
    sqlx::query("ROLLBACK")
        .execute(pool)
        .await
        .expect("ROLLBACK tx failed");
}

pub(crate) async fn with_tx<F>(test_body: fn(PgPool, SocketAddr) -> F)
where
    F: Future<Output = ()>,
{
    let (pool, socket) = startup(true).await;
    test_body(pool.clone(), socket).await;
    rollback(&pool).await;
}

pub(crate) async fn no_tx<F>(test_body: fn(PgPool, SocketAddr) -> F)
where
    F: Future<Output = ()>,
{
    let (pool, socket) = startup(true).await;
    test_body(pool.clone(), socket).await;
}
