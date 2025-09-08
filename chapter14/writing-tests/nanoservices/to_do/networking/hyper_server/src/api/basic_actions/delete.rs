use glue::errors::{NanoServiceError, NanoServiceErrorStatus};
use glue::safe_eject;
use hyper::body::Bytes;
use hyper::{header, Response};
use to_do_core::api::basic_actions::{
    delete::delete as delete_core,
    get::get_all as get_all_core
};
use http_body_util::Full;
use to_do_dal::to_do_items::transactions::{
    delete::DeleteOne,
    get::GetAll
};
use glue::token::HeaderToken;
use auth_kernel::user_session::transactions::get::GetUserSession;


/// Deletes a task by name.
/// 
/// # Arguments
/// * `name` - The name of the task to delete.
/// 
/// # Returns
/// A `Response` with a body containing all the to-do items.
pub async fn delete_by_name<T, X>(name: &str, token: HeaderToken) -> Result<Response<Full<Bytes>>, NanoServiceError> 
where
    T: DeleteOne + GetAll,
    X: GetUserSession
{
    let user = X::get_user_session(token.unique_id).await?;
    let _ = delete_core::<T>(name, user.user_id).await?;
    let json_body = safe_eject!(
        serde_json::to_string(&get_all_core::<T>(user.user_id).await?),
        NanoServiceErrorStatus::Unknown
    )?;
    safe_eject!(
        Response::builder()
                .header(header::CONTENT_TYPE, "application/json")
                .body(Full::new(Bytes::from(json_body))),
        NanoServiceErrorStatus::Unknown
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use hyper::{
        service::service_fn,
        Request,
        Response,
        http::StatusCode,
    };
    use glue::token::HeaderToken;
    use to_do_dal::to_do_items::transactions::delete::DeleteOneResponse;
    use to_do_dal::to_do_items::schema::{ToDoItem, AllToDOItems};
    use std::future::Future;
    use auth_kernel::user_session::schema::UserSession;


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