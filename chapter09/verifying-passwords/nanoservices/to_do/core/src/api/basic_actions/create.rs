use dal::to_do_items::schema::{NewToDoItem, ToDoItem};
use dal::to_do_items::transactions::create::SaveOne;
use glue::errors::NanoServiceError;


pub async fn create<T: SaveOne>(item: NewToDoItem) -> Result<ToDoItem, NanoServiceError> {
    println!("Creating item {:?}", item);
    let created_item = T::save_one(item).await?;
    Ok(created_item)
}
