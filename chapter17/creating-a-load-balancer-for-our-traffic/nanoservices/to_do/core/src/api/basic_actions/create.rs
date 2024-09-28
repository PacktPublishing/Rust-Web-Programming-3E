use dal::to_do_items::schema::{NewToDoItem, ToDoItem};
use dal::to_do_items::transactions::create::SaveOne;
use glue::errors::NanoServiceError;


pub async fn create<T: SaveOne>(item: NewToDoItem, user_id: i32) -> Result<ToDoItem, NanoServiceError> {
    // maybe get the limit of items allowed for user
    T::save_one(item, user_id).await
}

#[cfg(test)]
mod tests {

    use super::*;
    use dal::to_do_items::schema::ToDoItem;
    use dal::to_do_items::transactions::create::SaveOneResponse;
    use dal::to_do_items::enums::TaskStatus;
    use std::future::Future;

    #[tokio::test]
    async fn test_create_ok() {

        struct ReturnOneMock;

        impl SaveOne for ReturnOneMock {
            fn save_one(item: NewToDoItem, user_id: i32) 
            -> impl Future<Output = SaveOneResponse> + Send {
                if item.title != "coding" {
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
        let item = NewToDoItem {
            title: "coding".to_string(),
            status: TaskStatus::PENDING,
        };
        let result = create::<ReturnOneMock>(item, 1).await;
        assert!(result.is_ok());
    }
}
