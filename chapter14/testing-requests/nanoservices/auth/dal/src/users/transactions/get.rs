use crate::users::schema::User;
use glue::errors::{NanoServiceError, NanoServiceErrorStatus};
use std::future::Future;

use super::super::descriptors::SqlxPostGresDescriptor;
use crate::connections::sqlx_postgres::SQLX_POSTGRES_POOL;


pub trait GetByEmail {
    fn get_by_email(email: String) -> impl Future<Output = Result<User, NanoServiceError>> + Send;
}


pub trait GetByUniqueId {
    fn get_by_unique_id(id: String) -> impl Future<Output = Result<User, NanoServiceError>> + Send;
}


impl GetByEmail for SqlxPostGresDescriptor {
    fn get_by_email(email: String) -> impl Future<Output = Result<User, NanoServiceError>> + Send {
        sqlx_postgres_get_by_email(email)
    }
}


impl GetByUniqueId for SqlxPostGresDescriptor {
    fn get_by_unique_id(id: String) -> impl Future<Output = Result<User, NanoServiceError>> + Send {
        sqlx_postgres_get_by_unique_id(id)
    }
}


async fn sqlx_postgres_get_by_email(email: String) -> Result<User, NanoServiceError> {
    let item = sqlx::query_as::<_, User>("
        SELECT * FROM users WHERE email = $1"
    ).bind(email)
    .fetch_optional(&*SQLX_POSTGRES_POOL).await.map_err(|e| {
        NanoServiceError::new(e.to_string(), NanoServiceErrorStatus::Unknown)
    })?;
    match item {
        None => Err(NanoServiceError::new(
            "User not found".to_string(), 
            NanoServiceErrorStatus::NotFound
        )),
        Some(item) => Ok(item)
    }
}


async fn sqlx_postgres_get_by_unique_id(id: String) -> Result<User, NanoServiceError> {
    let item = sqlx::query_as::<_, User>("
        SELECT * FROM users WHERE unique_id = $1"
    ).bind(id)
    .fetch_optional(&*SQLX_POSTGRES_POOL).await.map_err(|e| {
        NanoServiceError::new(e.to_string(), NanoServiceErrorStatus::Unknown)
    })?;
    match item {
        None => Err(NanoServiceError::new(
            "User not found".to_string(), 
            NanoServiceErrorStatus::NotFound
        )),
        Some(item) => Ok(item)
    }
}