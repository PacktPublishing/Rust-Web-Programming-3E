use crate::to_do_items::schema::{ToDoItem, NewToDoItem};
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
#[cfg(feature = "sqlx-postgres")]
use glue::errors::NanoServiceErrorStatus;


pub type SaveOneResponse = Result<ToDoItem, NanoServiceError>;


pub trait SaveOne {
    fn save_one(item: NewToDoItem, user_id: i32) -> impl Future<Output = SaveOneResponse> + Send;
}

#[cfg(feature = "sqlx-postgres")]
impl SaveOne for SqlxPostGresDescriptor {
    fn save_one(item: NewToDoItem, user_id: i32) -> impl Future<Output = SaveOneResponse> + Send {
        sqlx_postgres_save_one(item, user_id)
    }
}

#[cfg(feature = "json-file")]
impl SaveOne for JsonFileDescriptor {
    fn save_one(item: NewToDoItem, user_id: i32) -> impl Future<Output = SaveOneResponse> + Send {
        json_file_save_one(item, user_id)
    }
}

#[cfg(feature = "sqlx-postgres")]
async fn sqlx_postgres_save_one(item: NewToDoItem, user_id: i32) -> SaveOneResponse {
    let item = sqlx::query_as::<_, ToDoItem>("
        INSERT INTO to_do_items (title, status)
        VALUES ($1, $2)
        RETURNING *"
    ).bind(item.title)
    .bind(item.status.to_string())
    .fetch_one(&*SQLX_POSTGRES_POOL).await.map_err(|e| {
        NanoServiceError::new(e.to_string(), NanoServiceErrorStatus::Unknown)
    })?;
    let _ = sqlx::query("
        INSERT INTO user_connections (user_id, to_do_id) 
        VALUES ($1, $2)"
    ).bind(user_id)
    .bind(item.id)
    .execute(&*SQLX_POSTGRES_POOL).await.map_err(|e| {
        NanoServiceError::new(e.to_string(), NanoServiceErrorStatus::Unknown)
    })?;
    Ok(item)
}


#[cfg(feature = "json-file")]
async fn json_file_save_one(item: NewToDoItem, user_id: i32) -> SaveOneResponse {
    let mut tasks = get_all::<ToDoItem>().unwrap_or_else(|_| HashMap::new());
    let to_do_item = ToDoItem {
        id: 1,
        title: item.title,
        status: item.status.to_string()
    };
    tasks.insert(to_do_item.title.to_string() + ":" + &user_id.to_string(), to_do_item.clone());
    let _ = save_all(&tasks)?;
    Ok(to_do_item)
}
