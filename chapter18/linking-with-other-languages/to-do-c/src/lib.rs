use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use to_do::Todo;


#[no_mangle]
pub extern "C" fn create_todo_json(title: *const c_char) -> *mut c_char {
    // Convert C string to Rust String
    let c_str = unsafe { CStr::from_ptr(title) };
    let title = c_str.to_str().unwrap_or_default().to_string();

    // Create a new Todo
    let todo = Todo {
        title,
        status: "PENDING".to_string(),
    };

    // Convert Todo struct to JSON
    let json = serde_json::to_string(&todo).unwrap_or_default();

    // Return the JSON as a C string
    CString::new(json).unwrap().into_raw()
}

/// Mark a Todo as done, modifying the JSON and returning the updated JSON string
#[no_mangle]
pub extern "C" fn mark_done_json(todo_json: *const c_char) -> *mut c_char {
    // Convert the input C string (JSON) to a Rust string
    let c_str = unsafe { CStr::from_ptr(todo_json) };
    let json = c_str.to_str().unwrap_or_default();

    // Deserialize JSON string into a Todo struct
    let mut todo: Todo = serde_json::from_str(json).unwrap();

    // Mark the Todo as done
    todo.status = "DONE".to_string();

    // Convert the updated Todo struct back to a JSON string
    let updated_json = serde_json::to_string(&todo).unwrap_or_default();

    // Return the updated JSON as a C string
    CString::new(updated_json).unwrap().into_raw()
}
