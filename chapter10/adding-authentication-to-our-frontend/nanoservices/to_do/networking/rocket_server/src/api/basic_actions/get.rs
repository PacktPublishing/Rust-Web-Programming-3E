use rocket::serde::json::Json;
use to_do_core::api::basic_actions::get::get_all as get_all_core;
use glue::errors::NanoServiceError;
use to_do_dal::to_do_items::descriptors::SqlxPostGresDescriptor;
use to_do_dal::to_do_items::schema::AllToDOItems;
use glue::token::HeaderToken;


/// Gets all tasks.
///
/// # Returns
/// A `Json` response with a body containing all tasks or an error message.
#[get("/get/all")]
pub async fn get_all(token: HeaderToken) -> Result<Json<AllToDOItems>, NanoServiceError> {
    let items = get_all_core::<SqlxPostGresDescriptor>().await?;
    Ok(Json(items))
}
