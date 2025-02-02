use rocket::serde::json::Json;
use to_do_core::api::basic_actions::get::{
    get_all as get_all_core,
    get_by_name as get_by_name_core
};
use to_do_core::structs::{AllToDoItems, ToDoItem};
use glue::errors::NanoServiceError;


/// Gets all tasks.
///
/// # Returns
/// A `Json` response with a body containing all tasks or an error message.
#[get("/get/all")]
pub async fn get_all() -> Result<Json<AllToDoItems>, NanoServiceError> {
    let items = get_all_core().await?;
    Ok(Json(items))
}


/// Gets a task by name.
/// 
/// # Arguments
/// * `name` - The name of the task to get.
/// 
/// # Returns
/// A `Json` response with a body containing the task specified in the URL or an error message.
#[get("/get/<name>")]
pub async fn get_by_name(name: &str) -> Result<Json<ToDoItem>, NanoServiceError> {
    Ok(Json(get_by_name_core(name).await?))
}
