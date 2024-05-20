use core::api::basic_actions::{
    create::create as create_core,
    get::get_all as get_all_core
};
use dal::to_do_items::schema::ToDoItem;
use glue::{
    errors::NanoServiceError,
    token::HeaderToken
};
use actix_web::{
    HttpResponse,
    web::Json
};


pub async fn create(token: HeaderToken, body: Json<ToDoItem>) -> Result<HttpResponse, NanoServiceError> {
    let _ = create_core(body.into_inner()).await?;
    Ok(HttpResponse::Created().json(get_all_core().await?))
}
