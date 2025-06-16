use actix_web::HttpRequest;
use glue::errors::{NanoServiceError, NanoServiceErrorStatus};
use base64::{Engine, engine::general_purpose};


#[derive(Debug)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}


pub async fn extract_credentials(req: HttpRequest) 
    -> Result<Credentials, NanoServiceError> {
    let header_value = match req.headers().get("Authorization") {
        Some(auth_header) => auth_header,
        None => return Err(
            NanoServiceError::new(
                "No credentials provided".to_string(), 
                NanoServiceErrorStatus::Unauthorized
            )
        ),
    
    };
    let encoded = match header_value.to_str() {
        Ok(encoded) => encoded,
        Err(_) => return Err(
            NanoServiceError::new(
                "Invalid credentials".to_string(), 
                NanoServiceErrorStatus::Unauthorized
            )
        ),
    };

    if !encoded.starts_with("Basic ") {
        return Err(
            NanoServiceError::new(
                "Invalid credentials".to_string(), 
                NanoServiceErrorStatus::Unauthorized
            )
        )
    }
    
    let base64_credentials = &encoded[6..];
    let decoded = general_purpose::STANDARD.decode(
        base64_credentials
    ).map_err(|e|{
        NanoServiceError::new(e.to_string(), 
        NanoServiceErrorStatus::Unauthorized)
    })?;
    let credentials = String::from_utf8(
        decoded
    ).map_err(|e|{
        NanoServiceError::new(e.to_string(), 
        NanoServiceErrorStatus::Unauthorized)}
    )?;

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
        return Err(
            NanoServiceError::new("Invalid credentials".to_string(), 
            NanoServiceErrorStatus::Unauthorized)
        )
    }

}
