use auth_dal::users::schema::{NewUser, User};
use auth_dal::users::transactions::create::SaveOne;
use glue::errors::NanoServiceError;
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Clone)]
pub struct CreateUser {
    pub email: String,
    pub password: String
}


pub async fn create<T: SaveOne>(data: CreateUser) -> Result<User, NanoServiceError> {
    let user = NewUser::new(data.email, data.password)?;
    let created_item = T::save_one(user).await?;
    Ok(created_item)
}
