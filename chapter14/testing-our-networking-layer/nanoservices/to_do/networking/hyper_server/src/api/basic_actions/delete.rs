use glue::errors::{NanoServiceError, NanoServiceErrorStatus};
use glue::safe_eject;
use hyper::body::Bytes;
use hyper::{header, Response};
use to_do_core::api::basic_actions::{
    delete::delete as delete_core,
    get::get_all as get_all_core
};
use http_body_util::Full;
use to_do_dal::to_do_items::transactions::{
    delete::DeleteOne,
    get::GetAll
};
use glue::token::HeaderToken;
use auth_kernel::api::users::get::get_user_by_unique_id;


/// Deletes a task by name.
/// 
/// # Arguments
/// * `name` - The name of the task to delete.
/// 
/// # Returns
/// A `Response` with a body containing all the to-do items.
pub async fn delete_by_name<T: DeleteOne + GetAll>(name: &str, token: HeaderToken) -> Result<Response<Full<Bytes>>, NanoServiceError> {
    let user = get_user_by_unique_id(
        token.unique_id
    ).await?;
    let _ = delete_core::<T>(name, user.id).await?;
    let json_body = safe_eject!(
        serde_json::to_string(&get_all_core::<T>(user.id).await?),
        NanoServiceErrorStatus::Unknown
    )?;
    safe_eject!(
        Response::builder()
                .header(header::CONTENT_TYPE, "application/json")
                .body(Full::new(Bytes::from(json_body))),
        NanoServiceErrorStatus::Unknown
    )
}