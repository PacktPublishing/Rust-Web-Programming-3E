use rocket::serde::json::Json;
use to_do_core::api::basic_actions::{
    delete::delete as delete_core,
    get::get_all as get_all_core
};
use to_do_dal::to_do_items::schema::AllToDOItems;
use glue::errors::NanoServiceError;
use to_do_dal::to_do_items::descriptors::SqlxPostGresDescriptor;
use glue::token::HeaderToken;
use auth_kernel::api::users::get::get_user_by_unique_id;


/// Deletes a task by name.
/// 
/// # Arguments
/// * `name` - The name of the task to delete.
/// 
/// # Returns
/// A `Json` response with all the to-do items.
#[delete("/delete/<name>")]
pub async fn delete_by_name(token: HeaderToken, name: &str) -> Result<Json<AllToDOItems>, NanoServiceError> {
    let user = get_user_by_unique_id(
        token.unique_id
    ).await?;
    let _ = delete_core::<SqlxPostGresDescriptor>(name, user.id).await?;
    Ok(Json(get_all_core::<SqlxPostGresDescriptor>(user.id).await?))
}
