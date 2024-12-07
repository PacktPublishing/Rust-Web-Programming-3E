// File: compute-unit/src/lib.rs
extern crate alloc;
use std::thread::sleep;
use std::time::Duration;
use core::alloc::Layout;

// Declare the external host function
extern "C" {
    fn redis_get(key_ptr: i32, key_len: i32) -> i32;
}

// for allocating memory
#[no_mangle]
pub unsafe extern "C" fn ns_malloc(size: u32, alignment: u32) -> *mut u8 {
    let layout = Layout::from_size_align_unchecked(
        size as usize, 
        alignment as usize
    );
    alloc::alloc::alloc(layout)
}


// Import WebAssembly functions for interaction.
#[no_mangle]
pub extern "C" fn run_compute(ptr: i32, len: i32) -> i32 {
    let memory = unsafe {
        std::slice::from_raw_parts_mut(ptr as *mut u8, len as usize)
    };
    if memory.len() < 8 {
        panic!("Memory size too small!");
    }
    let highest_num = i32::from_le_bytes([
        memory[0],
        memory[1],
        memory[2],
        memory[3],
    ]);
    let mut current_num = i32::from_le_bytes([
        memory[4],
        memory[5],
        memory[6],
        memory[7],
    ]);
    // let value = unsafe { redis_get(key_ptr, key_len) };
    current_num += 1;
    sleep(Duration::from_secs(2));
    let new_bytes = current_num.to_le_bytes();
    for i in 0..4 {
        memory[4 + i] = new_bytes[i];
    }
    println!("{:?}", memory);
    if current_num >= highest_num {
        return 1
    }
    return 0
}
