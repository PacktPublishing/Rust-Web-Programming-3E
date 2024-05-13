use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct SomeDataStruct {
    pub name: String,
    pub names: Vec<String>,
}


#[repr(C)]
pub struct ResultPointer {
    pub ptr: i32,
    pub len: i32
}
