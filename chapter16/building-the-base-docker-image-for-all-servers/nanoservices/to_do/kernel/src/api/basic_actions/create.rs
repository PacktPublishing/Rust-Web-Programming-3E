#[cfg(feature = "http")]
mod common_imports {
    pub use glue::errors::NanoServiceError;
    pub use dal::to_do_items::schema::NewToDoItem;
    pub use dal::to_do_items::enums::TaskStatus;
    pub use dal::to_do_items::schema::AllToDOItems;
    pub use glue::errors::NanoServiceErrorStatus;
    pub use glue::token::HeaderToken;
}
#[cfg(feature = "http")]
mod reqwest_imports {
    pub use reqwest::Client;
    pub use glue::urls::get_to_do_url;
}


#[cfg(any(feature = "http"))]
pub async fn create_item(token: common_imports::HeaderToken, title: String, status: common_imports::TaskStatus) 
    -> Result<common_imports::AllToDOItems, common_imports::NanoServiceError> {

    #[cfg(feature = "http")]
    let user = create_item_api_call(token, title, status).await?.into();

    return Ok(user)
}

#[cfg(feature = "http")]
async fn create_item_api_call(token: common_imports::HeaderToken, title: String, status: common_imports::TaskStatus) 
    -> Result<common_imports::AllToDOItems, common_imports::NanoServiceError> {

    let full_url = format!(
        "{}/api/v1/create", 
        reqwest_imports::get_to_do_url()?
    );
    let body = common_imports::NewToDoItem {
        title,
        status
    };

    let client = reqwest_imports::Client::new();
    let response = client
        .post(&full_url)
        .header("token", token.encode()?)
        .json(&body)
        .send()
        .await
        .map_err(|e| {
            common_imports::NanoServiceError::new(
                e.to_string(),
                common_imports::NanoServiceErrorStatus::BadRequest
            )
        })?;
        
    let status_code = response.status().as_u16();

    if status_code == 201 {
        let items = response
            .json::<common_imports::AllToDOItems>()
            .await
            .map_err(|e: reqwest::Error| common_imports::NanoServiceError::new(
                e.to_string(),
                common_imports::NanoServiceErrorStatus::from_code(
                    status_code
                )
            ))?;
        return Ok(items)
    } else {
        let error_message = response.text().await.unwrap_or_else(|_| 
            "No error message".to_string()
        );
        return Err(common_imports::NanoServiceError::new(
            error_message,
            common_imports::NanoServiceErrorStatus::from_code(
                status_code
            )
        ))
    }
}