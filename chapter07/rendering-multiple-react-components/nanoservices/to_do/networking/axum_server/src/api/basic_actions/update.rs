use to_do_core::api::basic_actions::{
    update::update as update_core,
    get::get_all as get_all_core
};
use to_do_core::structs::ToDoItem;
use glue::errors::NanoServiceError;
use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
};
use glue::token::HeaderToken;


/// updates an item in the to-do list.
/// 
/// # Arguments
/// - `body` - The JSON body containing the item to be updated.
/// 
/// # Returns
/// All of the items in the to-do list.
// pub async fn update(Json(body): Json<ToDoItem>) -> Result<impl IntoResponse, NanoServiceError> {
pub async fn update(token: HeaderToken, Json(body): Json<ToDoItem>) -> Result<impl IntoResponse, NanoServiceError> {
    // Call the core create function
    update_core(body).await?;

    // Fetch all items after creation
    let all_items = get_all_core().await?;

    // Return the response with a status code and JSON body
    Ok((StatusCode::OK, Json(all_items)))
}
