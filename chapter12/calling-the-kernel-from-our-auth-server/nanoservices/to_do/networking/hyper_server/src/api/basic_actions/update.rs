use glue::errors::{NanoServiceError, NanoServiceErrorStatus};
use glue::safe_eject;
use glue::token::HeaderToken;
use to_do_core::api::basic_actions::{
    update::update as update_core,
    get::get_all as get_all_core
};
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::{header, Response, Request, body::Incoming};
use glue::hyper_utils::extract_body::extract_body;
use to_do_dal::to_do_items::transactions::{
    update::UpdateOne,
    get::GetAll
};
use to_do_dal::to_do_items::schema::ToDoItem;
use auth_kernel::user_session::transactions::get::GetUserSession;


/// Updates an item in the to-do list.
/// 
/// # Arguments
/// - `req` - The request containing the JSON body with the item to be updated.
/// 
/// # Returns
/// All of the items in the to-do list.
pub async fn update<T: UpdateOne + GetAll, X: GetUserSession>(req: Request<Incoming>, token: HeaderToken) -> Result<Response<Full<Bytes>>, NanoServiceError> {
    let todo_item = extract_body::<ToDoItem>(req).await?;
    let session = X::get_user_session(token.unique_id).await?;
    update_core::<T>(todo_item, session.user_id).await?;
    let json_body = safe_eject!(
        serde_json::to_string(&get_all_core::<T>(session.user_id).await?),
        NanoServiceErrorStatus::Unknown
    )?;
    safe_eject!(
        Response::builder()
                .header(header::CONTENT_TYPE, "application/json")
                .body(Full::new(Bytes::from(json_body))),
        NanoServiceErrorStatus::Unknown
    )
}
