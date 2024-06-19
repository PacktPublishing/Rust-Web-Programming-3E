use dal::to_do_items::transactions::{
    create::SaveOne,
    get::GetAll
};
use core::api::basic_actions::{
    create::create as create_core,
    get::get_all as get_all_core
};
use dal::to_do_items::schema::NewToDoItem;
use glue::{
    errors::NanoServiceError,
    token::HeaderToken
};
use actix_web::{
    HttpResponse,
    web::Json
};


pub async fn create<T: SaveOne + GetAll>(token: HeaderToken, body: Json<NewToDoItem>) 
    -> Result<HttpResponse, NanoServiceError> {
    let _ = create_core::<T>(body.into_inner()).await?;
    Ok(HttpResponse::Created().json(get_all_core::<T>().await?))
}
