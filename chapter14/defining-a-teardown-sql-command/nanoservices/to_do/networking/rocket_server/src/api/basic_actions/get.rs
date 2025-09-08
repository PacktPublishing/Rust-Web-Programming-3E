use rocket::serde::json::Json;
use to_do_core::api::basic_actions::get::get_all as get_all_core;
use glue::errors::NanoServiceError;
use to_do_dal::to_do_items::descriptors::SqlxPostGresDescriptor;
use to_do_dal::to_do_items::schema::AllToDOItems;
use glue::token::HeaderToken;
use auth_kernel::api::users::get::get_user_by_unique_id;


/// Gets all tasks.
///
/// # Returns
/// A `Json` response with a body containing all tasks or an error message.
#[get("/get/all")]
pub async fn get_all(token: HeaderToken) -> Result<Json<AllToDOItems>, NanoServiceError> {
    let user = get_user_by_unique_id(
        token.unique_id
    ).await?;
    let items = get_all_core::<SqlxPostGresDescriptor>(user.id).await?;
    Ok(Json(items))
}
