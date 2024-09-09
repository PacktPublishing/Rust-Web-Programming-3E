use axum::{
    response::IntoResponse,
    http::StatusCode,
    Json,
};
use to_do_core::api::basic_actions::get::get_all as get_all_core;
use glue::errors::NanoServiceError;


/// Gets all tasks.
///
/// # Returns
/// A `Json` response with a body containing all tasks or an error message.
pub async fn get_all() -> Result<impl IntoResponse, NanoServiceError> {
    let items = get_all_core().await?;
    Ok((StatusCode::OK, Json(items)).into_response())
}
