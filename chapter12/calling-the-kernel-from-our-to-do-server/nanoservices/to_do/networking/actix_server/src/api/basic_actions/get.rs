use to_do_core::api::basic_actions::get::get_all as get_all_core;
use actix_web::HttpResponse;
use glue::errors::NanoServiceError;
use to_do_dal::to_do_items::transactions::get::GetAll;
use glue::token::HeaderToken;
use auth_kernel::user_session::transactions::get::GetUserSession;


/// Gets all tasks.
/// 
/// # Returns
/// An `HttpResponse` with a JSON body containing all tasks.
pub async fn get_all<T, X>(token: HeaderToken) 
-> Result<HttpResponse, NanoServiceError> 
where
    T: GetAll,
    X: GetUserSession
{
    let session = X::get_user_session(token.unique_id).await?;
    Ok(HttpResponse::Ok().json(
        get_all_core::<T>(session.user_id).await?)
    )
}
