use to_do_core::api::basic_actions::{
    update::update as update_core,
    get::get_all as get_all_core
};
use to_do_core::structs::ToDoItem;
use glue::{
    errors::NanoServiceError,
    token::HeaderToken
};
use actix_web::{
    HttpResponse,
    web::Json
};


/// Update an item in the to do list.
/// 
/// # Arguments
/// * `body` - The item to update.
/// 
/// # Returns
/// A JSON response containing all items in the to do list.
pub async fn update(token: HeaderToken, body: Json<ToDoItem>) -> Result<HttpResponse, NanoServiceError> {
    let _ = update_core(body.into_inner()).await?;
    Ok(HttpResponse::Ok().json(get_all_core().await?))
}
