use serde::{Serialize, Deserialize};


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum KvMessage {
    Get(Vec<u8>),
    Put((Vec<u8>, Vec<u8>)),
    Del(Vec<u8>),
    Success(bool),
    ReturnValue(Option<Vec<u8>>),
    Error(String),
}


impl KvMessage {

    pub fn package(&self) -> Vec<u8> {
        let data = bincode::serialize(&self).unwrap();
        let len = data.len() as u32;
        let len_buf = len.to_be_bytes();
        let mut buf = Vec::new();
        buf.extend_from_slice(&mut len_buf.to_vec());
        buf.extend_from_slice(&mut data.to_vec());
        buf
    }

}
