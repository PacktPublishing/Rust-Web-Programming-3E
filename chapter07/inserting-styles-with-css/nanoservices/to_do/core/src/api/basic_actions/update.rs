use to_do_dal::json_file::{
    get_all as get_all_handle,
    save_all
};
use crate::structs::ToDoItem;
use glue::errors::{
    NanoServiceError,
    NanoServiceErrorStatus
};


/// Update an item in the to do list.
/// 
/// # Arguments
/// * `item` - The item to update.
pub async fn update(item: ToDoItem) -> Result<(), NanoServiceError> {
    let mut all_items = get_all_handle::<ToDoItem>()?;
    if !all_items.contains_key(&item.title) {
        return Err(NanoServiceError::new(
            format!("Item with name {} not found", item.title),
            NanoServiceErrorStatus::NotFound
        ));
    }
    all_items.insert(item.title.clone(), item);
    save_all(&all_items)?;
    Ok(())
}
