use dal::json_file::get_all as get_all_handle;
use crate::structs::{
    ToDoItem,
    AllToDOItems
};


pub async fn get_all() -> Result<AllToDOItems, String> {
    Ok(AllToDOItems::from_hashmap(
        get_all_handle::<ToDoItem>()?
    ))
}
