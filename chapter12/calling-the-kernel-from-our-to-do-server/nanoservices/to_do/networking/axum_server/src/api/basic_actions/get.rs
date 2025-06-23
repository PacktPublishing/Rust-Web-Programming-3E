use axum::{
    response::IntoResponse,
    http::StatusCode,
    Json,
};
use to_do_core::api::basic_actions::get::get_all as get_all_core;
use glue::errors::NanoServiceError;
use to_do_dal::to_do_items::transactions::get::GetAll;
use glue::token::HeaderToken;
use auth_kernel::user_session::{self, transactions::get::GetUserSession};


/// Gets all tasks.
///
/// # Returns
/// A `Json` response with a body containing all tasks or an error message.
pub async fn get_all<T: GetAll, X: GetUserSession>(token: HeaderToken) -> Result<impl IntoResponse, NanoServiceError> {
    let session = X::get_user_session(token.unique_id).await?;
    let items = get_all_core::<T>(session.user_id).await?;
    Ok((StatusCode::OK, Json(items)).into_response())
}
