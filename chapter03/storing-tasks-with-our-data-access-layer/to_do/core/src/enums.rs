//! File: to_do/core/src/enums.rs
use std::fmt;


/// Enum for defining the status of a task
/// 
/// # Variants
/// * `DONE` - Task is completed
/// * `PENDING` - Task is yet to be completed
#[derive(Debug, PartialEq)]
pub enum TaskStatus {
    DONE,
    PENDING
}

impl fmt::Display for TaskStatus {

    /// Writes the the standard output such as the terminal.
    /// 
    /// # Arguments
    /// * `f` - A mutable reference to the formatter
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            &Self::DONE => {write!(f, "DONE")},
            &Self::PENDING => {write!(f, "PENDING")}
        }
    }
}


// Below are unit tests to assert what the Display trait implementation returns
// We cover unit tests in chapter 14 of the book but just running this code by
// using the cargo test command will show you that the tests pass.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_status() {
        assert_eq!(TaskStatus::DONE.to_string(), "DONE");
        assert_eq!(TaskStatus::PENDING.to_string(), "PENDING");

        let done_status = format!("{}", TaskStatus::DONE);
        let pending_status = format!("{}", TaskStatus::PENDING);

        assert_eq!(done_status, "DONE");
        assert_eq!(pending_status, "PENDING");

        let done_status = TaskStatus::DONE.to_string();
        let pending_status = TaskStatus::PENDING.to_string();

        assert_eq!(done_status, "DONE");
        assert_eq!(pending_status, "PENDING");
    }
}
