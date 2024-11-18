use std::convert::Infallible;
use std::net::SocketAddr;

use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response, Method};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;
use http_body_util::Full;
use glue::hyper_utils::extract_header::extract_token;
use glue::extract_hyper_header_token;

mod api;


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
    let response = match (req.method(), segments.as_slice()) {
        (&Method::GET, ["api", "v1", "get", "all"]) => {
            api::basic_actions::get::get_all().await
        }
        // Example: match the "/api/v1/get/<name>" path
        (&Method::GET, ["api", "v1", "get", name]) => {
            // Here `name` is the extracted name segment from the URL
            api::basic_actions::get::get_by_name(name).await
        }
        (&Method::POST, ["api", "v1", "create"]) => {
            // Extract and parse the JSON body
            api::basic_actions::create::create(req).await
        }
        (&Method::PATCH, ["api", "v1", "update"]) => {
            let token = extract_hyper_header_token!(&req);
            // Extract and parse the JSON body
            api::basic_actions::update::update(req, token).await
        }
        (&Method::DELETE, ["api", "v1", "delete", name]) => {
            // Here `name` is the extracted name segment from the URL
            api::basic_actions::delete::delete_by_name(name).await
        }
        _ => Ok(not_found()),
    };
    match response {
        Ok(response) => Ok(response),
        Err(err) => {
            Ok(err.into_hyper_response())
        }
    }
}

/// Returns a 404 Not Found response.
fn not_found() -> Response<Full<Bytes>> {
    Response::builder()
        .status(404)
        .body(Full::new(Bytes::from("Not Found")))
        .unwrap()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
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