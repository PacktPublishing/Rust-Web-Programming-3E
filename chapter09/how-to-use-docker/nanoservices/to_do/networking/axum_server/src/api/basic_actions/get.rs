use axum::{
    response::IntoResponse,
    extract::Path,
    http::StatusCode,
    Json,
};
use to_do_core::api::basic_actions::get::{
    get_all as get_all_core,
    get_by_name as get_by_name_core
};
use glue::errors::NanoServiceError;


/// Gets all tasks.
///
/// # Returns
/// A `Json` response with a body containing all tasks or an error message.
pub async fn get_all() -> Result<impl IntoResponse, NanoServiceError> {
    let items = get_all_core().await?;
    Ok((StatusCode::OK, Json(items)).into_response())
}


/// Gets a task by name.
/// 
/// # Arguments
/// * `name` - The name of the task to get.
/// 
/// # Returns
/// A `Json` response with a body containing the task specified in the URL or an error message.
pub async fn get_by_name(Path(name): Path<String>) -> Result<impl IntoResponse, NanoServiceError> {
    let item = get_by_name_core(&name).await?;
    Ok((StatusCode::OK, Json(item)).into_response())
}
