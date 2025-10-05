use dal::to_do_items::schema::ToDoItem;
use glue::errors::NanoServiceError;
use dal::to_do_items::transactions::update::UpdateOne;


pub async fn update<T: UpdateOne>(item: ToDoItem, user_id: i32) -> Result<(), NanoServiceError> {
    let _ = T::update_one(item, user_id).await?;
    Ok(())
}
