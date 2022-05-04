use actix_web::web::Form;
use actix_web::{HttpResponse, Responder};

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

// http --form POST localhost:8000/subscribe name=John email=john@example.com
pub async fn subscribe(_form: Form<FormData>) -> impl Responder {
    HttpResponse::Ok().finish()
}
