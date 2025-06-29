use to_do_core::api::basic_actions::{
    update::update as update_core,
    get::get_all as get_all_core
};
use to_do_dal::to_do_items::schema::ToDoItem;
use glue::errors::NanoServiceError;
use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
};
use glue::token::HeaderToken;
use to_do_dal::to_do_items::transactions::{
    update::UpdateOne,
    get::GetAll
};
use auth_kernel::api::users::get::get_user_by_unique_id;


/// updates an item in the to-do list.
/// 
/// # Arguments
/// - `body` - The JSON body containing the item to be updated.
/// 
/// # Returns
/// All of the items in the to-do list.
// pub async fn update(Json(body): Json<ToDoItem>) -> Result<impl IntoResponse, NanoServiceError> {
pub async fn update<T: UpdateOne + GetAll>(token: HeaderToken, Json(body): Json<ToDoItem>) -> Result<impl IntoResponse, NanoServiceError> {
    let user = get_user_by_unique_id(
        token.unique_id
    ).await?;
    // Call the core create function
    let _ = update_core::<T>(body, user.id).await?;

    // Fetch all items after creation
    let all_items = get_all_core::<T>(user.id).await?;

    // Return the response with a status code and JSON body
    Ok((StatusCode::OK, Json(all_items)))
}
