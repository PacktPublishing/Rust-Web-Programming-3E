use rocket::serde::json::Json;
use to_do_core::api::basic_actions::{
    delete::delete as delete_core,
    get::get_all as get_all_core
};
use to_do_dal::to_do_items::schema::AllToDOItems;
use glue::errors::NanoServiceError;
use to_do_dal::to_do_items::descriptors::SqlxPostGresDescriptor;
use glue::token::HeaderToken;
use auth_kernel::api::users::get::get_user_by_unique_id;
use to_do_dal::to_do_items::transactions::{
    delete::DeleteOne,
    get::GetAll
};
use auth_kernel::user_session::transactions::get::GetUserSession;
use auth_kernel::user_session::descriptors::RedisSessionDescriptor;


// Generic handler logic
pub async fn delete_by_name_generic<T, X>(
    token: HeaderToken,
    name: &str
) -> Result<Json<AllToDOItems>, NanoServiceError> 
where 
    T: DeleteOne + GetAll,
    X: GetUserSession
{
    let user = X::get_user_session(token.unique_id).await?;
    let _ = delete_core::<T>(name, user.user_id).await?;
    Ok(Json(get_all_core::<T>(user.user_id).await?))
}

/// Deletes a task by name.
/// 
/// # Arguments
/// * `name` - The name of the task to delete.
/// 
/// # Returns
/// A `Json` response with all the to-do items.
#[delete("/delete/<name>")]
pub async fn delete_by_name(token: HeaderToken, name: &str) -> Result<Json<AllToDOItems>, NanoServiceError> {
    delete_by_name_generic::<SqlxPostGresDescriptor, RedisSessionDescriptor>(token, name).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::{
        local::blocking::Client,
        http::{Status, ContentType},
        Route,
    };
    use glue::token::HeaderToken;
    use to_do_dal::to_do_items::transactions::delete::DeleteOneResponse;
    use to_do_dal::to_do_items::schema::{ToDoItem, AllToDOItems};
    use std::future::Future;
    use auth_kernel::user_session::schema::UserSession;
    use glue::errors::NanoServiceErrorStatus;


    fn generate_to_do_item() -> ToDoItem {
        ToDoItem {
            id: 1,
            title: "test".to_string(),
            status: "PENDING".to_string(),
        }
    }

    fn generate_get_all_return() -> Vec<ToDoItem> {
        vec![
            generate_to_do_item()
        ]
    }

    struct MockDbHandle;

    impl DeleteOne for MockDbHandle {
        fn delete_one(title: String, _user_id: i32) -> impl Future<Output = DeleteOneResponse> + Send {
            async fn run(title: String) -> DeleteOneResponse {
                if title == "coding" {
                    return Ok(generate_to_do_item())
                }
                return Err(
                    NanoServiceError::new(
                        "Item not found".to_string(),
                        NanoServiceErrorStatus::NotFound
                    )
                )
            }
            run(title)
        }
    }

    impl GetAll for MockDbHandle {
        fn get_all(user_id: i32) -> impl Future<Output = Result<Vec<ToDoItem>, NanoServiceError>> + Send {
            async fn run(user_id: i32) 
                -> Result<Vec<ToDoItem>, NanoServiceError> {
                if user_id == 2 {
                    return Err(
                        NanoServiceError::new(
                            "error getting items got get all".to_string(),
                            NanoServiceErrorStatus::Unknown
                        )
                    )
                }
                Ok(generate_get_all_return())
            }
            run(user_id)
        }
    }



}