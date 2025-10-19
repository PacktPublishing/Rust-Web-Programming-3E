use crate::users::schema::User;
use glue::errors::{NanoServiceError, NanoServiceErrorStatus};
use std::future::Future;
use uuid::Uuid;

use super::super::descriptors::SqlxPostGresDescriptor;
use crate::connections::sqlx_postgres::SQLX_POSTGRES_POOL;


pub trait RefreshUser {
    fn refresh_user(user: User) 
    -> impl Future<Output = Result<String, NanoServiceError>> + Send;
}


impl RefreshUser for SqlxPostGresDescriptor {
    fn refresh_user(user: User) 
    -> impl Future<Output = Result<String, NanoServiceError>> + Send {
        sqlx_postgres_refresh_user(user)
    }
}


async fn sqlx_postgres_refresh_user(user: User) 
    -> Result<String, NanoServiceError> {
    let new_unique_id = Uuid::new_v4().to_string();
    
    let rows_affected = sqlx::query(
        "UPDATE users SET unique_id = $1 WHERE unique_id = $2"
    )
    .bind(user.unique_id)
    .bind(&new_unique_id)
    .execute(&*SQLX_POSTGRES_POOL)
    .await
    .map_err(|e| NanoServiceError::new(
        e.to_string(), 
        NanoServiceErrorStatus::Unknown
    ))?
    .rows_affected();
    
    if rows_affected == 0 {
        return Err(NanoServiceError::new(
            "User not found".to_string(), 
            NanoServiceErrorStatus::NotFound
        ));
    }
    if rows_affected > 1 {
        return Err(NanoServiceError::new(
            "More than one user found".to_string(), 
            NanoServiceErrorStatus::Unknown
        ));
    }
    Ok(new_unique_id)
}
