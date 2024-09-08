use to_do_core::api::basic_actions::get::get_all as get_all_core;
use actix_web::HttpResponse;


/// Gets all tasks.
/// 
/// # Returns
/// An `HttpResponse` with a JSON body containing all tasks.
pub async fn get_all() -> HttpResponse {
    let all_items = match get_all_core().await {
        Ok(items) => items,
        Err(e) => return HttpResponse::InternalServerError().json(e)
    };
    HttpResponse::Ok().json(all_items)
}
