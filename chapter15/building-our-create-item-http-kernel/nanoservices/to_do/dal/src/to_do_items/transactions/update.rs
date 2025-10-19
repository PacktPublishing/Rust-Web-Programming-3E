use crate::to_do_items::schema::ToDoItem;
use glue::errors::NanoServiceError;
use std::future::Future;

#[cfg(feature = "json-file")]
use super::super::descriptors::JsonFileDescriptor;
#[cfg(feature = "json-file")]
use crate::json_file::{get_all, save_all};
#[cfg(feature = "json-file")]
use std::collections::HashMap;

#[cfg(feature = "sqlx-postgres")]
use crate::connections::sqlx_postgres::SQLX_POSTGRES_POOL;
#[cfg(feature = "sqlx-postgres")]
use super::super::descriptors::SqlxPostGresDescriptor;

#[cfg(any(feature = "json-file", feature = "sqlx-postgres"))]
use glue::errors::NanoServiceErrorStatus;


pub type UpdateOneResponse = Result<ToDoItem, NanoServiceError>;


pub trait UpdateOne {
    fn update_one(item: ToDoItem, user_id: i32) -> impl Future<Output = UpdateOneResponse> + Send;
}


#[cfg(feature = "sqlx-postgres")]
impl UpdateOne for SqlxPostGresDescriptor {
    fn update_one(item: ToDoItem, _user_id: i32) -> impl Future<Output = UpdateOneResponse> + Send {
        sqlx_postgres_update_one(item)
    }
}


#[cfg(feature = "json-file")]
impl UpdateOne for JsonFileDescriptor {
    fn update_one(item: ToDoItem, user_id: i32) -> impl Future<Output = UpdateOneResponse> + Send {
        json_file_update_one(item, user_id)
    }
}


#[cfg(feature = "sqlx-postgres")]
async fn sqlx_postgres_update_one(item: ToDoItem) -> UpdateOneResponse {
    let item = sqlx::query_as::<_, ToDoItem>("
        UPDATE to_do_items
        SET title = $1, status = $2
        WHERE id = $3
        RETURNING *"
    ).bind(item.title)
    .bind(item.status.to_string())
    .bind(item.id)
    .fetch_one(&*SQLX_POSTGRES_POOL).await.map_err(|e| {
        NanoServiceError::new(e.to_string(), NanoServiceErrorStatus::Unknown)
    })?;
    Ok(item)
}


#[cfg(feature = "json-file")]
async fn json_file_update_one(item: ToDoItem, user_id: i32) -> UpdateOneResponse {
    let mut tasks = get_all::<ToDoItem>().unwrap_or_else(|_| HashMap::new());
    let key = item.title.clone() + ":" + &user_id.to_string();
    if !tasks.contains_key(&key) {
        return Err(NanoServiceError::new(
            format!("Item with name {} not found", item.title),
            NanoServiceErrorStatus::NotFound
        ));
    }
    tasks.insert(key, item.clone());
    let _ = save_all(&tasks)?;
    Ok(item)
}
