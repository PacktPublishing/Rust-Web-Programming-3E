use std::convert::Infallible;
use std::net::SocketAddr;

use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response, Method};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;
use http_body_util::Full;

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
    let response = match (req.method(), path) {
        (&Method::GET, "/api/v1/get/all") => api::basic_actions::get::get_all().await,
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