//! Extracts the auth credentials from the request header.
use actix_web::HttpRequest;
use glue::errors::{NanoServiceError, NanoServiceErrorStatus};
use base64::{Engine, engine::general_purpose};


/// The credentials extracted from the request header.
#[derive(Debug)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}


/// Extracts the auth credentials from the request header.
/// 
/// # Parameters
/// * req: the request to extract the credentials from
/// 
/// # Returns
/// * the credentials extracted from the request header
pub async fn extract_credentials(req: HttpRequest) -> Result<Credentials, NanoServiceError> {
    // Extract Authorization header
    let header_value = match req.headers().get("Authorization") {
        Some(auth_header) => auth_header,
        None => return Err(NanoServiceError::new("No credentials provided".to_string(), NanoServiceErrorStatus::Unauthorized)),
    
    };
    let encoded = match header_value.to_str() {
        Ok(encoded) => encoded,
        Err(_) => return Err(NanoServiceError::new("Invalid credentials".to_string(), NanoServiceErrorStatus::Unauthorized)),
    };
    if !encoded.starts_with("Basic ") {
        return Err(NanoServiceError::new("Invalid credentials".to_string(), NanoServiceErrorStatus::Unauthorized))
    }
    // Decode credentials
    let base64_credentials = &encoded[6..];
    let decoded = general_purpose::STANDARD.decode(base64_credentials).map_err(|e|{NanoServiceError::new(e.to_string(), NanoServiceErrorStatus::Unauthorized)})?;
    let credentials = String::from_utf8(decoded).map_err(|e|{NanoServiceError::new(e.to_string(), NanoServiceErrorStatus::Unauthorized)})?;
    let parts: Vec<&str> = credentials.splitn(2, ':').collect();

    if parts.len() == 2 {
        let email = parts[0];
        let password = parts[1];

        return Ok(Credentials {
            email: email.to_string(),
            password: password.to_string(),
        });
    }
    else {
        return Err(NanoServiceError::new("Invalid credentials".to_string(), NanoServiceErrorStatus::Unauthorized))
    }
}
