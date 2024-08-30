use dal::to_do_items::schema::AllToDOItems;
use dal::to_do_items::transactions::get::GetAll;
use glue::errors::NanoServiceError;


pub async fn get_all<T: GetAll>(user_id: i32) -> Result<AllToDOItems, NanoServiceError> {
    let all_items = T::get_all(user_id).await?;
    AllToDOItems::from_vec(all_items)
}
