use std::convert::Infallible;
use std::net::SocketAddr;

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

use hyper::server::conn::http2;
use hyper::rt::Executor;
use std::future::Future;


/// A custom executor that uses Tokio's `tokio::spawn` to spawn futures for
/// the hyper server.
#[derive(Clone)]
struct TokioExecutor;

impl<F> Executor<F> for TokioExecutor
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    fn execute(&self, future: F) {
        tokio::spawn(future);
    }
}




/// Handles an incoming request.
/// 
/// # Notes
/// If you want to route requests to different functions, you can do so in this function.
/// 
/// # Arguments
/// * `body` - The body of the incoming request
/// 
/// # Returns
/// A `Result` containing the response to the request or an error
async fn handle(body: Request<hyper::body::Incoming>) 
    -> Result<Response<Full<Bytes>>, Infallible> {
    println!("Received a request");
    println!("Headers: {:?}", body.headers());
    println!("Body: {:?}", body.into_body());
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}


#[tokio::main]
async fn main() 
    -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        tokio::task::spawn(async move {
            if let Err(err) = http2::Builder::new(TokioExecutor)
                .serve_connection(io, service_fn(handle))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}

