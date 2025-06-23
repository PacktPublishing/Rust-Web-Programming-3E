use to_do_core::api::basic_actions::{
    create::create as create_core,
    get::get_all as get_all_core
};
use glue::errors::NanoServiceError;
use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
};
use to_do_dal::to_do_items::schema::NewToDoItem;
use to_do_dal::to_do_items::transactions::{
    create::SaveOne,
    get::GetAll
};
use glue::token::HeaderToken;
use auth_kernel::api::users::get::get_user_by_unique_id;


/// Creates an item in the to-do list.
/// 
/// # Arguments
/// - `body` - The JSON body containing the item to be created.
/// 
/// # Returns
/// All of the items in the to-do list.
pub async fn create<T: SaveOne + GetAll>(token: HeaderToken, Json(body): Json<NewToDoItem>) -> Result<impl IntoResponse, NanoServiceError> {
    let user = get_user_by_unique_id(token.unique_id).await?;
    let _ = create_core::<T>(body.into(), user.id).await?;
    Ok((StatusCode::CREATED, Json(get_all_core::<T>(user.id).await?)))
}
