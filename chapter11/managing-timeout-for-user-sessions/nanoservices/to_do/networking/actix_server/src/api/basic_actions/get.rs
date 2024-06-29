use dal::to_do_items::transactions::get::GetAll;
use core::api::basic_actions::get::get_all as get_all_core;
use actix_web::HttpResponse;
use glue::errors::NanoServiceError;
use glue::token::HeaderToken;
use auth_kernel::api::users::get::get_user_by_unique_id;


pub async fn get_all<T: GetAll>(token: HeaderToken) -> Result<HttpResponse, NanoServiceError> {
    let user = get_user_by_unique_id(token.unique_id).await?;
    Ok(HttpResponse::Ok().json(get_all_core::<T>(user.id).await?))
}
