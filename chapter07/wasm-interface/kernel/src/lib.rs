use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct SomeDataStruct {
    pub name: String,
    pub names: Vec<String>,
}
