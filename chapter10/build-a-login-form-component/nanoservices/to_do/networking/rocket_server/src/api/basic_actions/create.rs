use to_do_core::api::basic_actions::{
    create::create as create_core,
    get::get_all as get_all_core
};
use glue::errors::NanoServiceError;
use rocket::serde::json::Json;
use rocket::response::status::Created;
use to_do_dal::to_do_items::schema::{NewToDoItem, AllToDOItems};
use to_do_dal::to_do_items::descriptors::SqlxPostGresDescriptor;
use glue::token::HeaderToken;


/// Creates an item in the to-do list.
/// 
/// # Arguments
/// - `body` - The JSON body containing the item to be created.
/// 
/// # Returns
/// All of the items in the to-do list.
#[post("/create", data = "<body>")]
pub async fn create(token: HeaderToken, body: Json<NewToDoItem>) 
    -> Result<Created<Json<AllToDOItems>>, NanoServiceError> {
    let _ = create_core::<SqlxPostGresDescriptor>(body.into_inner()).await?;
    let all_items = get_all_core::<SqlxPostGresDescriptor>().await?;
    Ok(Created::new("/create").body(Json(all_items)))
}
