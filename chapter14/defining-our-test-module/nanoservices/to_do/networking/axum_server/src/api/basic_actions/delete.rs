use to_do_core::api::basic_actions::{
    delete::delete as delete_core,
    get::get_all as get_all_core
};
use axum::{
    response::IntoResponse,
    extract::Path,
    http::StatusCode,
    Json,
};
use glue::errors::NanoServiceError;
use to_do_dal::to_do_items::transactions::{
    delete::DeleteOne,
    get::GetAll
};
use glue::token::HeaderToken;
use auth_kernel::api::users::get::get_user_by_unique_id;


/// Deletes an item from the to-do list by name.
/// 
/// # Arguments
/// - `req` - The HTTP request.
/// 
/// # Returns
/// All of the items in the to-do list.
pub async fn delete_by_name<T: DeleteOne + GetAll>(token: HeaderToken, Path(name): Path<String>) -> Result<impl IntoResponse, NanoServiceError> {
    let user = get_user_by_unique_id(
        token.unique_id
    ).await?;
    let _ = delete_core::<T>(&name, user.id).await?;
    Ok((StatusCode::OK, Json(get_all_core::<T>(user.id).await?)).into_response())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        Router,
        routing::delete,
        body::Body,
        http::{Request, StatusCode},
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