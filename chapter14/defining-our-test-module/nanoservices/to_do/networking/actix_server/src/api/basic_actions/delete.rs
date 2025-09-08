use to_do_core::api::basic_actions::{
    delete::delete as delete_core,
    get::get_all as get_all_core
};
use actix_web::{
    HttpResponse,
    HttpRequest
};
use glue::errors::{
    NanoServiceError,
    NanoServiceErrorStatus
};
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
pub async fn delete_by_name<T: DeleteOne + GetAll>(
    token: HeaderToken, 
    req: HttpRequest
) -> Result<HttpResponse, NanoServiceError> {
    let user = get_user_by_unique_id(
        token.unique_id
    ).await?;
    match req.match_info().get("name") {
        Some(name) => {
            delete_core::<T>(name, user.id).await?;
        },
        None => {
            return Err(
                NanoServiceError::new(
                    "Name not provided".to_string(),
                    NanoServiceErrorStatus::BadRequest
                )
            )
        }
    };
    Ok(HttpResponse::Ok().json(get_all_core::<T>(
        user.id
    ).await?))
}


#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{
        dev::ServiceResponse,
        self, body::MessageBody, http::header::ContentType, test::{
            call_service, init_service, TestRequest
        }, web, App
    };
    use actix_http::Request;
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
