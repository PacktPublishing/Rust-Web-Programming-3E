//! File: core/src/structs/pending.rs
use super::base::Base;
use super::super::enums::TaskStatus;


/// Struct for pending tasks.
/// 
/// # Fields
/// * `super_struct` - The base struct for the task pointing to the data that can be stored.
pub struct Pending {
    pub super_struct: Base
}
impl Pending {

    /// The constructor for the pending task.
    /// 
    /// # Arguments
    /// * `input_title` - The title of the task
    /// 
    /// # Returns
    /// A new instance of the pending task
    pub fn new(input_title: &str) -> Self {
        let base = Base{
            title: input_title.to_string(),
            status: TaskStatus::PENDING
        };
        return Pending{super_struct: base}
    }
}


// Below are unit tests to assert what the constructor implementation returns
// We cover unit tests in chapter 14 of the book but just running this code by
// using the cargo test command will show you that the tests pass.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pending_task() {
        let pending_task = Pending::new("Test Task");
        assert_eq!(pending_task.super_struct.title, "Test Task");
        assert_eq!(pending_task.super_struct.status, TaskStatus::PENDING);
    }
}
