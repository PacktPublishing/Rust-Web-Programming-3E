use to_do_core::api::basic_actions::{
    create::create as create_core,
    get::get_all as get_all_core
};
use glue::errors::NanoServiceError;
use actix_web::{
    HttpResponse,
    web::Json
};
use to_do_dal::to_do_items::schema::NewToDoItem;
use to_do_dal::to_do_items::transactions::{
    create::SaveOne,
    get::GetAll
};
use glue::token::HeaderToken;



/// Creates an item in the to-do list.
/// 
/// # Arguments
/// - `body` - The JSON body containing the item to be created.
/// 
/// # Returns
/// All of the items in the to-do list.
pub async fn create<T: SaveOne + GetAll>(
    token: HeaderToken, 
    body: Json<NewToDoItem>
) -> Result<HttpResponse, NanoServiceError> {
let _ = create_core::<T>(body.into_inner()).await?;
Ok(HttpResponse::Created().json(get_all_core::<T>().await?))
}

