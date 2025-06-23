use to_do_core::api::basic_actions::{
    delete::delete as delete_core,
    get::get_all as get_all_core
};
use axum::{
    response::IntoResponse,
    extract::Path,
    http::StatusCode,
    Json,
};
use glue::errors::NanoServiceError;
use to_do_dal::to_do_items::transactions::{
    delete::DeleteOne,
    get::GetAll
};
use glue::token::HeaderToken;
use auth_kernel::user_session::transactions::get::GetUserSession;


/// Deletes an item from the to-do list by name.
/// 
/// # Arguments
/// - `req` - The HTTP request.
/// 
/// # Returns
/// All of the items in the to-do list.
pub async fn delete_by_name<T: DeleteOne + GetAll, X: GetUserSession>(token: HeaderToken, Path(name): Path<String>) -> Result<impl IntoResponse, NanoServiceError> {
    let session = X::get_user_session(token.unique_id).await?;
    let _ = delete_core::<T>(&name, session.user_id).await?;
    Ok((StatusCode::OK, Json(get_all_core::<T>(session.user_id).await?)).into_response())
}
