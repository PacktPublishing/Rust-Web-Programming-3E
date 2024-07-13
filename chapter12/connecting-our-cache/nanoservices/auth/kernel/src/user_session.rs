use cache_client::{update, UserSessionStatus, login};
use glue::errors::{NanoServiceError, NanoServiceErrorStatus};
use crate::api::users::get::get_user_by_unique_id;


pub struct UserSession {
    pub user_id: i32
}


impl UserSession {
    pub async fn new(unique_id: String) -> Result<Self, NanoServiceError> {
        let address = std::env::var("CACHE_API_URL").map_err(|e|{
            NanoServiceError::new(e.to_string(), NanoServiceErrorStatus::BadRequest)
        })?;
        let user_id = update(&address, &unique_id).await?;
        match user_id {
            UserSessionStatus::Ok(id) => Ok(Self { user_id: id }),
            UserSessionStatus::Refresh => {
                let user = get_user_by_unique_id(unique_id.clone()).await?;
                let _ = login(&address, &unique_id, 20, user.id).await?;
                match user_id {
                    UserSessionStatus::Ok(id) => Ok(Self { user_id: id }),
                    _ => Err(NanoServiceError::new(
                        "Failed to update user session".to_string(), 
                        NanoServiceErrorStatus::Unknown)
                    )
                }
            }
        }
    }
}