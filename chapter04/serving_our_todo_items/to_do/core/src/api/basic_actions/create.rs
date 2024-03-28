use crate::enums::TaskStatus;
use crate::structs::ToDoItem;
use dal::json_file::save_one;


pub fn create(title: &str, status: TaskStatus) -> Result<ToDoItem, String> {
    let item = ToDoItem {
        title: title.to_string(),
        status
    };
    let _ = save_one(&title.to_string(), &item)?;
    Ok(item)
}
