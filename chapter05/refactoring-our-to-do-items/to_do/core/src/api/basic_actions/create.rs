//! File: to_do/core/src/api/basic_actions/create.rs
use crate::enums::TaskStatus;
use crate::structs::ToDoItem;

#[cfg(feature = "json-file-storage")]
use to_do_dal::json_file::save_one;


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
pub fn create(title: &str, status: TaskStatus) 
    -> Result<ToDoItem, String> {
    let item = ToDoItem {
        title: title.to_string(),
        status
    };
    let _ = save_one(&title.to_string(), &item)?;
    Ok(item)
}
