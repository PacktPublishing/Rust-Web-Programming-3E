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


/// Deletes an item from the to-do list by name.
/// 
/// # Arguments
/// - `req` - The HTTP request.
/// 
/// # Returns
/// All of the items in the to-do list.
pub async fn delete_by_name(Path(name): Path<String>) -> Result<impl IntoResponse, NanoServiceError> {
    let _ = delete_core(&name).await?;
    Ok((StatusCode::OK, Json(get_all_core().await?)).into_response())
}
