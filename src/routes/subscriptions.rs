use actix_web::web::Form;
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use tracing_actix_web::RequestId;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

// http --form POST localhost:8000/subscribe name=John email=john@example.com
// while true;do http --form POST localhost:8000/subscribe name=John email=john-$(date +%s)@example.com;sleep 5;done
pub async fn subscribe(form: Form<FormData>, connection: web::Data<PgPool>, request_id: RequestId) -> impl Responder {
    log::info!("[{request_id}] Saving subscriber (name={name}, email={email})", name=form.name, email=form.email, request_id=request_id);
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
            log::info!("[{}] Subscription saved successfully", request_id);
            HttpResponse::Ok()
        },
        Err(e) => {
            log::error!("[{}] Query execution failed: '{:?}'", request_id, e);
            HttpResponse::InternalServerError()
        }
    }
}
