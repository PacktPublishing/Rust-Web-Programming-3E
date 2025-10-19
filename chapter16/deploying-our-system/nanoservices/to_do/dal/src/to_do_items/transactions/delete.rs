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


pub type DeleteOneResponse = Result<ToDoItem, NanoServiceError>;


pub trait DeleteOne {
    fn delete_one(title: String, user_id: i32) -> impl Future<Output = DeleteOneResponse> + Send;
}

#[cfg(feature = "sqlx-postgres")]
impl DeleteOne for SqlxPostGresDescriptor {
    fn delete_one(title: String, user_id: i32) -> impl Future<Output = DeleteOneResponse> + Send {
        sqlx_postgres_delete_one(title, user_id)
    }
}

#[cfg(feature = "json-file")]
impl DeleteOne for JsonFileDescriptor {
    fn delete_one(title: String, user_id: i32) -> impl Future<Output = DeleteOneResponse> + Send {
        json_file_delete_one(title, user_id)
    }
}


#[cfg(feature = "sqlx-postgres")]
async fn sqlx_postgres_delete_one(title: String, user_id: i32) -> DeleteOneResponse {
    let item = sqlx::query_as::<_, ToDoItem>("
        DELETE FROM to_do_items
        WHERE title = $1
        RETURNING *"
    ).bind(title)
    .fetch_one(&*SQLX_POSTGRES_POOL).await.map_err(|e| {
        NanoServiceError::new(e.to_string(), NanoServiceErrorStatus::Unknown)
    })?;
    let _ = sqlx::query("
        DELETE FROM user_connections
        WHERE user_id = $1 AND to_do_id = $2"
    ).bind(user_id)
    .bind(item.id)
    .execute(&*SQLX_POSTGRES_POOL).await.map_err(|e| {
        NanoServiceError::new(e.to_string(), NanoServiceErrorStatus::Unknown)
    })?;
    Ok(item)
}


#[cfg(feature = "json-file")]
async fn json_file_delete_one(title: String, user_id: i32) -> DeleteOneResponse {
    let mut tasks = get_all::<ToDoItem>().unwrap_or_else(|_| HashMap::new());
    let to_do_item = tasks.remove(&title + ":" + &user_id.to_string()).ok_or_else(|| {
        NanoServiceError::new("Item not found".to_string(), NanoServiceErrorStatus::NotFound)
    })?;
    let _ = save_all(&tasks)?;
    Ok(to_do_item)
}


#[cfg(feature = "sqlx-postgres")]
#[cfg(test)]
mod tests {

    use super::*;
    use crate::migrations::run_migrations;
    use glue::sql_commands::WIPE_DB;

    async fn setup() {
        sqlx::query(WIPE_DB).execute(&*SQLX_POSTGRES_POOL).await.unwrap();
        run_migrations().await;
    }

    #[tokio::test]
    async fn test_sqlx_postgres_delete_one_ok() {
        setup().await;
        // define the item to be deleted
        let title = "test_sqlx_postgres_delete_one".to_string();
        let user_id = 1;
        let item = ToDoItem {
            id: 1,
            title: title.clone(),
            status: "PENDING".to_string()
        };

        // insert the item into the database
        let item = sqlx::query_as::<_, ToDoItem>("
            INSERT INTO to_do_items (title, status)
            VALUES ($1, $2)
            RETURNING *"
        ).bind(item.title)
        .bind(item.status.to_string())
        .fetch_one(&*SQLX_POSTGRES_POOL).await.unwrap();

        // insert the user connection
        let _ = sqlx::query("
            INSERT INTO user_connections (user_id, to_do_id) 
            VALUES ($1, $2)"
        ).bind(user_id)
        .bind(item.id)
        .execute(&*SQLX_POSTGRES_POOL).await.unwrap();

        // delete the item
        let result = sqlx_postgres_delete_one(title.clone(), user_id).await.unwrap();
        assert_eq!(result.title, title);
        
        // check that the item is no longer in the database
        let result = sqlx::query_as::<_, ToDoItem>("
            SELECT * FROM to_do_items
            WHERE title = $1"
        ).bind(title)
        .fetch_optional(&*SQLX_POSTGRES_POOL).await.unwrap();
        assert!(result.is_none());

        // check that the user connection is no longer in the database
        let result = sqlx::query_as::<_, (i64,)>("
            SELECT COUNT(*) FROM user_connections
            WHERE user_id = $1 AND to_do_id = $2"
        ).bind(user_id)
        .bind(item.id)
        .fetch_one(&*SQLX_POSTGRES_POOL).await.unwrap();
        assert_eq!(result.0, 0);
    }

    #[tokio::test]
    async fn test_sqlx_postgres_delete_no_existing_item() {
        setup().await;
        let title = "test_sqlx_postgres_delete_one_not_found".to_string();
        let user_id = 1;
        let result = sqlx_postgres_delete_one(
            title.clone(), user_id
        ).await;
        assert!(result.is_err());
    }

}