use to_do_dal::to_do_items::transactions::delete::DeleteOne;
use glue::errors::NanoServiceError;


/// Deletes a to-do item.
/// 
/// # Arguments
/// - `id` - a string slice that specifies the id of the item to delete.
/// - `user_id` - the ID of the user associated with the item
/// 
/// # Returns
/// The deleted item.
pub async fn delete<T: DeleteOne>(id: &str, user_id: i32) 
    -> Result<(), NanoServiceError> {
    let _ = T::delete_one(id.to_string(), user_id).await?;
    Ok(())
}

