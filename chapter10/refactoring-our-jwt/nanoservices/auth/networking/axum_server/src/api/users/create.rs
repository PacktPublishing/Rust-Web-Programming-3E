use auth_dal::users::transactions::create::SaveOne;
use auth_core::api::users::create::{
    create as create_core,
    CreateUser
};
use auth_dal::users::schema::NewUser;
use glue::errors::NanoServiceError;
use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
};


pub async fn create<T: SaveOne>(Json(body): Json<CreateUser>) -> Result<impl IntoResponse, NanoServiceError> {
    let _ = create_core::<T>(body).await?;
    Ok(StatusCode::CREATED)
}
