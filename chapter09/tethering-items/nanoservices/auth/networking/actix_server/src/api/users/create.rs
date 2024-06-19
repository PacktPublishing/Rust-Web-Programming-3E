use auth_dal::users::transactions::create::SaveOne;
use auth_core::api::users::create::{
    create as create_core,
    CreateUser
};
use glue::errors::NanoServiceError;
use actix_web::{
    HttpResponse,
    web::Json
};


pub async fn create<T: SaveOne>(body: Json<CreateUser>) 
    -> Result<HttpResponse, NanoServiceError> {
    let _ = create_core::<T>(body.into_inner()).await?;
    Ok(HttpResponse::Created().finish())
}
