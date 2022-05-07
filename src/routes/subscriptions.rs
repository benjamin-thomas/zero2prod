use actix_web::web::Form;
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

// http --form POST localhost:8000/subscribe name=John email=john@example.com
pub async fn subscribe(form: Form<FormData>, connection: web::Data<PgPool>) -> impl Responder {
    log::info!("Saving subscriber (name={name}, email={email})", name=form.name, email=form.email);
    let res = sqlx::query!(
        r#"
        INSERT INTO subscriptions (email, name, subscribed_at)
        VALUES ($1, $2, now())
        "#,
        form.email,
        form.name,
    )
    // We use `get_ref` to get an immutable reference to the `PgConnection`
    // wrapped by `web::Data`.
    .execute(connection.get_ref())
    .await;

    match res {
        Ok(_) => {
            log::info!("Subscription saved successfully");
            HttpResponse::Ok()
        },
        Err(e) => {
            log::error!("Query execution failed: '{:?}'", e);
            HttpResponse::InternalServerError()
        }
    }
}
