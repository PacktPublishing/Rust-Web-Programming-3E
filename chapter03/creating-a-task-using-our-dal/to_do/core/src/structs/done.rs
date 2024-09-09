//! File: core/src/structs/done.rs
use super::base::Base;
use super::super::enums::TaskStatus;


/// Struct for done tasks.
/// 
/// # Fields
/// * `super_struct` - The base struct for the task pointing to the data that can be stored.
pub struct Done {
    pub super_struct: Base
}

impl Done {

    /// The constructor for the done task.
    /// 
    /// # Arguments
    /// * `input_title` - The title of the task
    /// 
    /// # Returns
    /// A new instance of the done task
    pub fn new(input_title: &str) -> Self {
        let base = Base {
            title: input_title.to_string(),
            status: TaskStatus::DONE
        };
        return Done{super_struct: base}
    }
}


// Below are unit tests to assert what the constructor implementation returns
// We cover unit tests in chapter 14 of the book but just running this code by
// using the cargo test command will show you that the tests pass.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_done_task() {
        let done_task = Done::new("Test Task");
        assert_eq!(done_task.super_struct.title, "Test Task");
        assert_eq!(done_task.super_struct.status, TaskStatus::DONE);
    }
}