use to_do_core::api::basic_actions::{
    update::update as update_core,
    get::get_all as get_all_core
};
use glue::errors::NanoServiceError;
use glue::token::HeaderToken;
use rocket::serde::json::Json;
use to_do_dal::to_do_items::descriptors::SqlxPostGresDescriptor;
use to_do_dal::to_do_items::schema::{AllToDOItems, ToDoItem};


/// Updates an item in the to-do list.
/// 
/// # Arguments
/// - `body` - The JSON body containing the item to be updated.
/// 
/// # Returns
/// All of the items in the to-do list.
#[patch("/update", data = "<body>")]
pub async fn update(token: HeaderToken, body: Json<ToDoItem>) 
    -> Result<Json<AllToDOItems>, NanoServiceError> {
    let _ = update_core::<SqlxPostGresDescriptor>(body.into_inner()).await?;
    Ok(Json(get_all_core::<SqlxPostGresDescriptor>().await?))
}
