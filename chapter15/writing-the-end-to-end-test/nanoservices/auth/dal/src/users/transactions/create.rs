use crate::users::schema::{NewUser, User};
use glue::errors::{NanoServiceError, NanoServiceErrorStatus};
use std::future::Future;
use crate::connections::sqlx_postgres::SQLX_POSTGRES_POOL;
use super::super::descriptors::SqlxPostGresDescriptor;


pub trait SaveOne {
    fn save_one(user: NewUser) -> impl Future<Output = Result<User, NanoServiceError>> + Send;
}


impl SaveOne for SqlxPostGresDescriptor {
    fn save_one(user: NewUser) -> impl Future<Output = Result<User, NanoServiceError>> + Send {
        sqlx_postgres_save_one(user)
    }
}


async fn sqlx_postgres_save_one(user: NewUser) -> Result<User, NanoServiceError> {
    let user = sqlx::query_as::<_, User>("
        INSERT INTO users (email, password, unique_id)
        VALUES ($1, $2, $3)
        RETURNING *"
    )
    .bind(user.email)
    .bind(user.password.to_string())
    .bind(user.unique_id)
    .fetch_one(&*SQLX_POSTGRES_POOL).await.map_err(|e| {
        NanoServiceError::new(e.to_string(), NanoServiceErrorStatus::Unknown)
    })?;
    Ok(user)
}
