use auth_dal::users::transactions::{
    get::GetByUniqueId,
    refresh::RefreshUser
};
use glue::errors::NanoServiceError;


pub async fn refresh_user<T>(id: String) -> Result<String, NanoServiceError> 
where T: GetByUniqueId + RefreshUser
{
    let user = T::get_by_unique_id(id).await?;
    Ok(T::refresh_user(user).await?)
}
