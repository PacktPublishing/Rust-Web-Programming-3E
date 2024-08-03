use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use actix_web::dev::{Transform, Service};
use futures_util::future::{ok, Ready};
use std::task::{Context, Poll};
use std::pin::Pin;
use crate::logger::logger::log_info;


pub struct ActixLogger;

impl<S, B> Transform<S, ServiceRequest> for ActixLogger
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = LoggingMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(LoggingMiddleware { service })
    }
}

pub struct LoggingMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for LoggingMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn futures_util::Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            let req_info = format!(
                "{} {} {}",
                res.request().method(),
                res.request().uri(),
                res.status().as_str()
            );

            log_info(&format!("Request: {}", req_info)).await;

            Ok(res)
        })
    }
}
