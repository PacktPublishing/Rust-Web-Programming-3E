use to_do_core::api::basic_actions::{
    create::create as create_core,
    get::get_all as get_all_core
};
use to_do_core::structs::ToDoItem;
use glue::errors::NanoServiceError;
use actix_web::{
    HttpResponse,
    web::Json
};


/// Creates an item in the to-do list.
/// 
/// # Arguments
/// - `body` - The JSON body containing the item to be created.
/// 
/// # Returns
/// All of the items in the to-do list.
pub async fn create(body: Json<ToDoItem>) 
    -> Result<HttpResponse, NanoServiceError> {
    let _ = create_core(body.into_inner()).await?;
    Ok(HttpResponse::Created().json(get_all_core().await?))
}
