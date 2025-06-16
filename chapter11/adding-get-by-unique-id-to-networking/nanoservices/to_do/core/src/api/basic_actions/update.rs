use to_do_dal::to_do_items::schema::ToDoItem;
use glue::errors::NanoServiceError;
use to_do_dal::to_do_items::transactions::update::UpdateOne;


/// Update an item in the to do list.
/// 
/// # Arguments
/// * `item` - The item to update.
pub async fn update<T: UpdateOne>(item: ToDoItem) -> Result<(), NanoServiceError> {
    let _ = T::update_one(item).await?;
    Ok(())
}
