use to_do_dal::json_file::get_all as get_all_handle;
use crate::structs::{
    ToDoItem,
    AllToDoItems
};


/// Gets all tasks from the JSON file.
/// 
/// # Returns
/// An `AllToDoItems` struct.
pub async fn get_all() -> Result<AllToDoItems, String> {
    Ok(AllToDoItems::from_hashmap(
        get_all_handle::<ToDoItem>()?
    ))
}
