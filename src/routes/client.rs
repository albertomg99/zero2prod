use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use sqlx::MySqlPool;
use uuid::Uuid;
#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[tracing::instrument(
    name = "Adding a new client",
    skip(form, pool),
    fields(subscriber_email = %form.email, subscriber_name = %form.name)
)]

pub async fn nou_client(form: web::Form<FormData>, pool: web::Data<MySqlPool>) -> impl Responder {
    match insert_client(&pool, &form).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(name = "Saving new client to DB", skip(form, pool))]

pub async fn insert_client(pool: &MySqlPool, form: &FormData) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"INSERT INTO subscriptions (id, email, name, subscribed_at)
                    VALUES (?, ?, ?, ?)"#,
    )
    .bind(Uuid::new_v4())
    .bind(form.email.clone())
    .bind(form.name.clone())
    .bind(Utc::now())
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}
