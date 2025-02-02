use to_do_core::api::basic_actions::get::get_all as get_all_core;
use actix_web::HttpResponse;
use glue::errors::NanoServiceError;
use to_do_dal::to_do_items::transactions::get::GetAll;



/// Gets all tasks.
/// 
/// # Returns
/// An `HttpResponse` with a JSON body containing all tasks.
pub async fn get_all<T: GetAll>() -> Result<HttpResponse, NanoServiceError> {
    Ok(HttpResponse::Ok().json(get_all_core::<T>().await?))
}
