use actix_web::{HttpResponse, Responder};

// http localhost:8080/health
pub(crate) async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("UP")
}
