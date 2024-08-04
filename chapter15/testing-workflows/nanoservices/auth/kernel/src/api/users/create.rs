#[cfg(any(feature = "core-postgres", feature = "http"))]
mod common_imports {
    pub use glue::errors::NanoServiceError;
    pub use auth_core::api::users::create::CreateUser;
}
#[cfg(feature = "core-postgres")]
mod core_imports {
    pub use auth_dal::users::descriptors::SqlxPostGresDescriptor;
    pub use auth_core::api::users::create::create as create_user_core;
}
#[cfg(feature = "http")]
mod reqwest_imports {
    pub use reqwest::Client;
    pub use glue::errors::NanoServiceErrorStatus;
    pub use glue::urls::get_auth_url;
}


#[cfg(any(feature = "core-postgres", feature = "http"))]
pub async fn create_user(user: common_imports::CreateUser) 
    -> Result<(), common_imports::NanoServiceError> {

    #[cfg(feature = "core-postgres")]
    core_imports::create_user_core::<
        core_imports::SqlxPostGresDescriptor
    >(user).await?;

    #[cfg(feature = "http")]
    create_user_api_call(user).await?;

    return Ok(())
}


#[cfg(feature = "http")]
async fn create_user_api_call(user: common_imports::CreateUser) 
    -> Result<(), common_imports::NanoServiceError> {
    let url = reqwest_imports::get_auth_url()?;
    let full_url = format!("{}/api/v1/users/create", url);

    let client = reqwest_imports::Client::new();
    let _ = client
        .post(&full_url)
        .json(&user)
        .send()
        .await
        .map_err(|e| {
            common_imports::NanoServiceError::new(
                e.to_string(),
                reqwest_imports::NanoServiceErrorStatus::BadRequest
            )
        })?;

    return Ok(())
}