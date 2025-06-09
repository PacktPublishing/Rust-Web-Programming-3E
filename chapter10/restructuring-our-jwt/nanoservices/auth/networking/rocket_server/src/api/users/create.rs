use auth_core::api::users::create::{
    create as create_core,
    CreateUser
};
use glue::errors::NanoServiceError;
use rocket::serde::json::Json;
use rocket::response::status::Created;
use auth_dal::users::descriptors::SqlxPostGresDescriptor;


/// Creates an item in the to-do list.
/// 
/// # Arguments
/// - `body` - The JSON body containing the item to be created.
/// 
/// # Returns
/// All of the items in the to-do list.
#[post("/create", data = "<body>")]
pub async fn create(body: Json<CreateUser>) 
    -> Result<Created<Json<()>>, NanoServiceError> {
    let _ = create_core::<SqlxPostGresDescriptor>(body.into_inner()).await?;
    Ok(Created::new("/create"))
}
