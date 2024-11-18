use glue::errors::{NanoServiceError, NanoServiceErrorStatus};
use glue::safe_eject;
use hyper::body::Bytes;
use hyper::{header, Response};
use to_do_core::api::basic_actions::get::{
    get_all as get_all_core,
    get_by_name as get_by_name_core
};
use http_body_util::Full;


/// Gets all tasks.
/// 
/// # Returns
/// A `Response` with a body containing all tasks or an error message.
pub async fn get_all() -> Result<Response<Full<Bytes>>, NanoServiceError> {
    let items = get_all_core().await?;
    let json_body = safe_eject!(
        serde_json::to_string(&items),
        NanoServiceErrorStatus::Unknown
    )?;
    safe_eject!(
        Response::builder()
                .header(header::CONTENT_TYPE, "application/json")
                .body(Full::new(Bytes::from(json_body))),
        NanoServiceErrorStatus::Unknown
    )
}


/// Gets a task by name.
/// 
/// # Arguments
/// * `name` - The name of the task to get.
/// 
/// # Returns
/// A `Response` with a body containing the task specified in the URL or an error message.
pub async fn get_by_name(name: &str) -> Result<Response<Full<Bytes>>, NanoServiceError> {
    let item = get_by_name_core(name).await?;
    let json_body = safe_eject!(
        serde_json::to_string(&item),
        NanoServiceErrorStatus::Unknown
    )?;
    safe_eject!(
        Response::builder()
                .header(header::CONTENT_TYPE, "application/json")
                .body(Full::new(Bytes::from(json_body))),
        NanoServiceErrorStatus::Unknown
    )
}
