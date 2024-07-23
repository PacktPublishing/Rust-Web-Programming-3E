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


fn get_handle() -> Result<File, NanoServiceError> {
    let file_path = env::var("JSON_STORE_PATH").unwrap_or("./tasks.json".to_string());
    let file = safe_eject!(OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&file_path),
        NanoServiceErrorStatus::Unknown,
        "Error reading JSON file"
    )?;
    Ok(file)
}

fn get_write_handle() -> Result<File, NanoServiceError> {
    let file_path = env::var("JSON_STORE_PATH").unwrap_or("./tasks.json".to_string());
    let file = safe_eject!(OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&file_path),
        NanoServiceErrorStatus::Unknown,
        "Error reading JSON file (write handle)"
    )?;
    Ok(file)
}


pub fn get_all<T: DeserializeOwned>() -> Result<HashMap<String, T>, NanoServiceError> {
    let mut file = get_handle()?;
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


pub fn save_all<T: Serialize + std::fmt::Debug>(tasks: &HashMap<String, T>) -> Result<(), NanoServiceError> {
    let mut file = get_write_handle()?;
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


pub fn get_one<T: DeserializeOwned + Clone>(id: &str) -> Result<T, NanoServiceError> {
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


pub fn save_one<T>(id: &str, task: &T) -> Result<(), NanoServiceError>
where
    T: Serialize + DeserializeOwned + Clone + std::fmt::Debug,
{
    let mut tasks = get_all::<T>().unwrap_or_else(|_| HashMap::new());
    tasks.insert(id.to_string(), task.clone());
    save_all(&tasks)
}


pub fn delete_one<T>(id: &str) -> Result<(), NanoServiceError>
where
    T: Serialize + DeserializeOwned + Clone + std::fmt::Debug,
{
    let mut tasks = get_all::<T>().unwrap_or(HashMap::new());
    match tasks.remove(id) {
        Some(_) => {
            // save_all(&tasks)?;
            // Ok(())
        },
        None => return Err(
            NanoServiceError::new(
                format!("Task with title {} not found", id), 
                NanoServiceErrorStatus::NotFound
            )
        )
    }
    save_all(&tasks)
}

