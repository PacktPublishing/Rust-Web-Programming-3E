use glue::errors::{NanoServiceError, NanoServiceErrorStatus};
use glue::safe_eject;
use to_do_core::api::basic_actions::{
    create::create as create_core,
    get::get_all as get_all_core
};
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::{header, Response, Request, body::Incoming};
use to_do_core::structs::ToDoItem;
use glue::hyper_utils::extract_body::extract_body;


/// Creates an item in the to-do list.
/// 
/// # Arguments
/// - `req` - The request containing the JSON body with the item to be created.
/// 
/// # Returns
/// All of the items in the to-do list.
pub async fn create(req: Request<Incoming>) -> Result<Response<Full<Bytes>>, NanoServiceError> {
    let todo_item = extract_body::<ToDoItem>(req).await?;
    create_core(todo_item).await?;
    let json_body = safe_eject!(
        serde_json::to_string(&get_all_core().await?),
        NanoServiceErrorStatus::Unknown
    )?;
    safe_eject!(
        Response::builder()
                .header(header::CONTENT_TYPE, "application/json")
                .status(201)
                .body(Full::new(Bytes::from(json_body))),
        NanoServiceErrorStatus::Unknown
    )
}
