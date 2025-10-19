use auth_dal::users::transactions::{
    get::GetByUniqueId,
    refresh::RefreshUser
};
use glue::{
    errors::NanoServiceError,
    token::HeaderToken
};
use actix_web::HttpResponse;
use auth_core::api::auth::refresh::refresh_user as refresh_user_core;


pub async fn refresh_user<T>(token: HeaderToken) 
    -> Result<HttpResponse, NanoServiceError> 
where T: GetByUniqueId + RefreshUser
{
    let id = refresh_user_core::<T>(token.unique_id).await?;
    let mut response = HttpResponse::Ok();
    response.insert_header(("rtoken", id));
    Ok(response.finish())
}
