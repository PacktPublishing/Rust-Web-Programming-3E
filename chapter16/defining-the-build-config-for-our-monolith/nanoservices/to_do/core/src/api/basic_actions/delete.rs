use dal::to_do_items::transactions::delete::DeleteOne;
use glue::errors::NanoServiceError;


pub async fn delete<T: DeleteOne>(id: &str, user_id: i32) -> Result<(), NanoServiceError> {
    let _ = T::delete_one(id.to_string(), user_id).await?;
    Ok(())
}


#[cfg(test)]
mod tests {

    use super::*;
    use dal::to_do_items::schema::ToDoItem;
    use dal::to_do_items::transactions::delete::DeleteOneResponse;
    use std::future::Future;

    #[tokio::test]
    async fn test_delete_ok() {

        struct ReturnOneMock;

        impl DeleteOne for ReturnOneMock {
            fn delete_one(id: String, user_id: i32) -> impl Future<Output = DeleteOneResponse> + Send {
                if id != "coding" {
                    panic!("Invalid title");
                }
                if user_id != 1 {
                    panic!("Invalid user_id");
                }
                async {
                    Ok(ToDoItem {
                        id: 1,
                        title: "coding".to_string(),
                        status: "PENDING".to_string(),
                    })
                }
            }
        }
        let result = delete::<ReturnOneMock>("coding", 1).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_err() {

        struct ReturnErrorMock;

        impl DeleteOne for ReturnErrorMock {
            fn delete_one(id: String, user_id: i32) -> impl Future<Output = DeleteOneResponse> + Send {
                if id != "coding" {
                    panic!("Invalid title");
                }
                if user_id != 1 {
                    panic!("Invalid user_id");
                }
                async {
                    Err(NanoServiceError::new("Some Error".to_string(), glue::errors::NanoServiceErrorStatus::NotFound))
                }
            }
        }
        let result = delete::<ReturnErrorMock>("coding", 1).await;
        match result {
            Ok(_) => panic!("Expected error"),
            Err(e) => {
                assert_eq!(e.status, glue::errors::NanoServiceErrorStatus::NotFound);
                assert_eq!(e.message, "Some Error");
            }
        }
    }

}
