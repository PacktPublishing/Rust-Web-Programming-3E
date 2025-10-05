
#[cfg(test)]
mod tests {
    use to_do_kernel::api::basic_actions::delete::delete_item;
    use dal::connections::sqlx_postgres::SQLX_POSTGRES_POOL;
    use dal::to_do_items::schema::ToDoItem;
    use dal::migrations::run_migrations;
    use glue::sql_commands::WIPE_DB;
    use glue::token::HeaderToken;
    use glue::urls::get_cache_url;
    use cache_client::login;

    async fn setup() {
        sqlx::query(WIPE_DB).execute(
            &*SQLX_POSTGRES_POOL
        ).await.unwrap();
        run_migrations().await;
    }
    #[tokio::test]
    async fn test_delete_item() {
        setup().await;
        let user_id = "123".to_string();
        let perm_user_id = 1;

        login(
            &get_cache_url().unwrap(), 
            &user_id, 
            60, 
            perm_user_id
        ).await.unwrap();

        let title = "test_sqlx_postgres_delete_one".to_string();
        let item = ToDoItem {
            id: 1,
            title: title.clone(),
            status: "PENDING".to_string()
        };
        let item = sqlx::query_as::<_, ToDoItem>("
            INSERT INTO to_do_items (title, status)
            VALUES ($1, $2)
            RETURNING *"
        ).bind(item.title)
        .bind(item.status.to_string())
        .fetch_one(&*SQLX_POSTGRES_POOL).await.unwrap();

        assert_eq!(item.title, title);
        assert_eq!(item.status, "PENDING");
        assert_eq!(item.id, 1);

        let _ = sqlx::query("
            INSERT INTO user_connections (user_id, to_do_id) 
            VALUES ($1, $2)"
        ).bind(perm_user_id)
        .bind(item.id)
        .execute(&*SQLX_POSTGRES_POOL).await.unwrap();

        let token = HeaderToken {
            unique_id: user_id.clone()
        };
        let outcome = delete_item(token, title).await.unwrap();
        assert_eq!(outcome.done.len(), 0);
        assert_eq!(outcome.pending.len(), 0);

        let items: Vec<ToDoItem> = sqlx::query_as::<_, ToDoItem>(
            "SELECT * FROM to_do_items;"
        )
        .fetch_all(&*SQLX_POSTGRES_POOL)
        .await
        .unwrap();
        assert_eq!(items.len(), 0);

        let connections: Vec<(i32, i32)> = sqlx::query_as::<_, (i32, i32)>(
            "SELECT * FROM user_connections;"
        )
        .fetch_all(&*SQLX_POSTGRES_POOL)
        .await
        .unwrap();
        assert_eq!(connections.len(), 0);
    }
}
