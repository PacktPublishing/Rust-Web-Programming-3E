use std::error::Error;
use redis::aio::{ConnectionLike, MultiplexedConnection};
use redis::Value;
use glue::errors::{NanoServiceError, NanoServiceErrorStatus};


async fn get_connnection(address: &str) -> Result<MultiplexedConnection, NanoServiceError> {
    let client = redis::Client::open(address).map_err(|e|{
        NanoServiceError::new(
            e.to_string(), 
            NanoServiceErrorStatus::Unknown
        )
    })?;
    let con = client.get_multiplexed_async_connection().await.map_err(|e|{
        NanoServiceError::new(
            e.to_string(), 
            NanoServiceErrorStatus::Unknown
        )
    })?;
    Ok(con)
}


fn unpack_result_string(result: Value) -> Result<String, NanoServiceError> {
    match result {
        Value::Status(s) => Ok(s),
        _ => Err(NanoServiceError::new(
            "Error converting the result into a string".to_string(), 
            NanoServiceErrorStatus::Unknown
        ))
    }
}


pub async fn login(
        address: &str,
        user_id: &str, 
        timeout_mins: usize,
        perm_user_id: i32
    ) -> Result<(), NanoServiceError> {
    let mut con = get_connnection(address).await?;

    let result = con
        .req_packed_command(
            &redis::cmd("login.set")
                .arg(user_id)
                .arg(timeout_mins)
                .arg(perm_user_id.to_string())
                .clone(),
        )
        .await.map_err(|e|{
            NanoServiceError::new(
                e.to_string(), 
                NanoServiceErrorStatus::Unknown
            )
        })?;
    match result {
        Value::Okay => {
            return Ok(());
        },
        _ => {
            return Err(NanoServiceError::new(
                format!("{:?}", result), 
                NanoServiceErrorStatus::Unknown
            ));
        }
    }
}


#[derive(Debug)]
pub enum UserSessionStatus {
    Ok(i32),
    Refresh
}


pub async fn update(
        address: &str,
        user_id: &str
    ) -> Result<UserSessionStatus, NanoServiceError> {
    let mut con = get_connnection(address).await?;

    let result = con
        .req_packed_command(
            &redis::cmd("update.set")
                .arg(user_id)
                .clone(),
        )
        .await.map_err(|e|{
            NanoServiceError::new(
                e.to_string(), 
                NanoServiceErrorStatus::Unknown
            )
        })?;
    let result = unpack_result_string(result)?;
    match result.as_str() {
        "TIMEOUT" => {
            return Err(NanoServiceError::new(
                "Session has timed out".to_string(), 
                NanoServiceErrorStatus::Unauthorized
            ));
        },
        "NOT_FOUND" => {
            return Err(NanoServiceError::new(
                "Session not found".to_string(), 
                NanoServiceErrorStatus::Unauthorized
            ));
        },
        "REFRESH" => {
            return Ok(UserSessionStatus::Refresh)
        },
        _ => {}
    }
    let perm_user_id = match result.parse::<i32>() {
        Ok(perm_user_id) => perm_user_id,
        Err(_) => {
            return Err(NanoServiceError::new(
                "Error converting the result into a string".to_string(), 
                NanoServiceErrorStatus::Unknown
            ));
        }
    };
    Ok(UserSessionStatus::Ok(perm_user_id))
}

pub async fn logout(
        address: &str,
        user_id: &str
    ) -> Result<String, Box<dyn Error>> {
    let mut con = get_connnection(address).await?;

    let result = con
        .req_packed_command(
            &redis::cmd("logout.set")
                .arg(user_id)
                .clone(),
        )
        .await.map_err(|e|{
            NanoServiceError::new(
                e.to_string(), 
                NanoServiceErrorStatus::Unknown
            )
        })?;
    let result_string = unpack_result_string(result)?;
    Ok(result_string)
}
