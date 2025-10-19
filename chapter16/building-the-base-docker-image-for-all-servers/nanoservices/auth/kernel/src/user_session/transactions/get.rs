use std::future::Future;
use crate::user_session::schema::UserSession;
use glue::errors::{NanoServiceError, NanoServiceErrorStatus};
use cache_client::{update, UserSessionStatus, login};
use crate::api::users::get::get_user_by_unique_id;
use crate::user_session::descriptors::RedisSessionDescriptor;


pub trait GetUserSession {
    fn get_user_session(unique_id: String) -> impl Future<Output = Result<UserSession, NanoServiceError>>;
}


impl GetUserSession for RedisSessionDescriptor {
    fn get_user_session(unique_id: String) -> impl Future<Output = Result<UserSession, NanoServiceError>> {
        get_session_redis(unique_id)
    }
}


pub async fn get_session_redis(unique_id: String) -> Result<UserSession, NanoServiceError> {
    let address = std::env::var("CACHE_API_URL").map_err(|e|{
        NanoServiceError::new(e.to_string(), NanoServiceErrorStatus::BadRequest)
    })?;
    let user_id = update(&address, &unique_id).await?;
    match user_id {
        UserSessionStatus::Ok(id) => Ok(UserSession { user_id: id }),
        UserSessionStatus::Refresh => {
            let user = get_user_by_unique_id(unique_id.clone()).await?;
            let _ = login(&address, &unique_id, 20, user.id).await?;
            match user_id {
                UserSessionStatus::Ok(id) => Ok(UserSession { user_id: id }),
                _ => Err(NanoServiceError::new(
                    "Failed to update user session".to_string(), 
                    NanoServiceErrorStatus::Unknown)
                )
            }
        }
    }
}
