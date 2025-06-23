use glue::errors::{NanoServiceError, NanoServiceErrorStatus};
use glue::safe_eject;
use hyper::body::Bytes;
use hyper::{header, Response};
use to_do_core::api::basic_actions::get::get_all as get_all_core;
use http_body_util::Full;
use to_do_dal::to_do_items::transactions::get::GetAll;
use glue::token::HeaderToken;
use auth_kernel::api::users::get::get_user_by_unique_id;


/// Gets all tasks.
/// 
/// # Returns
/// A `Response` with a body containing all tasks or an error message.
pub async fn get_all<T: GetAll>(token: HeaderToken) -> Result<Response<Full<Bytes>>, NanoServiceError> {
    let user = get_user_by_unique_id(
        token.unique_id
    ).await?;
    let items = get_all_core::<T>(user.id).await?;
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
