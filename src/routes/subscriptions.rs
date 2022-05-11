use crate::domain::new_subscriber::NewSubscriber;
use crate::domain::subscriber_email::SubscriberEmail;
use crate::domain::subscriber_name::SubscriberName;
use actix_web::web::Form;
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

impl TryFrom<FormData> for NewSubscriber {
    type Error = String;

    fn try_from(value: FormData) -> Result<Self, Self::Error> {
        let name = SubscriberName::parse(value.name)?;
        let email = SubscriberEmail::parse(value.email)?;

        Ok(Self { email, name })
    }
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
    // form.0 refers to the underlying FormData
    let result = NewSubscriber::try_from(form.0); // same as: `form.0.try_into();`

    let new_subscriber = match result {
        Ok(new_subscriber) => new_subscriber,
        Err(_) => return HttpResponse::BadRequest(),
    };

    match insert_subscriber(&pool.get_ref(), new_subscriber).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}

#[tracing::instrument(skip(pool, new_subscriber))]
async fn insert_subscriber(
    pool: &PgPool,
    new_subscriber: NewSubscriber,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (email, name, subscribed_at)
        VALUES ($1, $2, now())
        "#,
        new_subscriber.email.as_ref(),
        new_subscriber.name.as_ref(),
        // Utc::now()
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
