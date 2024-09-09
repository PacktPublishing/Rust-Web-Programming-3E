use std::fmt;
use serde::{Serialize, Deserialize};
use crate::enums::TaskStatus;


/// A struct representing a to-do item.
/// 
/// # Fields
/// - `title`: The title of the to-do item.
/// - `status`: The status of the to-do item.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToDoItem {
    pub title: String,
    pub status: TaskStatus
}


impl fmt::Display for ToDoItem {

    /// Formats the `ToDoItem` struct.
    /// 
    /// # Arguments
    /// - `f`: A mutable reference to a `fmt::Formatter`.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.status {
            TaskStatus::PENDING => write!(
                f, "Pending: {}", 
                self.title
            ),
            TaskStatus::DONE => write!(
                f, "Done: {}", 
                self.title
            ),
        }
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_display() {
        let to_do_item = ToDoItem {
            title: "Test".to_string(),
            status: TaskStatus::PENDING
        };
        assert_eq!(
            format!("{}", to_do_item),
            "Pending: Test"
        );
    }

}