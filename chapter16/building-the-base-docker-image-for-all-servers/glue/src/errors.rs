use serde::{Deserialize, Serialize};
use thiserror::Error;
use std::fmt;

#[cfg(feature = "actix")]
use actix_web::{HttpResponse, error::ResponseError, http::StatusCode};


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

impl NanoServiceErrorStatus {
    /// Constructs an error status from a numeric code.
    /// 
    /// # Arguments
    /// * `code` - The numeric code representing the error status.
    /// 
    /// # Returns
    /// * `NanoServiceErrorStatus` - The corresponding error status.
    pub fn from_code(code: u16) -> NanoServiceErrorStatus {
        match code {
            404 => NanoServiceErrorStatus::NotFound,
            403 => NanoServiceErrorStatus::Forbidden,
            400 => NanoServiceErrorStatus::BadRequest,
            409 => NanoServiceErrorStatus::Conflict,
            401 => NanoServiceErrorStatus::Unauthorized,
            _ => NanoServiceErrorStatus::Unknown,
        }
    }
}


/// The custom error that Actix web automatically converts to a HTTP response.
/// 
/// # Fields
/// * `message` - The message of the error.
/// * `status` - The status of the error.
#[derive(Serialize, Deserialize, Debug, Error)]
pub struct NanoServiceError {
    pub message: String,
    pub status: NanoServiceErrorStatus
}

impl NanoServiceError {

    /// Constructs a new error.
    /// 
    /// # Arguments
    /// * `message` - The message of the error.
    /// * `status` - The status of the error.
    /// 
    /// # Returns
    /// * `CustomError` - The new error.
    pub fn new(message: String, status: NanoServiceErrorStatus) -> NanoServiceError {
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


#[cfg(feature = "actix")]
impl ResponseError for NanoServiceError {
    
    /// Yields the status code for the error.
    /// 
    /// # Returns
    /// * `StatusCode` - The status code for the error.
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

    /// Constructs a HTTP response for the error.
    /// 
    /// # Returns
    /// * `HttpResponse` - The HTTP response for the error.
    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        HttpResponse::build(status_code).json(self.message.clone())
    }
}


#[macro_export]
macro_rules! safe_eject {
    ($e:expr, $err_status:expr) => {
        $e.map_err(|x| NanoServiceError::new(x.to_string(), $err_status))
    };
    ($e:expr, $err_status:expr, $message_context:expr) => {
        $e.map_err(|x| NanoServiceError::new(
                format!("{}: {}", $message_context, x.to_string()),
                $err_status
            )
        )
    };
}
