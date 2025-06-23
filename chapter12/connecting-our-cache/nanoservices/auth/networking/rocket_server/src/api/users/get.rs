use auth_core::api::users::get::get_by_unique_id as get_by_unique_id_core;
use glue::errors::NanoServiceError;
use rocket::serde::json::Json;
use auth_dal::users::descriptors::SqlxPostGresDescriptor;
use rocket::get;
use glue::token::HeaderToken;
use rocket::serde::Serialize;


#[get("/get")]
pub async fn get(token: HeaderToken) 
    -> Result<Json<impl Serialize>, NanoServiceError> {
    let user = get_by_unique_id_core::<SqlxPostGresDescriptor>(token.unique_id).await?;
    Ok(Json(user))
}
