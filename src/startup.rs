use crate::background_jobs::pg_queue::PgQueue;
use crate::email_client::EmailClient;
use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub fn run(
    listener: TcpListener,
    pg_pool: PgPool,
    pg_queue: PgQueue,
    email_client: EmailClient,
) -> Result<Server, std::io::Error> {
    // Wrap the connection in a smart pointer
    let pg_pool = web::Data::new(pg_pool);
    let queue = web::Data::new(pg_queue);
    let email_client = web::Data::new(email_client);

    // Capture `connection` from the surrounding environment
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health", web::get().to(health_check))
            .route("/subscribe", web::post().to(subscribe))
            // Get a pointer copy and attach it to the application state
            .app_data(pg_pool.clone())
            .app_data(queue.clone())
            .app_data(email_client.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
