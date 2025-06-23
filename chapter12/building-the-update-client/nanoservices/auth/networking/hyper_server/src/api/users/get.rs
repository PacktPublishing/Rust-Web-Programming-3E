use glue::errors::{NanoServiceError, NanoServiceErrorStatus};
use auth_core::api::users::get::get_by_unique_id as get_by_unique_id_core;
use glue::token::HeaderToken;
use glue::safe_eject;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::{header, Response};
use auth_dal::users::transactions::get::GetByUniqueId;
use serde_json::to_vec;


pub async fn get_by_unique_id<T: GetByUniqueId>(token: HeaderToken) -> Result<Response<Full<Bytes>>, NanoServiceError> {
    let user = get_by_unique_id_core::<T>(token.unique_id).await?;
    let json_body = to_vec(&user).map_err(|e| NanoServiceError::new(
        e.to_string(),
        NanoServiceErrorStatus::Unknown,
    ))?;
    safe_eject!(
        Response::builder()
                .header(header::CONTENT_TYPE, "application/json")
                .status(200)
                .body(Full::new(Bytes::from(json_body))),
        NanoServiceErrorStatus::Unknown
    )
}