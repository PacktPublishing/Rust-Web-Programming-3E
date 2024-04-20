use core::api::basic_actions::{
    create::create as create_core,
    get::get_all as get_all_core
};
use core::structs::ToDoItem;
use glue::{
    errors::NanoServiceError,
    token::HeaderToken
};
use actix_web::{
    HttpResponse,
    web::Json
};


pub async fn create(token: HeaderToken, body: Json<ToDoItem>) -> Result<HttpResponse, NanoServiceError> {
    println!("Token: {}", token.message);
    let _ = create_core(body.into_inner()).await?;
    Ok(HttpResponse::Created().json(get_all_core().await?))
}
