#[cfg(feature = "http")]
mod common_imports {
    pub use glue::errors::NanoServiceError;
    pub use dal::to_do_items::schema::AllToDOItems;
    pub use glue::errors::NanoServiceErrorStatus;
}
#[cfg(feature = "http")]
mod reqwest_imports {
    pub use reqwest::Client;
    pub use glue::token::HeaderToken;
    pub use glue::urls::get_to_do_url;
}


#[cfg(any(feature = "http"))]
pub async fn delete_item(token: reqwest_imports::HeaderToken, name: String) 
    -> Result<common_imports::AllToDOItems, common_imports::NanoServiceError> {

    #[cfg(feature = "http")]
    let user: common_imports::AllToDOItems = delete_item_api_call(token, name).await?.into();

    return Ok(user)
}


#[cfg(feature = "http")]
async fn delete_item_api_call(token: reqwest_imports::HeaderToken, name: String) 
    -> Result<common_imports::AllToDOItems, common_imports::NanoServiceError> {

    let full_url = format!(
        "{}/api/v1/delete/{}", 
        reqwest_imports::get_to_do_url()?,
        name
    );

    let client = reqwest_imports::Client::new();
    let response = client
        .delete(&full_url)
        .header("token", token.encode()?)
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
        let trimmed_user = response
            .json::<common_imports::AllToDOItems>()
            .await
            .map_err(|e| common_imports::NanoServiceError::new(
                e.to_string(),
                common_imports::NanoServiceErrorStatus::from_code(
                    status_code
                )
            ))?;
        return Ok(trimmed_user)
    } else {
        return Err(common_imports::NanoServiceError::new(
            "Failed to delete item".to_string(),
            common_imports::NanoServiceErrorStatus::from_code(
                status_code
            )
        ))
    }
}
