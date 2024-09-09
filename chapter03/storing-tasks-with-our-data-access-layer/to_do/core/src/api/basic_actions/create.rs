//! File: to_do/core/src/api/basic_actions/create.rs
use std::fmt;
use crate::structs::{
    done::Done,
    pending::Pending,
};
use crate::enums::TaskStatus;


/// This enum is a wrapper for the different item types supported for the create API.
/// 
/// # Variants
/// - `Pending` - Represents a pending item.
/// - `Done` - Represents a done item.
pub enum ItemTypes {
    Pending(Pending),
    Done(Done),
}


impl fmt::Display for ItemTypes {

    /// This function formats the `ItemTypes` enum.
    /// 
    /// # Arguments
    /// - `f` - A mutable reference to a `fmt::Formatter` instance.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ItemTypes::Pending(pending) => write!(
                f, "Pending: {}", 
                pending.super_struct.title
            ),
            ItemTypes::Done(done) => write!(
                f, "Done: {}", 
                done.super_struct.title
            ),
        }
    }
}


/// This function creates a new item based on the title and status provided.
/// 
/// # Notes
/// This is the external interface for the create item API.
/// 
/// # Arguments
/// - `title` - The title of the item to be created.
/// - `status` - The status of the item to be created.
/// 
/// # Returns
/// An `ItemTypes` enum representing the item created.
pub fn create(title: &str, status: TaskStatus) -> ItemTypes {
    match status {
        TaskStatus::PENDING => {
            ItemTypes::Pending(Pending::new(&title))
        },
        TaskStatus::DONE => {
            ItemTypes::Done(Done::new(&title))
        },
    }
}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_fmt() {
        let pending = Pending::new("laundry");
        let done = Done::new("coding");
        let pending_item = ItemTypes::Pending(pending);
        let done_item = ItemTypes::Done(done);
        assert_eq!(format!("{}", pending_item), "Pending: laundry");
        assert_eq!(format!("{}", done_item), "Done: coding");
    }

    #[test]
    fn test_create() {
        let pending = create("laundry", TaskStatus::PENDING);
        let done = create("coding", TaskStatus::DONE);
        match pending {
            ItemTypes::Pending(pending) => {
                assert_eq!(pending.super_struct.title, "laundry");
            },
            _ => panic!("Expected a pending item"),
        }
        match done {
            ItemTypes::Done(done) => {
                assert_eq!(done.super_struct.title, "coding");
            },
            _ => panic!("Expected a done item"),
        }
    }
}
