//! The entry point for the axum server.
use axum::routing::Router;
use std::net::SocketAddr;
use tokio::main;
mod api;


#[main]
async fn main() {
    // Build our application with a route
    let app = Router::new();
    let app = api::views_factory(app);

    // Specify the address and port to bind the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
