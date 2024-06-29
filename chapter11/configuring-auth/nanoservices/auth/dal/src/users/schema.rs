use glue::errors::{NanoServiceError, NanoServiceErrorStatus};
use serde::{Serialize, Deserialize};
use argon2::{
    Argon2, 
    PasswordHasher, 
    PasswordVerifier, 
    password_hash::{
        SaltString, 
        PasswordHash
    }
};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewUser {
    pub email: String,
    pub password: String,
    pub unique_id: String
}


impl NewUser {

    pub fn new(email: String, password: String) 
        -> Result<NewUser, NanoServiceError> {
        let unique_id = uuid::Uuid::new_v4().to_string();
        let salt = SaltString::generate(&mut rand::thread_rng());
        let argon2_hasher = Argon2::default();
        let hash = argon2_hasher.hash_password(
            password.as_bytes(), 
            &salt
        ).map_err(|e|{
            NanoServiceError::new(
                format!("Failed to hash password: {}", e),
                NanoServiceErrorStatus::Unknown
            )
        })?.to_string();
        Ok(NewUser {
            email,
            password: hash,
            unique_id
        })
    }

}


#[derive(Deserialize, Debug, Clone, PartialEq, sqlx::FromRow, Serialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub unique_id: String
}

impl User {
    
    pub fn verify_password(&self, password: String) 
    -> Result<bool, NanoServiceError> {
        let argon2_hasher = Argon2::default();
        let parsed_hash = PasswordHash::new(
            &self.password
        ).map_err(|e|{
            NanoServiceError::new(
                format!("Failed to parse password hash: {}", e),
                NanoServiceErrorStatus::Unknown
            )
        })?;
        let is_valid = argon2_hasher.verify_password(
            password.as_bytes(), 
            &parsed_hash
        ).is_ok();
        Ok(is_valid)
    }
    
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


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_verify() {
        let password = "password".to_string();
        let email = "test@gmail.com".to_string();

        let user = User {
            id: 1,
            email: email.clone(),
            password: NewUser::new(email.clone(), password.clone()).unwrap().password,
            unique_id: "unique_id".to_string()
        };
        assert_eq!(user.verify_password(password).unwrap(), true);
    }
}
