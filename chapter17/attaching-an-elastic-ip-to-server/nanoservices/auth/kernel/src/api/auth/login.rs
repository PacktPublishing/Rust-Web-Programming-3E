#[cfg(any(feature = "core-postgres", feature = "http"))]
mod common_imports {
    pub use glue::errors::NanoServiceError;
    pub use glue::errors::NanoServiceErrorStatus;
}
#[cfg(feature = "core-postgres")]
mod core_imports {
    pub use auth_dal::users::descriptors::SqlxPostGresDescriptor;
    pub use auth_core::api::auth::login::login as login_core;
}
#[cfg(feature = "http")]
mod reqwest_imports {
    pub use reqwest::Client;
    pub use glue::urls::get_auth_url;
    pub use base64::engine::general_purpose::STANDARD;
}

#[cfg(feature = "http")]
use base64::Engine;


#[cfg(feature = "http")]
#[cfg(not(feature = "core-postgres"))]
pub async fn login_user(email: String, password: String) 
    -> Result<String, common_imports::NanoServiceError> {
    login_user_api_call(email, password).await
}

#[cfg(feature = "core-postgres")]
#[cfg(not(feature = "http"))]
pub async fn login_user(email: String, password: String) 
    -> Result<String, common_imports::NanoServiceError> {
    core_imports::login_core::<core_imports::SqlxPostGresDescriptor>(email, password).await
}


#[cfg(feature = "http")]
async fn login_user_api_call(email: String, password: String) 
    -> Result<String, common_imports::NanoServiceError> {
    let url = reqwest_imports::get_auth_url()?;
    let full_url = format!("{}/api/v1/auth/login", url);

    // Create the basic auth header value
    let auth = format!("{}:{}", email, password);
    let encoded_auth = reqwest_imports::STANDARD.encode(auth);
    let auth_header_value = format!("Basic {}", encoded_auth);

    let client = reqwest_imports::Client::new();
    let response = client
        .get(&full_url)
        .header("Authorization", auth_header_value)
        .send()
        .await
        .map_err(|e| {
            common_imports::NanoServiceError::new(
                e.to_string(),
                common_imports::NanoServiceErrorStatus::BadRequest
            )
        })?;

    let status_code = response.status().as_u16();

    if status_code == 200 {
        let token: String = response.json().await.map_err(|e| {
            common_imports::NanoServiceError::new(
                e.to_string(),
                common_imports::NanoServiceErrorStatus::BadRequest
            )
        })?;
        return Ok(token)
    }
    else {
        let response_message = match response.json().await {
            Ok(json) => json,
            Err(_) => "Unknown error".to_string()
        };
    
        return Err(common_imports::NanoServiceError::new(
            response_message, 
            common_imports::NanoServiceErrorStatus::from_code(
                status_code
            )
        ))
    }
}