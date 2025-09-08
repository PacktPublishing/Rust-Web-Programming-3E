use axum::routing::Router;
use std::net::SocketAddr;
use tokio::main;
use auth_dal::migrations::run_migrations;

mod api;
mod extract_auth;
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
    let addr = SocketAddr::from(([127, 0, 0, 1], 8081));
    println!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
