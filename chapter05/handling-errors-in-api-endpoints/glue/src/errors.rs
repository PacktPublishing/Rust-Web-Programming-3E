//! Defines the error type passed between layers and servers.
use serde::{Deserialize, Serialize};
use thiserror::Error;
use std::fmt;

#[cfg(feature = "actix")]
use actix_web::{
    HttpResponse, 
    error::ResponseError, 
    http::StatusCode
};

#[cfg(feature = "rocket")]
use rocket::http::Status;
#[cfg(feature = "rocket")]
use rocket::response::{Responder, Response};
#[cfg(feature = "rocket")]
use rocket::Request;

#[cfg(feature = "axum")]
use axum::{
    response::{IntoResponse, Response as AxumResponse},
    http::StatusCode as AxumStatusCode,
};
use axum::Json;


/// The status of the error.
/// 
/// # Notes
/// This is used to determine the HTTP status code to return.
#[derive(Error, Debug, Serialize, Deserialize, PartialEq)]
pub enum NanoServiceErrorStatus {
    #[error("Requested resource was not found")]
    NotFound,
    #[error("You are forbidden to access requested resource.")]
    Forbidden,
    #[error("Unknown Internal Error")]
    Unknown,
    #[error("Bad Request")]
    BadRequest,
    #[error("Conflict")]
    Conflict,
    #[error("Unauthorized")]
    Unauthorized
}


/// The error struct that is passed between layers and servers.
/// 
/// # Fields
/// * `message` - The error message.
/// * `status` - The status of the error.
#[derive(Serialize, Deserialize, Debug, Error)]
pub struct NanoServiceError {
    pub message: String,
    pub status: NanoServiceErrorStatus
}
impl NanoServiceError {
    pub fn new(message: String, status: NanoServiceErrorStatus) 
        -> NanoServiceError {
        NanoServiceError {
            message,
            status
        }
    }
}


impl fmt::Display for NanoServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}


/// Implementing the ResponseError trait for actix_web servers.
#[cfg(feature = "actix")]
impl ResponseError for NanoServiceError {

    /// The status code for the actix web error response.
    /// 
    /// # Returns
    /// The status code for the error.
    fn status_code(&self) -> StatusCode {
        match self.status {
            NanoServiceErrorStatus::NotFound => 
                StatusCode::NOT_FOUND,
            NanoServiceErrorStatus::Forbidden => 
                StatusCode::FORBIDDEN,
            NanoServiceErrorStatus::Unknown => 
                StatusCode::INTERNAL_SERVER_ERROR,
            NanoServiceErrorStatus::BadRequest => 
                StatusCode::BAD_REQUEST,
            NanoServiceErrorStatus::Conflict => 
                StatusCode::CONFLICT,
            NanoServiceErrorStatus::Unauthorized => 
                StatusCode::UNAUTHORIZED
        }
    }
    
    /// Constructs an HTTP response for the error.
    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        HttpResponse::build(status_code).json(self.message.clone())
    }
    
}


#[cfg(feature = "rocket")]
#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for NanoServiceError {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        let status = match self.status {
            NanoServiceErrorStatus::NotFound => Status::NotFound,
            NanoServiceErrorStatus::Forbidden => Status::Forbidden,
            NanoServiceErrorStatus::Unknown => Status::InternalServerError,
            NanoServiceErrorStatus::BadRequest => Status::BadRequest,
            NanoServiceErrorStatus::Conflict => Status::Conflict,
            NanoServiceErrorStatus::Unauthorized => Status::Unauthorized,
        };

        Response::build()
            .status(status)
            .sized_body(self.message.len(), std::io::Cursor::new(self.message))
            .ok()
    }
}

/// Implementing the IntoResponse trait for Axum.
#[cfg(feature = "axum")]
impl IntoResponse for NanoServiceError {
    fn into_response(self) -> AxumResponse {
        let status_code = match self.status {
            NanoServiceErrorStatus::NotFound => AxumStatusCode::NOT_FOUND,
            NanoServiceErrorStatus::Forbidden => AxumStatusCode::FORBIDDEN,
            NanoServiceErrorStatus::Unknown => AxumStatusCode::INTERNAL_SERVER_ERROR,
            NanoServiceErrorStatus::BadRequest => AxumStatusCode::BAD_REQUEST,
            NanoServiceErrorStatus::Conflict => AxumStatusCode::CONFLICT,
            NanoServiceErrorStatus::Unauthorized => AxumStatusCode::UNAUTHORIZED,
        };
        
        (status_code, Json(self.message)).into_response()
    }
}