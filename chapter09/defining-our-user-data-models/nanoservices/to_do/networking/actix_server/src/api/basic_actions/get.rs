use dal::to_do_items::transactions::get::GetAll;
use core::api::basic_actions::get::get_all as get_all_core;
use actix_web::HttpResponse;
use glue::errors::NanoServiceError;


pub async fn get_all<T: GetAll>() -> Result<HttpResponse, NanoServiceError> {
    Ok(HttpResponse::Ok().json(get_all_core::<T>().await?))
}
