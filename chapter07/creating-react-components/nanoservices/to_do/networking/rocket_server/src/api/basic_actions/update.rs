use to_do_core::api::basic_actions::{
    update::update as update_core,
    get::get_all as get_all_core
};
use to_do_core::structs::{ToDoItem, AllToDoItems};
use glue::errors::NanoServiceError;
use glue::token::HeaderToken;
use rocket::serde::json::Json;


/// Updates an item in the to-do list.
/// 
/// # Arguments
/// - `body` - The JSON body containing the item to be updated.
/// 
/// # Returns
/// All of the items in the to-do list.
#[patch("/update", data = "<body>")]
pub async fn update(token: HeaderToken, body: Json<ToDoItem>) 
    -> Result<Json<AllToDoItems>, NanoServiceError> {
    let _ = update_core(body.into_inner()).await?;
    Ok(Json(get_all_core().await?))
}
