use serde::de::DeserializeOwned;
use hyper::{Request, body::Incoming};
use crate::errors::{NanoServiceError, NanoServiceErrorStatus};
use hyper::body::Buf;
use http_body_util::BodyExt;
use crate::safe_eject;


/// Extracts the body from a request and deserializes it into a struct.
/// 
/// # Arguments
/// * `req` - The request containing the JSON body.
/// 
/// # Returns
/// The deserialized struct.
pub async fn extract_body<X: DeserializeOwned>(req: Request<Incoming>) -> Result<X, NanoServiceError> {
    let whole_body = safe_eject!(
        req.collect().await,
        NanoServiceErrorStatus::BadRequest
    )?.aggregate();

    let body: X = safe_eject!(
        serde_json::from_reader(whole_body.reader()),
        NanoServiceErrorStatus::BadRequest,
        "Failed to parse JSON body"
    )?;
    Ok(body)
}
