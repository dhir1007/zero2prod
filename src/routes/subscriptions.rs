use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use chrono::Utc;
use uuid::Uuid;
#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

// we always return 200 OK for now
pub async fn subscribe(
    _form: web::Form<FormData>,
    _db_pool: web::Data<PgPool>
) -> HttpResponse {
    log::info!("New subscription attempted: {:?}", _form.0);
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        _form.email,
        _form.name,
        Utc::now()
    ).execute(_db_pool.get_ref())
    .await
    {
        Ok(_) => {
            log::info!("New subscription successful");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            log::error!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}