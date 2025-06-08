use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewUser {
    pub email: String,
    pub password: String,
    pub unique_id: String
}


#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub unique_id: String
}


#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TrimmedUser {
    pub id: i32,
    pub email: String,
    pub unique_id: String
}

impl From<User> for TrimmedUser {
    fn from(user: User) -> Self {
        TrimmedUser {
            id: user.id,
            email: user.email,
            unique_id: user.unique_id
        }
    }
}
