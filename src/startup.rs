use crate::background_jobs::pg_queue::PgQueue;
use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

#[actix_web::get("/hello/{name}")]
async fn hello1(name: web::Path<String>) -> impl actix_web::Responder {
    format!("Hello {name}! (1)")
}

async fn hello2(name: web::Path<String>) -> impl actix_web::Responder {
    format!("Hello {name}! (2)")
}

async fn hello_maud(name: web::Path<String>) -> actix_web::Result<maud::Markup> {
    Ok(maud::html! {
        html {
            body {
                h1 { "Hello " (name.into_inner()) "!" }
            }
        }
    })
}

pub fn run(
    listener: TcpListener,
    pg_pool: PgPool,
    pg_queue: PgQueue,
) -> Result<Server, std::io::Error> {
    // Wrap the connection in a smart pointer
    let pg_pool = web::Data::new(pg_pool);

    let pg_queue = Data::new(pg_queue);

    // Capture `connection` from the surrounding environment
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health", web::get().to(health_check))
            .route("/subscribe", web::post().to(subscribe))
            .route("/hello2/{name}", web::get().to(hello2))
            .service(hello1)
            .route("/hello3/{name}", web::get().to(hello_maud))
            // .service(hello_maud)
            // Get a pointer copy and attach it to the application state
            .app_data(pg_pool.clone())
            .app_data(pg_queue.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
