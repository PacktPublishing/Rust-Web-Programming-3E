use serde::{de::DeserializeOwned, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs::{OpenOptions, File};
use std::io::{Read, Write};


fn get_handle() -> Result<File, String> {
    let file_path = env::var("JSON_STORE_PATH").unwrap_or("./tasks.json".to_string());
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&file_path)
        .map_err(|e| format!("Error opening file: {}", e))?;
    Ok(file)
}


pub fn get_all<T: DeserializeOwned>() -> Result<HashMap<String, T>, String> {
    let mut file = get_handle()?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e| format!("Error reading file: {}", e))?;
    let tasks: HashMap<String, T> = serde_json::from_str(
        &contents).map_err(|e| format!("Error parsing JSON: {}", e)
    )?;
    Ok(tasks)
}


pub fn save_all<T: Serialize>(tasks: &HashMap<String, T>) -> Result<(), String> {
    let mut file = get_handle()?;
    let json = serde_json::to_string_pretty(tasks).map_err(|e| format!("Error serializing JSON: {}", e))?;
    file.write_all(json.as_bytes()).map_err(|e| format!("Error writing file: {}", e))?;
    Ok(())
}


pub fn get_one<T: DeserializeOwned + Clone>(id: &str) -> Result<T, String> {
    let tasks = get_all::<T>()?;
    match tasks.get(id) {
        Some(t) => Ok(t.clone()),
        None => Err(format!("Task with id {} not found", id))
    }
}


pub fn save_one<T>(id: &str, task: &T) -> Result<(), String>
where
    T: Serialize + DeserializeOwned + Clone,
{
    let mut tasks = get_all::<T>().unwrap_or_else(|_| HashMap::new());
    tasks.insert(id.to_string(), task.clone());
    save_all(&tasks)
}


pub fn delete_one<T>(id: &str) -> Result<(), String>
where
    T: Serialize + DeserializeOwned + Clone,
{
    let mut tasks = get_all::<T>().unwrap_or(HashMap::new());
    tasks.remove(id);
    save_all(&tasks)
}

