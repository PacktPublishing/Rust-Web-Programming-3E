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


/// Deletes an item from the to-do list by name.
/// 
/// # Arguments
/// - `req` - The HTTP request.
/// 
/// # Returns
/// All of the items in the to-do list.
pub async fn delete_by_name<T: DeleteOne + GetAll>(token: HeaderToken, Path(name): Path<String>) -> Result<impl IntoResponse, NanoServiceError> {
    let _ = delete_core::<T>(&name).await?;
    Ok((StatusCode::OK, Json(get_all_core::<T>().await?)).into_response())
}
