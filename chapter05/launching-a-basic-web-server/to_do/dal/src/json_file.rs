//! File: to_do/dal/src/json_file.rs
use serde::{de::DeserializeOwned, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs::{OpenOptions, File};
use std::io::{Read, Write};


/// Opens a file.
/// 
/// # Arguments
/// - `path` - An optional string slice that specifies the path to the file.
/// 
/// # Returns
/// a file handle to perform read/write operations with.
fn get_handle(path: Option<&str>) -> Result<File, String> {
    let path = match path {
        Some(p) => p,
        None => {
            &env::var("JSON_STORE_PATH").unwrap_or(
                "./tasks.json".to_string()
            )
        }
    };
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&path)
        .map_err(
            |e| format!("Error opening file: {}", e)
        )?;
    Ok(file)
}


/// Gets all the tasks from the JSON file.
/// 
/// # Returns
/// a hashmap of tasks.
pub fn get_all<T: DeserializeOwned>() 
    -> Result<HashMap<String, T>, String> {
    let mut file = get_handle(None)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(
        |e| format!("Error reading file: {}", e)
    )?;
    let tasks: HashMap<String, T> = serde_json::from_str(
        &contents).map_err(|e| format!("Error parsing JSON: {}", e)
    )?;
    Ok(tasks)
}


/// Saves all the tasks to the JSON file.
/// 
/// # Arguments
/// - `tasks` - a hashmap of tasks.
pub fn save_all<T: Serialize>(tasks: &HashMap<String, T>) 
    -> Result<(), String> {
    let mut file = get_handle(None)?;
    let json = serde_json::to_string_pretty(tasks).map_err(
        |e| format!("Error serializing JSON: {}", e)
    )?;
    file.write_all(json.as_bytes()).map_err(
        |e| format!("Error writing file: {}", e)
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
    -> Result<T, String> {
    let tasks = get_all::<T>()?;
    match tasks.get(id) {
        Some(t) => Ok(t.clone()),
        None => Err(format!("Task with id {} not found", id))
    }
}


/// Saves a task to the JSON file.
/// 
/// # Arguments
/// - `id` - a string slice that specifies the id of the task.
/// - `task` - a reference to the task to save.
pub fn save_one<T>(id: &str, task: &T) 
-> Result<(), String>
where
    T: Serialize + DeserializeOwned + Clone,
{
    let mut tasks = get_all::<T>().unwrap_or_else(
        |_| HashMap::new()
    );
    tasks.insert(id.to_string(), task.clone());
    save_all(&tasks)
}


/// Deletes a task from the JSON file.
/// 
/// # Arguments
/// - `id` - a string slice that specifies the id of the task to delete.
pub fn delete_one<T>(id: &str) -> Result<(), String>
where
    T: Serialize + DeserializeOwned + Clone,
{
    let mut tasks = get_all::<T>().unwrap_or(
        HashMap::new()
    );
    tasks.remove(id);
    save_all(&tasks)
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