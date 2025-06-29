use std::fmt;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
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


/// A struct representing all to-do items.
/// 
/// # Fields
/// - `pending`: A vector of pending to-do items.
/// - `done`: A vector of done to-do items.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AllToDoItems {
    pub pending: Vec<ToDoItem>,
    pub done: Vec<ToDoItem>
}


impl AllToDoItems {

    /// Converts a `HashMap` of to-do items to an `AllToDoItems` struct.
    /// 
    /// # Arguments
    /// - `all_items`: A `HashMap` of to-do items.
    /// 
    /// # Returns
    /// An `AllToDoItems` struct.
    pub fn from_hashmap(all_items: HashMap<String, ToDoItem>) 
        -> AllToDoItems {
        let mut pending = Vec::new();
        let mut done = Vec::new();

        for (_, item) in all_items {
            match item.status {
                TaskStatus::PENDING => pending.push(item),
                TaskStatus::DONE => done.push(item)
            }
        }
        AllToDoItems {
            pending,
            done
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

    #[test]
    fn test_from_hashmap() {
        let mut all_items = HashMap::new();
        all_items.insert(
            "1".to_string(),
            ToDoItem {
                title: "Test".to_string(),
                status: TaskStatus::PENDING
            }
        );
        all_items.insert(
            "2".to_string(),
            ToDoItem {
                title: "Test".to_string(),
                status: TaskStatus::DONE
            }
        );
        let all_to_do_items = AllToDoItems::from_hashmap(all_items);
        assert_eq!(
            all_to_do_items.pending.len(),
            1
        );
        assert_eq!(
            all_to_do_items.done.len(),
            1
        );
    }

}