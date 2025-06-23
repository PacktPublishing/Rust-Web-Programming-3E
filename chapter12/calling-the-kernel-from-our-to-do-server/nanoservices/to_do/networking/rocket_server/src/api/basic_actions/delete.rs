use rocket::serde::json::Json;
use to_do_core::api::basic_actions::{
    delete::delete as delete_core,
    get::get_all as get_all_core
};
use to_do_dal::to_do_items::schema::AllToDOItems;
use glue::errors::NanoServiceError;
use to_do_dal::to_do_items::descriptors::SqlxPostGresDescriptor;
use glue::token::HeaderToken;
use auth_kernel::user_session::descriptors::RedisSessionDescriptor;
use auth_kernel::user_session::transactions::get::GetUserSession;


/// Deletes a task by name.
/// 
/// # Arguments
/// * `name` - The name of the task to delete.
/// 
/// # Returns
/// A `Json` response with all the to-do items.
#[delete("/delete/<name>")]
pub async fn delete_by_name(token: HeaderToken, name: &str) -> Result<Json<AllToDOItems>, NanoServiceError> {
    let session = RedisSessionDescriptor::get_user_session(token.unique_id).await?;
    let _ = delete_core::<SqlxPostGresDescriptor>(name, session.user_id).await?;
    Ok(Json(get_all_core::<SqlxPostGresDescriptor>(session.user_id).await?))
}
