#[cfg(any(feature = "core-postgres", feature = "http"))]
mod common_imports {
    pub use auth_dal::users::schema::TrimmedUser;
    pub use glue::errors::NanoServiceError;
}
#[cfg(feature = "core-postgres")]
mod core_imports {
    pub use auth_core::api::users::get::get_by_unique_id as get_by_unique_id_core;
    pub use auth_dal::users::descriptors::SqlxPostGresDescriptor;
}
#[cfg(feature = "http")]
mod reqwest_imports {
    pub use reqwest::Client;
    pub use glue::errors::NanoServiceErrorStatus;
    pub use glue::token::HeaderToken;
}
#[cfg(any(feature = "core-postgres", feature = "http"))]
use common_imports::*;
#[cfg(feature = "core-postgres")]
use core_imports::*;
#[cfg(feature = "http")]
use reqwest_imports::*;


#[cfg(any(feature = "core-postgres", feature = "http"))]
pub async fn get_user_by_unique_id(id: String) -> Result<TrimmedUser, NanoServiceError> {

    #[cfg(feature = "core-postgres")]
    let user: TrimmedUser = get_by_unique_id_core::<SqlxPostGresDescriptor>(id).await?.into();

    #[cfg(feature = "http")]
    let user: TrimmedUser = get_user_by_unique_id_api_call(id).await?.into();

    return Ok(user)
}


#[cfg(feature = "http")]
async fn get_user_by_unique_id_api_call(id: String) -> Result<TrimmedUser, NanoServiceError> {
    let url = std::env::var("AUTH_API_URL").map_err(|e|{
        NanoServiceError::new(e.to_string(), NanoServiceErrorStatus::BadRequest)
    })?;

    let full_url = format!("{}/api/v1/users/get", url);

    let header_token = HeaderToken {
        unique_id: id
    }.encode()?;

    let client = Client::new();
    let response = client
        .get(&full_url)
        .header("token", header_token)
        .send()
        .await
        .map_err(|e| {
            NanoServiceError::new(
                e.to_string(),
                NanoServiceErrorStatus::BadRequest
            )
        })?;

    if response.status().is_success() {
        let trimmed_user = response
            .json::<TrimmedUser>()
            .await
            .map_err(|e| NanoServiceError::new(
                e.to_string(),
                NanoServiceErrorStatus::BadRequest
            ))?;
        return Ok(trimmed_user)
    } else {
        return Err(NanoServiceError::new(
            format!("Failed to get user: {}", response.status()),
            NanoServiceErrorStatus::BadRequest,
        ))
    }
}
