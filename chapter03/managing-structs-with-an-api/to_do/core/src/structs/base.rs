//! File: to_do/core/src/structs/base.rs
use super::super::enums::TaskStatus;


/// Base struct for the task.
/// 
/// # Notes
/// This base struct is referenced by other to-do item structs representing the
/// core data that would be stored. The other structs referencing this struct act
/// as interfaces for the data held in this struct.
/// 
/// # Fields
/// * `title` - The title of the task
/// * `status` - The status of the task
pub struct Base {
    pub title: String,
    pub status: TaskStatus
}
