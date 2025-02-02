use glue::errors::{NanoServiceError, NanoServiceErrorStatus};
use glue::safe_eject;
use hyper::body::Bytes;
use hyper::{header, Response};
use to_do_core::api::basic_actions::{
    delete::delete as delete_core,
    get::get_all as get_all_core
};
use http_body_util::Full;


/// Deletes a task by name.
/// 
/// # Arguments
/// * `name` - The name of the task to delete.
/// 
/// # Returns
/// A `Response` with a body containing all the to-do items.
pub async fn delete_by_name(name: &str) -> Result<Response<Full<Bytes>>, NanoServiceError> {
    let _ = delete_core(name).await?;
    let json_body = safe_eject!(
        serde_json::to_string(&get_all_core().await?),
        NanoServiceErrorStatus::Unknown
    )?;
    safe_eject!(
        Response::builder()
                .header(header::CONTENT_TYPE, "application/json")
                .body(Full::new(Bytes::from(json_body))),
        NanoServiceErrorStatus::Unknown
    )
}