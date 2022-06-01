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
    format!("Hello {name}! (endpoint #1)")
}

async fn hello2(name: web::Path<String>) -> impl actix_web::Responder {
    format!("Hello {name}! (endpoint #2)")
}

async fn hello_maud(name: web::Path<String>) -> actix_web::Result<maud::Markup> {
    Ok(maud::html! {
        html {
            body {
                h1 { "Hello " (name.into_inner()) "! (endpoint #3)" }
            }
        }
    })
}

#[derive(askama::Template)]
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    name: &'a str,
}

// I cannot set default values with web::Path<T>
async fn hello_askama_default() -> impl actix_web::Responder {
    askama_actix::TemplateToResponse::to_response(&HelloTemplate { name: "world" })
}
async fn hello_askama(name: web::Path<String>) -> impl actix_web::Responder {
    askama_actix::TemplateToResponse::to_response(&HelloTemplate { name: &name })
    // HelloTemplate { name: &name }.to_response() // I can get this nicer syntax if I use/import TemplateToResponse
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
            //
            // Main
            //
            .route("/", web::get().to(hello_askama_default))
            .route("/health", web::get().to(health_check))
            //
            // Subscribe
            //
            .route("/subscribe", web::post().to(subscribe))
            //
            // Hello
            //
            .service(hello1)
            .route("/hello2/{name}", web::get().to(hello2))
            .route("/hello3/{name}", web::get().to(hello_maud))
            .route("/hello4/{name}", web::get().to(hello_askama))
            // .service(hello_maud)
            // Get a pointer copy and attach it to the application state
            .app_data(pg_pool.clone())
            .app_data(pg_queue.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
