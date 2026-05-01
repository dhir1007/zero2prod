use actix_web::{HttpResponse, web};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;
#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

// we always return 200 OK for now
pub async fn subscribe(_form: web::Form<FormData>, _db_pool: web::Data<PgPool>) -> HttpResponse {

    
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        "Adding a new subscriber..",
        %request_id,
        subscriber_email = %_form.email,
        subscriber_name = %_form.name,
    );
    // Using `enter` in an async function is a recipe for disaster!
    // Bear with me for now, but don't do this at home.
    // See the following section on `Instrumenting Futures`
    let _request_span_guard = request_span.enter();


    tracing::info!(
        "{request_id}: New subscription attempted: {email}, {name}",
        request_id = request_id,
        email = _form.email,
        name = _form.name
    );
    tracing::info!("{request_id}: request received");
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        _form.email,
        _form.name,
        Utc::now()
    )
    .execute(_db_pool.as_ref())
    .await
    {
        Ok(_) => {
            tracing::info!("{request_id}: New subscription successful");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!(
                "{request_id}:Failed to execute query: {error}",
                request_id = request_id,
                error = e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
