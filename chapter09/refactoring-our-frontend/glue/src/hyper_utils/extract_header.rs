use hyper::Request;
use crate::errors::{NanoServiceError, NanoServiceErrorStatus};
use crate::token::HeaderToken;
use hyper::Response;
use http_body_util::Full;
use hyper::body::Bytes;


/// Extracts the token from the header of the request.
/// 
/// # Arguments
/// * `req` - The request containing the token header
/// 
/// # Returns
/// the token from the header
pub async fn extract_token(req: &Request<hyper::body::Incoming>) -> Result<HeaderToken, Response<Full<Bytes>>> {
    let headers = req.headers();
    let token = match headers.get("token") {
        Some(token) => token.to_str().unwrap(),
        None => {
            return Err(NanoServiceError::new(
                "token not found in request header".to_string(), 
                NanoServiceErrorStatus::Unauthorized
            ).into_hyper_response())
        }
    };
    let token = HeaderToken {
        message: token.to_string()
    };
    Ok(token)
}


#[macro_export]
macro_rules! extract_hyper_header_token {
    ($req:expr) => {
        match extract_token($req).await {
            Ok(token) => token,
            Err(error) => return Ok(error)
        }
    };
}
