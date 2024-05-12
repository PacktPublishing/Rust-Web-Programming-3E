extern crate alloc;
use core::alloc::Layout;
use kernel::SomeDataStruct;
// cargo build --target wasm32-wasi --release


// for allocating memory
#[no_mangle]
pub unsafe extern "C" fn ns_malloc(size: u32, alignment: u32) -> *mut u8 {
    let layout = Layout::from_size_align_unchecked(size as usize, alignment as usize);
    alloc::alloc::alloc(layout)
}


// for deallocating memory
#[no_mangle]
pub unsafe extern "C" fn ns_free(ptr: *mut u8, size: u32, alignment: u32) {
    let layout = Layout::from_size_align_unchecked(size as usize, alignment as usize);
    alloc::alloc::dealloc(ptr, layout);
}


#[repr(C)]
pub struct OutcomePointer {
    ptr: i32,
    len: i32
}


#[no_mangle]
pub extern "C" fn entry_point(ptr: *const u8, len: usize) -> *const OutcomePointer {
    let bytes = unsafe { std::slice::from_raw_parts(ptr, len) };
    let mut data_struct: SomeDataStruct = bincode::deserialize(bytes).unwrap();
    data_struct.names.push("new name".to_string());

    let serialized_data = bincode::serialize(&data_struct).unwrap();
    let len = serialized_data.len();
    let out_ptr = serialized_data.leak().as_ptr();

    let result = Box::new(OutcomePointer{
        ptr: out_ptr as i32,
        len: len as i32
    });
    Box::into_raw(result) as *const OutcomePointer
}
