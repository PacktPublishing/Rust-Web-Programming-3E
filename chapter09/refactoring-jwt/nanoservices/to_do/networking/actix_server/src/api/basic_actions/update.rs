use dal::to_do_items::transactions::{
    update::UpdateOne,
    get::GetAll
};
use core::api::basic_actions::{
    update::update as update_core,
    get::get_all as get_all_core
};
use dal::to_do_items::schema::ToDoItem;
use glue::errors::NanoServiceError;
use actix_web::{
    HttpResponse,
    web::Json
};


pub async fn update<T: UpdateOne + GetAll>(body: Json<ToDoItem>) -> Result<HttpResponse, NanoServiceError> {
    println!("Updating item {:?}", body);
    let _ = update_core::<T>(body.into_inner()).await?;
    Ok(HttpResponse::Ok().json(get_all_core::<T>().await?))
}
