use rocket::serde::json::Json;
use rocket::http::Status;
use to_do_core::api::basic_actions::get::get_all as get_all_core;
use to_do_core::structs::AllToDoItems;


/// Gets all tasks.
///
/// # Returns
/// A `Json` response with a body containing all tasks or an error message.
#[get("/get/all")]
pub async fn get_all() -> Result<Json<AllToDoItems>, Status> {
    match get_all_core().await {
        Ok(items) => Ok(Json(items)),
        Err(_) => Err(Status::InternalServerError),
    }
}
