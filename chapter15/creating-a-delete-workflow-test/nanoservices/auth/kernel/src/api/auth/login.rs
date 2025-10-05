#[cfg(any(feature = "core-postgres", feature = "http"))]
mod common_imports {
    pub use glue::errors::NanoServiceError;
}
#[cfg(feature = "core-postgres")]
mod core_imports {
    pub use auth_dal::users::descriptors::SqlxPostGresDescriptor;
    pub use auth_core::api::auth::login::login as login_core;
}
#[cfg(feature = "http")]
mod reqwest_imports {
    pub use reqwest::Client;
    pub use glue::errors::NanoServiceErrorStatus;
    pub use glue::urls::get_auth_url;
    pub use base64::engine::general_purpose::STANDARD;
}
#[cfg(feature = "http")]
use base64::Engine;


#[cfg(any(feature = "core-postgres", feature = "http"))]
pub async fn login_user(email: String, password: String) 
    -> Result<String, common_imports::NanoServiceError> {
    #[cfg(feature = "core-postgres")]
    let token = core_imports::login_core::<
        core_imports::SqlxPostGresDescriptor
    >(email, password).await?;
    #[cfg(feature = "http")]
    let token = login_user_api_call(email, password).await?.into();
    return Ok(token)
}


#[cfg(feature = "http")]
async fn login_user_api_call(email: String, password: String) 
    -> Result<String, common_imports::NanoServiceError> {
    let url = reqwest_imports::get_auth_url()?;
    let full_url = format!("{}/api/v1/auth/login", url);
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
                reqwest_imports::NanoServiceErrorStatus::BadRequest
            )
        })?;
    let status_code = response.status().as_u16();
    if status_code == 200 {
        let token: String = response.json().await.map_err(|e| {
            common_imports::NanoServiceError::new(
                e.to_string(),
                reqwest_imports::NanoServiceErrorStatus::BadRequest
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
            reqwest_imports::NanoServiceErrorStatus::from_code(
                status_code
            )
        ))
    }

}
