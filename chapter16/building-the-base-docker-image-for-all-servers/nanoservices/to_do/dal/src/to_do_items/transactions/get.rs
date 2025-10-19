use crate::to_do_items::schema::ToDoItem;
use glue::errors::NanoServiceError;
use std::future::Future;

#[cfg(feature = "json-file")]
use super::super::descriptors::JsonFileDescriptor;
#[cfg(feature = "json-file")]
use crate::json_file::get_all;
#[cfg(feature = "json-file")]
use std::collections::HashMap;

#[cfg(feature = "sqlx-postgres")]
use crate::connections::sqlx_postgres::SQLX_POSTGRES_POOL;
#[cfg(feature = "sqlx-postgres")]
use super::super::descriptors::SqlxPostGresDescriptor;
#[cfg(feature = "sqlx-postgres")]
use glue::errors::NanoServiceErrorStatus;


pub type GetAllResponse = Result<Vec<ToDoItem>, NanoServiceError>;

pub trait GetAll {
    fn get_all(user_id: i32) -> impl Future<Output = GetAllResponse> + Send;
}


#[cfg(feature = "sqlx-postgres")]
impl GetAll for SqlxPostGresDescriptor {
    fn get_all(user_id: i32) -> impl Future<Output = GetAllResponse> + Send {
        sqlx_postgres_get_all(user_id)
    }
}


#[cfg(feature = "json-file")]
impl GetAll for JsonFileDescriptor {
    fn get_all(user_id: i32) -> impl Future<Output = GetAllResponse> + Send {
        json_file_get_all(user_id)
    }
}


#[cfg(feature = "sqlx-postgres")]
async fn sqlx_postgres_get_all(user_id: i32) -> GetAllResponse {
    let items = sqlx::query_as::<_, ToDoItem>("
        SELECT * FROM to_do_items WHERE id IN (
            SELECT to_do_id FROM user_connections WHERE user_id = $1
        )")
    .bind(user_id)
    .fetch_all(&*SQLX_POSTGRES_POOL).await.map_err(|e| {
        NanoServiceError::new(e.to_string(), NanoServiceErrorStatus::Unknown)
    })?;
    Ok(items)
}


#[cfg(feature = "json-file")]
async fn json_file_get_all(user_id: i32) -> GetAllResponse {
    let tasks = get_all::<ToDoItem>().unwrap_or_else(|_| HashMap::new());
    let items = tasks.values().cloned().collect();
    let mut filtered_items: Vec<ToDoItem> = Vec::new();
    for item in items {
        let key = item.id.to_string().split(":").nth(1).unwrap();
        let item_user_id = key.parse::<i32>().unwrap();
        if item_user_id == user_id {
            filtered_items.push(item);
        }
    }
    Ok(filtered_items)
}
