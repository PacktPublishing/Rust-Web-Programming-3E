use dal::json_file::get_all as get_all_handle;
use crate::structs::{
    ToDoItem,
    AllToDOItems
};
use glue::errors::{
    NanoServiceError,
    NanoServiceErrorStatus
};


pub async fn get_all() -> Result<AllToDOItems, NanoServiceError> {
    Ok(AllToDOItems::from_hashmap(
        get_all_handle::<ToDoItem>()?
    ))
}


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
