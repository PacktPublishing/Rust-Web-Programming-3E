use dal::to_do_items::transactions::delete::DeleteOne;
use glue::errors::NanoServiceError;


pub async fn delete<T: DeleteOne>(id: &str, user_id: i32) -> Result<(), NanoServiceError> {
    let _ = T::delete_one(id.to_string(), user_id).await?;
    Ok(())
}
