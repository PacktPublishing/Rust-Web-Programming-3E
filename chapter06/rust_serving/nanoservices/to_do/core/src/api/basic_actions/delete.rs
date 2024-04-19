use dal::json_file::delete_one;
use crate::structs::ToDoItem;
use glue::errors::NanoServiceError;


pub async fn delete(id: &str) -> Result<(), NanoServiceError> {
    delete_one::<ToDoItem>(id)
}
