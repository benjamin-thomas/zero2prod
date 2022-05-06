async fn init_solo_pool() -> sqlx::Pool<sqlx::Postgres> {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect(&zero2prod::config::get_conn_string())
        .await
        .expect("Could not init pg pool");
    pool
}

async fn init_solo_pool_and_tx_start() -> sqlx::Pool<sqlx::Postgres> {
    let pool = init_solo_pool().await;

    sqlx::query("BEGIN")
        .execute(&pool)
        .await
        .expect("BEGIN tx failed");

    pool
}

async fn startup(with_tx: bool) -> (sqlx::PgPool, u16) {
    let pool = if with_tx {
        init_solo_pool_and_tx_start().await
    } else {
        init_solo_pool().await
    };
    let listener = std::net::TcpListener::bind("localhost:0").expect("Failed to create listener");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::startup::run(listener, pool.clone()).expect("Could not start server");

    tokio::spawn(server);

    (pool, port)
}

async fn rollback(pool: &sqlx::PgPool) {
    sqlx::query("ROLLBACK")
        .execute(pool)
        .await
        .expect("ROLLBACK tx failed");
}

pub async fn with_tx<F>(test_body: fn(pool: sqlx::PgPool, port: u16) -> F)
where
    F: std::future::Future<Output = ()>,
{
    let (pool, port) = startup(true).await;
    test_body(pool.clone(), port).await;
    rollback(&pool).await;
}

pub async fn no_tx<F>(test_body: fn(pool: sqlx::PgPool, port: u16) -> F)
where
    F: std::future::Future<Output = ()>,
{
    let (pool, port) = startup(true).await;
    test_body(pool.clone(), port).await;
}
