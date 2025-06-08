use to_do_dal::to_do_items::schema::AllToDOItems;
use to_do_dal::to_do_items::transactions::get::GetAll;
use glue::errors::NanoServiceError;


/// Gets all tasks from the JSON file.
/// 
/// # Returns
/// An `AllToDoItems` struct.
pub async fn get_all<T: GetAll>() -> Result<AllToDOItems, NanoServiceError> {
    let all_items = T::get_all().await?;
    AllToDOItems::from_vec(all_items)
}
