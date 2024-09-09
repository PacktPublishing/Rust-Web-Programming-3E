//! The entry point for the axum server.
use axum::{
    routing::{get, Router},
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
};
use std::net::SocketAddr;
use tokio::main;

/// A basic handler that receives a request and returns a greeting.
/// 
/// # Arguments
/// * `name` - The name extracted from the request path
/// 
/// # Returns
/// A string response
async fn greet(Path(name): Path<String>) -> impl IntoResponse {
    let name = if name.is_empty() { "World".to_string() } else { name };
    (StatusCode::OK, format!("Hello {}!", name))
}

/// A basic handler that returns a greeting with no inputs.
/// 
/// # Returns
/// A string response
async fn say_hello() -> impl IntoResponse {
    "Hello Again!"
}

#[main]
async fn main() {
    // Build our application with a route
    let app = Router::new()
        .route("/", get(|| greet(Path("World".to_string()))))
        .route("/:name", get(greet))
        .route("/say/hello", get(say_hello));

    // Specify the address and port to bind the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
