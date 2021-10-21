use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;
#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> impl Responder {
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        r#"Adding a new subscriber."#,
        %request_id,
        subscriber_email= %form.email, 
        subscriber_name=%form.name);

    let _request_span_guard = request_span.enter();

    tracing::info!(
        "RID:{}> Adding '{}' '{}' as a new subscriber.",
        request_id,
        form.email,
        form.name
    );
    tracing::info!(
        "RID:{}> Saving new subscriber details in the database",
        request_id
    );
    match sqlx::query!(
        r#"INSERT INTO subscriptions (id, email, name, subscribed_at)
                    VALUES ($1, $2, $3, $4)"#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.as_ref())
    .await
    {
        Ok(_) => {
            tracing::info!("RID:{}> New subscriber details saved in the DB", request_id);
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!("RID:{}> Failed to execute query: {:?}", request_id, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
