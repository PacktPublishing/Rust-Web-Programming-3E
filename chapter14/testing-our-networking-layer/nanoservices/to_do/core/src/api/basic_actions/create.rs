//! File: to_do/core/src/api/basic_actions/create.rs
use to_do_dal::to_do_items::schema::{NewToDoItem, ToDoItem};
use to_do_dal::to_do_items::transactions::create::SaveOne;
use glue::errors::NanoServiceError;


/// This function creates a new item based on the title and status provided.
/// 
/// # Notes
/// This is the external interface for the create item API.
/// 
/// # Arguments
/// - `item` - The item to be created
/// - `status` - The ID of the user associated with the item
/// 
/// # Returns
/// An `ItemTypes` enum representing the item created.
pub async fn create<T: SaveOne>(item: NewToDoItem, user_id: i32) 
    -> Result<ToDoItem, NanoServiceError> {
    let created_item = T::save_one(item, user_id).await?;
    Ok(created_item)
}

