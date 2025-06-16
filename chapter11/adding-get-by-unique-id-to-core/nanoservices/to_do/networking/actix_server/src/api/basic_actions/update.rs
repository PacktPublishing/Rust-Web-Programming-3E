use to_do_core::api::basic_actions::{
    update::update as update_core,
    get::get_all as get_all_core
};
use glue::{
    errors::NanoServiceError,
    token::HeaderToken
};
use actix_web::{
    HttpResponse,
    web::Json
};
use to_do_dal::to_do_items::transactions::{
    update::UpdateOne,
    get::GetAll
};
use to_do_dal::to_do_items::schema::ToDoItem;


/// Update an item in the to do list.
/// 
/// # Arguments
/// * `body` - The item to update.
/// 
/// # Returns
/// A JSON response containing all items in the to do list.
pub async fn update<T: UpdateOne + GetAll>(token: HeaderToken, body: Json<ToDoItem>) -> Result<HttpResponse, NanoServiceError> {
    let _ = update_core::<T>(body.into_inner()).await?;
    Ok(HttpResponse::Ok().json(get_all_core::<T>().await?))
}

