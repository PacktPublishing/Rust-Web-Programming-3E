use to_do_dal::json_file::get_all as get_all_handle;
use crate::structs::{
    ToDoItem,
    AllToDoItems
};
use glue::errors::{NanoServiceError, NanoServiceErrorStatus};


/// Gets all tasks from the JSON file.
/// 
/// # Returns
/// An `AllToDoItems` struct.
pub async fn get_all() -> Result<AllToDoItems, NanoServiceError> {
    Ok(AllToDoItems::from_hashmap(
        get_all_handle::<ToDoItem>()?
    ))
}


/// Gets a task by its name from the JSON file.
/// 
/// # Arguments
/// * `name` - The name of the task to get.
/// 
/// # Returns
/// A `ToDoItem` struct.
pub async fn get_by_name(name: &str) -> Result<ToDoItem, NanoServiceError> {
    Ok(get_all_handle::<ToDoItem>()?
        .remove(name)
        .ok_or(
            NanoServiceError::new(
                format!("Item with name {} not found", name),
                NanoServiceErrorStatus::NotFound
            )
        )?
    )
}

