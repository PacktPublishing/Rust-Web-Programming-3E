use std::convert::Infallible;
use std::net::SocketAddr;

use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;
use http_body_util::Full;
use rust_embed::RustEmbed;
use std::path::Path;
use mime_guess;
use glue::errors::{NanoServiceError, NanoServiceErrorStatus};


#[derive(RustEmbed)]
#[folder = "../../frontend/public"]
struct FrontendAssets;


async fn serve_frontend_asset(path: String) -> Result<Response<Full<Bytes>>, NanoServiceError> {
    let file = match Path::new(&path).file_name() {
        Some(file) => file.to_str().unwrap(),
        None => return Err(NanoServiceError::new( 
            "404 Not Found".to_string(), 
            NanoServiceErrorStatus::NotFound
        )),
    };
    match FrontendAssets::get(file) {
        Some(content) => {
            let mime_type = mime_guess::from_path(file)
                .first_or_octet_stream()
                .as_ref()
                .to_string();
            let bytes = Bytes::from(content.data.to_vec());
            return Response::builder()
                .header("Content-Type", mime_type)
                .header("Cache-Control", "public, max-age=604800")
                .body(Full::new(bytes)).map_err(|e| NanoServiceError::new( 
                    e.to_string(), 
                    NanoServiceErrorStatus::Unknown
                ))
        }
        None => return Err(NanoServiceError::new( 
            "404 Not Found".to_string(), 
            NanoServiceErrorStatus::NotFound
        ))
    }
}

/// Handles an incoming request and routes it to the appropriate handler.
/// 
/// # Arguments
/// * `req` - The incoming request
/// 
/// # Returns
/// A `Result` containing the response to the request or an error
async fn handle(req: Request<hyper::body::Incoming>) 
    -> Result<Response<Full<Bytes>>, Infallible> {
    
    let path = req.uri().path();
    // Split the path into segments
    let segments: Vec<&str> = path.trim_start_matches('/').split('/').collect();

    // Match the path to extract `name` for "/api/v1/get/<name>"
    let response: Result<Response<Full<Bytes>>, NanoServiceError> = match (req.method(), segments.as_slice()) {
        _ => Ok(catch_all(path).await),
    };
    match response {
        Ok(response) => Ok(response),
        Err(err) => {
            Ok(err.into_hyper_response())
        }
    }
}

/// Returns a 404 Not Found response.
async fn catch_all(path: &str) -> Response<Full<Bytes>> {
    if path.contains("/api/") {
        return Response::builder()
            .status(404)
            .body(Full::new(Bytes::from("Not Found")))
            .unwrap()
    }
    if path.contains("frontend/public") {
        return serve_frontend_asset(path.to_string()).await.unwrap()
    }
    let file_type = mime_guess::from_path(&path).first_raw().unwrap_or("text/html");
    if !file_type.contains("text/html") {
        return serve_frontend_asset(path.to_string()).await.unwrap()
    }

    Response::builder()
        .status(404)
        .header("Content-Type", "text/html")
        .header("Cache-Control", "public, max-age=604800")
        .body(Full::new(Bytes::from(include_str!("../index.html"))))
        .unwrap()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8001));
    let listener = TcpListener::bind(addr).await?;
    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(handle))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}