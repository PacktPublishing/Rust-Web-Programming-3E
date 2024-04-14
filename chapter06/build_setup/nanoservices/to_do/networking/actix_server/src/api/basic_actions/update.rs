use core::api::basic_actions::{
    update::update as update_core,
    get::get_all as get_all_core
};
use core::structs::ToDoItem;
use glue::errors::NanoServiceError;
use actix_web::{
    HttpResponse,
    web::Json
};


pub async fn update(body: Json<ToDoItem>) -> Result<HttpResponse, NanoServiceError> {
    let _ = update_core(body.into_inner()).await?;
    Ok(HttpResponse::Ok().json(get_all_core().await?))
}
