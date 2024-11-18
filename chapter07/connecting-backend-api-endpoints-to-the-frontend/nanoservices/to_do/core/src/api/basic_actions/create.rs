//! File: to_do/core/src/api/basic_actions/create.rs
use crate::structs::ToDoItem;

#[cfg(feature = "json-file-storage")]
use to_do_dal::json_file::save_one;
use glue::errors::NanoServiceError;


/// This function creates a new item based on the title and status provided.
/// 
/// # Notes
/// This is the external interface for the create item API.
/// 
/// # Arguments
/// - `title` - The title of the item to be created.
/// - `status` - The status of the item to be created.
/// 
/// # Returns
/// An `ItemTypes` enum representing the item created.
pub async fn create(item: ToDoItem) 
    -> Result<ToDoItem, NanoServiceError> {
    let _ = save_one(&item.title.to_string(), &item)?;
    Ok(item)
}

