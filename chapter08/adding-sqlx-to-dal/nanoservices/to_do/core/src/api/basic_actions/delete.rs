use dal::json_file::delete_one;
use dal::to_do_items::schema::ToDoItem;
use glue::errors::NanoServiceError;


pub async fn delete(id: &str) -> Result<(), NanoServiceError> {
    delete_one::<ToDoItem>(id)
}
