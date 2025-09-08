use axum::{
    response::IntoResponse,
    http::StatusCode,
    Json,
};
use to_do_core::api::basic_actions::get::get_all as get_all_core;
use glue::errors::NanoServiceError;
use to_do_dal::to_do_items::transactions::get::GetAll;
use glue::token::HeaderToken;
use auth_kernel::api::users::get::get_user_by_unique_id;
use auth_kernel::user_session::transactions::get::GetUserSession;


/// Gets all tasks.
///
/// # Returns
/// A `Json` response with a body containing all tasks or an error message.
pub async fn get_all<T, X>(token: HeaderToken) -> Result<impl IntoResponse, NanoServiceError>
where
    T: GetAll,
    X: GetUserSession
{
    let user = get_user_by_unique_id(token.unique_id).await?;
    let items = get_all_core::<T>(user.id).await?;
    Ok((StatusCode::OK, Json(items)).into_response())
}
