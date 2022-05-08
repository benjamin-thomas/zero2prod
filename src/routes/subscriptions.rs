use actix_web::web::{Form};
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

// http --form POST localhost:8000/subscribe name=Robert email=bob@example.com
// while true;do http --form POST localhost:8000/subscribe name=John email=john-$(date +%s)@example.com;sleep 5;done
#[tracing::instrument(
    name = "Register a new subscriber", // defaults to function name
    skip(form, pool),
    fields(
        user_name = %form.name,
        %form.email,
    )
)]
pub async fn subscribe(form: Form<FormData>, pool: web::Data<PgPool>) -> impl Responder {
    match insert_subscriber(pool.get_ref(), &form).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}

#[tracing::instrument(skip(form, pool))]
async fn insert_subscriber(pool: &PgPool, form: &FormData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (email, name, subscribed_at)
        VALUES ($1, $2, now())
        "#,
        form.email,
        form.name,
    )
    // We use `get_ref` to get an immutable reference to the `PgConnection`
    // wrapped by `web::Data`.
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Query execution failed: '{:?}'", e);
        e
    })?; // `?` operator returns the error early

    Ok(())
}
