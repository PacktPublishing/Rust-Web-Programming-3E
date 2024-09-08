use axum::{
    response::IntoResponse,
    http::StatusCode,
    Json,
};
use to_do_core::api::basic_actions::get::get_all as get_all_core;


/// Gets all tasks.
///
/// # Returns
/// A `Json` response with a body containing all tasks or an error message.
pub async fn get_all() -> impl IntoResponse {
    match get_all_core().await {
        Ok(items) => (StatusCode::OK, Json(items)).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
