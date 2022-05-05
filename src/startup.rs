use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, pg_pool: PgPool) -> Result<Server, std::io::Error> {
    // Wrap the connection in a smart pointer
    let pg_pool = web::Data::new(pg_pool);

    // Capture `connection` from the surrounding environment
    let server = HttpServer::new( move || {
        App::new()
            .route("/health", web::get().to(health_check))
            .route("/subscribe", web::post().to(subscribe))
            // Get a pointer copy and attach it to the application state
            .app_data(pg_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
