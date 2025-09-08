// -----------------------------------------------------------------------------
// Lightweight logging middleware for Axum, mirroring the Actix example you
// posted. Logs the request method, URI, and response status with `tracing`.
// -----------------------------------------------------------------------------
// Usage
// -----------------------------------------------------------------------------
// use axum::{Router, routing::get};
// use axum_logger::AxumLoggerLayer;
//
// #[tokio::main]
// async fn main() {
//     tracing_subscriber::fmt().init();
//
//     let app = Router::new()
//         .route("/", get(|| async { "Hello Axum" }))
//         .layer(AxumLoggerLayer); // <- plug it in here
//
//     axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
//         .serve(app.into_make_service())
//         .await
//         .unwrap();
// }
// -----------------------------------------------------------------------------
use axum::{
    body::Body,
    http::{Request, Response},
};
use futures::future::BoxFuture;
use std::task::{Context, Poll};
use tower::{Layer, Service};

/// A simple logging layer for Axum built on Tower.
#[derive(Clone, Default)]
pub struct AxumLoggerLayer;

impl<S> Layer<S> for AxumLoggerLayer {
    type Service = AxumLogger<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AxumLogger { inner }
    }
}

/// The service produced by `AxumLoggerLayer`.
#[derive(Clone)]
pub struct AxumLogger<S> {
    inner: S,
}

impl<S, ReqBody> Service<Request<ReqBody>> for AxumLogger<S>
where
    S: Service<Request<ReqBody>, Response = Response<Body>> + Clone + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Send + Sync + 'static,
    ReqBody: Send + 'static,
{
    type Response = Response<Body>;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        let method = req.method().clone();
        let uri = req.uri().clone();

        let fut = self.inner.call(req);

        Box::pin(async move {
            let res = fut.await?;
            tracing::info!("Request: {} {} {}", method, uri, res.status());
            Ok(res)
        })
    }
}
