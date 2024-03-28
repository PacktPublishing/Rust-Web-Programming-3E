use core::api::basic_actions::get::get_all as get_all_core;
use actix_web::HttpResponse;


pub async fn get_all() -> HttpResponse {
    let all_items = match get_all_core().await {
        Ok(items) => items,
        Err(e) => return HttpResponse::InternalServerError().json(e)
    };
    HttpResponse::Ok().json(all_items)
}
