use axum::{
    response::{Html, IntoResponse, Response}, 
    routing::get, 
    Router,
    body::Body
};
use rust_embed::RustEmbed;
use std::net::SocketAddr;
use std::path::Path;
use mime_guess;
use tower_http::cors::{Any, CorsLayer};


async fn index() -> Html<&'static str> {
    Html(include_str!("../index.html"))
}


#[derive(RustEmbed)]
#[folder = "../../frontend/public"]
struct FrontendAssets;


async fn serve_frontend_asset(path: String) -> Response<Body> {
    let file = match Path::new(&path).file_name() {
        Some(file) => file.to_str().unwrap(),
        None => return axum::response::Response::builder()
                   .status(404)
                   .body::<Body>("404 Not Found".into())
                   .unwrap().into_response(),
    };
    match FrontendAssets::get(file) {
        Some(content) => {
            let mime_type = mime_guess::from_path(file)
                .first_or_octet_stream()
                .as_ref()
                .to_string();
            axum::response::Response::builder()
                .header("Content-Type", mime_type)
                .header("Cache-Control", "public, max-age=604800")
                .body::<Body>(content.data.into())
                .unwrap()
        }
        None => axum::response::Response::builder()
                .status(404)
                .body("404 Not Found".into())
                .unwrap(),
    }
}


async fn catch_all(uri: axum::http::Uri) -> impl IntoResponse {
    let path = uri.path().to_string();
    if path.contains("/api/") {
        return axum::response::Response::builder()
            .status(404)
            .body("Not Found".into())
            .unwrap();
    }
    if path.contains("frontend/public") {
        return serve_frontend_asset(path).await
    }
    let file_type = mime_guess::from_path(&path).first_raw().unwrap_or("text/html");
    if !file_type.contains("text/html") {
        return serve_frontend_asset(path).await
    }
    index().await.into_response()
}


#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any) // Allow all origins
        .allow_methods(Any) // Allow all methods (GET, POST, etc.)
        .allow_headers(Any); // Allow all headers

    let app = Router::new()
        .layer(cors)
        .route("/", get(index))
        .fallback(catch_all);
    let addr = SocketAddr::from(([0, 0, 0, 0], 8001));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
