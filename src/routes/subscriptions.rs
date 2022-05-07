use actix_web::web::Form;
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;
use tracing::Instrument;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

// http --form POST localhost:8000/subscribe name=John email=john@example.com
// while true;do http --form POST localhost:8000/subscribe name=John email=john-$(date +%s)@example.com;sleep 5;done
pub async fn subscribe(form: Form<FormData>, pool: web::Data<PgPool>) -> impl Responder {
    let request_id = Uuid::new_v4();
    let req_span = tracing::info_span!("Adding a new subscriber", %request_id, %form.name, sub_email = %form.email, f = ?form.name);
    let _req_span_guard = req_span.enter();

    let query_span = tracing::info_span!("Saving the new subscriber");
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
    .execute(pool.get_ref())
    .instrument(query_span)
    .await;

    match res {
        Ok(_) => {
            tracing::info!("Subscription saved successfully");
            HttpResponse::Ok()
        },
        Err(e) => {
            tracing::error!("Query execution failed: '{:?}'", e);
            HttpResponse::InternalServerError()
        }
    }
}
