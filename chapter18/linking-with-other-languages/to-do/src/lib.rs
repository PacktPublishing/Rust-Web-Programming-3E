use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
pub struct Todo {
    pub title: String,
    pub status: String,
}
