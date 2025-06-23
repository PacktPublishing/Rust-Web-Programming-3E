//! File: to_do/core/src/enums.rs
use std::fmt;
use glue::errors::{NanoServiceError, NanoServiceErrorStatus};
use serde::{Serialize, Deserialize};


/// Enum for defining the status of a task
/// 
/// # Variants
/// * `DONE` - Task is completed
/// * `PENDING` - Task is yet to be completed
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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


impl TaskStatus {

    /// Converts a string to a TaskStatus enum.
    /// 
    /// # Arguments
    /// * `status` - A reference to a string that is to be converted to a TaskStatus enum
    /// 
    /// # Returns
    /// the constructe TaskStatus enum if the string is valid, otherwise an error message
    pub fn from_string(status: &String) 
        -> Result<TaskStatus, NanoServiceError> {
        match status.to_uppercase().as_str() {
            "DONE" => Ok(TaskStatus::DONE),
            "PENDING" => Ok(TaskStatus::PENDING),
            _ => Err(
                NanoServiceError::new(
                    "Invalid status".to_string(), 
                    NanoServiceErrorStatus::BadRequest
                )
            )
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

    #[test]
    fn test_task_status_from_string() {
        let done_status = "DONE".to_string();
        let pending_status = "PENDING".to_string();
        let invalid_status = "INVALID".to_string();

        assert_eq!(TaskStatus::from_string(&done_status).unwrap(), TaskStatus::DONE);
        assert_eq!(TaskStatus::from_string(&pending_status).unwrap(), TaskStatus::PENDING);
        assert_eq!(TaskStatus::from_string(&invalid_status).is_err(), true);
    }
}
