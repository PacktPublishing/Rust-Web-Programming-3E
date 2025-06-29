//! The entry point for the axum server.
use axum::routing::Router;
use std::net::SocketAddr;
use tokio::main;
mod api;
use to_do_dal::migrations::run_migrations;
use glue::logger::{
    network_wrappers::axum::AxumLoggerLayer,
    logger::init_logger
};


#[main]
async fn main() {
    init_logger();
    run_migrations().await;
    // Build our application with a route
    let app = Router::new();
    let app = api::views_factory(app).layer(AxumLoggerLayer);

    // Specify the address and port to bind the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
