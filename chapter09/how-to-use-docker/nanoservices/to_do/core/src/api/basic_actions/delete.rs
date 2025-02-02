use to_do_dal::json_file::delete_one;
use crate::structs::ToDoItem;
use glue::errors::NanoServiceError;

/// Deletes a to-do item.
/// 
/// # Arguments
/// - `id` - a string slice that specifies the id of the item to delete.
/// 
/// # Returns
/// The deleted item.
pub async fn delete(id: &str) -> Result<ToDoItem, NanoServiceError> {
    delete_one::<ToDoItem>(id)
}
