use actix_web::dev::Server;
use actix_web::{App, HttpResponse, HttpServer, web};
use serde::ser::{Serialize, SerializeSeq, Serializer};
use std::net::TcpListener;


// impl<T> Serialize for Vec<T>
// where
//     T: Serialize,
// {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let mut seq = serializer.serialize_seq(Some(self.len()))?;
//         for element in self {
//             seq.serialize_element(element)?;
//         }
//         seq.end()
//     }
// }

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
}

// we always return 200 OK for now
async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
