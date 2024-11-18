use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct InputData {
    message: String,
}

async fn post_handler(data: web::Json<InputData>) -> impl Responder {
    println!("Received: {:?}", data);
    let response_message = format!("Received: {}", data.message);
    HttpResponse::Ok().json(serde_json::json!({"response": response_message}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut builder = SslAcceptor::mozilla_intermediate(
        SslMethod::tls()
    ).unwrap();
    builder.set_private_key_file("key.pem", SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    HttpServer::new(|| {
        App::new()
        .route("/api/data", web::post().to(post_handler))
    })
    .bind_openssl("127.0.0.1:8080", builder)?
    .run()
    .await
}
