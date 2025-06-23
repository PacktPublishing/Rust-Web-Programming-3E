use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use auth_dal::users::transactions::get::GetByUniqueId;
use auth_core::api::users::get::get_by_unique_id as get_by_unique_id_core;
use glue::errors::NanoServiceError;
use glue::token::HeaderToken;


pub async fn get_by_unique_id<T: GetByUniqueId>(
    token: HeaderToken,
) -> Result<impl IntoResponse, NanoServiceError> {
    let user = get_by_unique_id_core::<T>(token.unique_id).await?;
    Ok((StatusCode::OK, Json(user)))
}
