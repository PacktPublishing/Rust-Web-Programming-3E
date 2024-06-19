use dal::to_do_items::schema::ToDoItem;
use glue::errors::NanoServiceError;
use dal::to_do_items::transactions::update::UpdateOne;


pub async fn update<T: UpdateOne>(item: ToDoItem) -> Result<(), NanoServiceError> {
    let _ = T::update_one(item).await?;
    Ok(())
}
