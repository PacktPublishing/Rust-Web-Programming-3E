

#[cfg(test)]
mod tests {

    use glue::token::HeaderToken;
    use to_do_kernel::api::basic_actions::create::create_item;
    use to_do_kernel::api::basic_actions::delete::delete_item;
    use dal::to_do_items::enums::TaskStatus;
    use auth_kernel::api::users::create::create_user;
    use auth_core::api::users::create::CreateUser;
    use auth_kernel::api::auth::login::login_user;

    use dal::migrations::run_migrations as run_todo_migrations;
    use auth_dal::migrations::run_migrations as run_auth_migrations;
    use dal::connections::sqlx_postgres::SQLX_POSTGRES_POOL as todo_pool;
    use auth_dal::connections::sqlx_postgres::SQLX_POSTGRES_POOL as auth_pool;
    use glue::sql_commands::WIPE_DB;

    async fn setup() {
        sqlx::query(WIPE_DB).execute(&*todo_pool).await.unwrap();
        sqlx::query(WIPE_DB).execute(&*auth_pool).await.unwrap();
        run_todo_migrations().await;
        run_auth_migrations().await;
    }

    #[tokio::test]
    async fn test_delete_item_ok() {

        setup().await;

        let user = CreateUser {
            email: "test@gmail.com".to_string(),
            password: "test".to_string(),
        };

        println!("creating user");
        create_user(user).await.unwrap();
        println!("user created");

        let token = login_user("test@gmail.com".to_string(), "test".to_string()).await.unwrap();
        println!("user logged in");

        let header_token = HeaderToken::decode(&token).unwrap();
        let header_token_ref = HeaderToken::decode(&token).unwrap();
        println!("token decoded");

        let items = match create_item(
            header_token_ref, 
            "code".to_string(), 
            TaskStatus::PENDING
        ).await {
            Ok(items) => items,
            Err(e) => {
                println!("error: {:?}", e);
                panic!("error creating item");
            }
        };
        assert_eq!(items.done.len(), 0);
        assert_eq!(items.pending.len(), 1);

        let items = delete_item(header_token, "code".to_string()).await.unwrap();

        assert_eq!(items.done.len(), 0);
        assert_eq!(items.pending.len(), 0);
    }

}
