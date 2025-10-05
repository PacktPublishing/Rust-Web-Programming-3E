use std::fmt;
use glue::errors::NanoServiceError;
use serde::{Serialize, Deserialize};
use super::enums::TaskStatus;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewToDoItem {
    pub title: String,
    pub status: TaskStatus
}


impl fmt::Display for NewToDoItem {
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


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "sqlx-postgres", derive(sqlx::FromRow))]
pub struct ToDoItem {
    pub id: i32,
    pub title: String,
    pub status: String
}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AllToDOItems {
    pub pending: Vec<ToDoItem>,
    pub done: Vec<ToDoItem>
}


impl AllToDOItems {

    pub fn from_vec(all_items: Vec<ToDoItem>) -> Result<AllToDOItems, NanoServiceError> {
        let mut pending = Vec::new();
        let mut done = Vec::new();

        for item in all_items {
            match TaskStatus::from_string(&item.status)? {
                TaskStatus::PENDING => pending.push(item),
                TaskStatus::DONE => done.push(item)
            }
        }
        Ok(AllToDOItems {
            pending,
            done
        })
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "sqlx-postgres", derive(sqlx::FromRow))]
pub struct UserConnection {
    pub user_id: i32,
    pub to_do_id: i32
}
