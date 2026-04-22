use actix_web::dev::Server;
use actix_web::{App, HttpResponse, HttpServer, web};
use std::net::TcpListener;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)
    .unwrap()
    .run();
    Ok(server)
}
#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
    a: i32,
    b: i32,
}
async fn subscribe(form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn add(form: web::Form<FormData>) -> HttpResponse {
    let sum = form.0.a + form.0.b;
    HttpResponse::Ok().body(sum.to_string())
}
