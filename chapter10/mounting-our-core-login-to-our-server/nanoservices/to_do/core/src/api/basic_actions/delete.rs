use to_do_dal::to_do_items::transactions::delete::DeleteOne;
use glue::errors::NanoServiceError;


/// Deletes a to-do item.
/// 
/// # Arguments
/// - `id` - a string slice that specifies the id of the item to delete.
/// 
/// # Returns
/// The deleted item.
pub async fn delete<T: DeleteOne>(id: &str) -> Result<(), NanoServiceError> {
    let _ = T::delete_one(id.to_string()).await?;
    Ok(())
}
