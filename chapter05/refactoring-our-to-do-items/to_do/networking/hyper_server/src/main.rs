use std::convert::Infallible;
use std::net::SocketAddr;

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response, Method};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;


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
    let response = match (req.method(), path) {
        (&Method::GET, "/") => greet(Some("World".to_string())),
        (&Method::GET, "/say/hello") => say_hello(),
        (&Method::GET, path) if path.starts_with("/") => {
            let name = path.trim_start_matches('/').to_string();
            greet(Some(name))
        }
        _ => not_found(),
    };
    Ok(response)
}

/// Returns a greeting with the provided name.
/// 
/// # Arguments
/// * `name` - The name to greet
/// 
/// # Returns
/// A `Response` containing the greeting
fn greet(name: Option<String>) -> Response<Full<Bytes>> {
    let name = name.unwrap_or_else(|| "World".to_string());
    Response::new(Full::new(Bytes::from(format!("Hello, {}!", name))))
}

/// Returns a fixed greeting response.
fn say_hello() -> Response<Full<Bytes>> {
    Response::new(Full::new(Bytes::from("Hello Again!")))
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
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
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