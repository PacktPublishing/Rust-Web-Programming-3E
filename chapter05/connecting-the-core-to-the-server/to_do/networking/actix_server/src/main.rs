//! The main entry point for the actix server
use actix_web::{web, App, HttpServer, Responder, HttpRequest};


mod api;


/// A basic handler that recieves a request and returns a greeting.
/// 
/// # Arguments
/// * `req` - The request object
/// 
/// # Returns
/// A string response
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", name)
}


#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .route("/", web::get().to(greet))
        .route("/{name}", web::get().to(greet))
        .route("/say/hello", web::get().to(|| 
                     async { "Hello Again!" }))
    })
    .workers(4)
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

