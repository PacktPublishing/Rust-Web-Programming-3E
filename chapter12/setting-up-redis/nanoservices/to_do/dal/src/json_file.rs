//! File: to_do/dal/src/json_file.rs
use serde::{de::DeserializeOwned, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs::{OpenOptions, File};
use std::io::{Read, Write};

use glue::errors::{
    NanoServiceError,
    NanoServiceErrorStatus
};
use glue::safe_eject;


/// Opens a file.
/// 
/// # Arguments
/// - `path` - An optional string slice that specifies the path to the file.
/// 
/// # Returns
/// a file handle to perform read/write operations with.
fn get_handle(path: Option<&str>) -> Result<File, NanoServiceError>{
    let path = match path {
        Some(p) => p,
        None => {
            &env::var("JSON_STORE_PATH").unwrap_or(
                "./tasks.json".to_string()
            )
        }
    };
    safe_eject!(OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&path),
        NanoServiceErrorStatus::Unknown,
        "Error writing tasks to JSON to file"
    )
}


/// Opens a file for writing.
/// 
/// # Arguments
/// - `path` - An optional string slice that specifies the path to the file.
/// 
/// # Returns
/// a file handle to perform write operations with.
fn get_write_handle(path: Option<&str>) -> Result<File, NanoServiceError> {
    let path = match path {
        Some(p) => p,
        None => {
            &env::var("JSON_STORE_PATH").unwrap_or(
                "./tasks.json".to_string()
            )
        }
    };
    let file = safe_eject!(OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true) // ensures file is empty
        .open(&path),
        NanoServiceErrorStatus::Unknown,
        "Error reading JSON file (write handle)"
    )?;
    Ok(file)
}



/// Gets all the tasks from the JSON file.
/// 
/// # Returns
/// a hashmap of tasks.
pub fn get_all<T: DeserializeOwned>() 
    -> Result<HashMap<String, T>, NanoServiceError> {
    let mut file = get_handle(None)?;
    let mut contents = String::new();
    safe_eject!(
        file.read_to_string(&mut contents), 
        NanoServiceErrorStatus::Unknown,
        "Error reading JSON file to get all tasks"
    )?;
    let tasks: HashMap<String, T> = safe_eject!(
        serde_json::from_str(&contents), 
        NanoServiceErrorStatus::Unknown,
        "Error parsing JSON file"
    )?;
    Ok(tasks)
}


/// Saves all the tasks to the JSON file.
/// 
/// # Arguments
/// - `tasks` - a hashmap of tasks.
pub fn save_all<T: Serialize>(tasks: &HashMap<String, T>) -> Result<(), NanoServiceError> {
    let mut file = get_write_handle(None)?;
    let json = safe_eject!(
        serde_json::to_string_pretty(tasks), 
        NanoServiceErrorStatus::Unknown,
        "Error serializing JSON before saving tasks"
    )?;
    safe_eject!(
        file.write_all(json.as_bytes()), 
        NanoServiceErrorStatus::Unknown,
        "Error writing tasks to JSON to file"
    )?;
    Ok(())
}


/// Gets a task from the JSON file.
/// 
/// # Arguments
/// - `id` - a string slice that specifies the id of the task.
/// 
/// # Returns
/// a task.
pub fn get_one<T: DeserializeOwned + Clone>(id: &str) 
    -> Result<T, NanoServiceError> {
    let tasks = get_all::<T>()?;
    match tasks.get(id) {
        Some(t) => Ok(t.clone()),
        None => Err(
            NanoServiceError::new(
                format!("Task with id {} not found", id), 
                NanoServiceErrorStatus::Unknown
            )
        )
    }
}


/// Saves a task to the JSON file.
/// 
/// # Arguments
/// - `id` - a string slice that specifies the id of the task.
/// - `task` - a reference to the task to save.
pub fn save_one<T>(id: &str, task: &T) -> Result<(), NanoServiceError>
where
    T: Serialize + DeserializeOwned + Clone,
{
    let mut tasks = get_all::<T>().unwrap_or_else(|_| HashMap::new());
    tasks.insert(id.to_string(), task.clone());
    save_all(&tasks)
}


/// Deletes a task from the JSON file.
/// 
/// # Arguments
/// - `id` - a string slice that specifies the id of the task to delete.
pub fn delete_one<T>(id: &str) -> Result<T, NanoServiceError>
where
    T: Serialize + DeserializeOwned + Clone + std::fmt::Debug,
{
    let mut tasks = get_all::<T>().unwrap_or(HashMap::new());
    match tasks.remove(id) {
        Some(deleted_item) => {
            save_all(&tasks)?;
            Ok(deleted_item)
        },
        None => Err(
            NanoServiceError::new(
                format!("Task with title {} not found", id), 
                NanoServiceErrorStatus::NotFound
            )
        )
    }
}


#[cfg(feature = "json-file")]
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_handle() {
        let file = get_handle(None);
        assert!(file.is_ok());

        let file = get_handle(Some("./non_existent_file.json"));
        assert!(file.is_ok());
    }

    #[test]
    fn test_get_all() {

        save_one::<String>(&"1", &"Task 1".to_string()).unwrap();

        let tasks = get_all::<String>().unwrap();
        println!("{:?}", tasks);

        let tasks = get_all::<i32>();
    }
}