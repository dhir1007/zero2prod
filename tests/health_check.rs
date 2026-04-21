use std::net::TcpListener;
use zero2prod::run;

async fn main() -> std::io::Result<()>{
    run()?.await
}
fn spawn_app() {
    let server = zero2prod::run().expect("Failed to bind address");
    let _  = tokio::
}
#[tokio::test]
async fn health_check_works() {

    spawn_app();
    
    let _ = tokio::spawn(zero2prod::run(listener).expect("Failed to bind address"));
    // ... test logic
    spawn_app().await.expect("Failed");
    let client = reqwest::Client::new();

    let response = client
            .get("https:/127.0.0.1:8000/health_check")
            .send()
            .await
            .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() -> std::io::Result<()> {
    zero2prod::run().await
}
