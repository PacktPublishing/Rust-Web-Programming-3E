// File: kernel/src/lib.rs
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    pub memory: Vec<u8>,
    pub contract: Vec<u8>,
}
