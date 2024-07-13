use auth_dal::users::transactions::get::GetByUniqueId;
use auth_core::api::users::get::get_by_unique_id as get_by_unique_id_core;
use glue::errors::NanoServiceError;
use glue::token::HeaderToken;
use actix_web::HttpResponse;


pub async fn get_by_unique_id<T: GetByUniqueId>(token: HeaderToken)
    -> Result<HttpResponse, NanoServiceError> {
    let user = get_by_unique_id_core::<T>(token.unique_id).await?;
    Ok(HttpResponse::Ok().json(user))
}
