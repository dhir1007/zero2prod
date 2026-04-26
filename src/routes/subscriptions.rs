use actix_web::{web, HttpResponse};
#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

// we always return 200 OK for now
pub async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}