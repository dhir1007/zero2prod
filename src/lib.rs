pub mod configuration;
pub mod routes;
pub mod startup;



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




#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

// we always return 200 OK for now
async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
