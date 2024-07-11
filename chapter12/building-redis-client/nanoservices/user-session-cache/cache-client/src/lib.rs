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
        timeout_mins: usize
    ) -> Result<(), NanoServiceError> {
    let mut con = get_connnection(address).await?;

    let result = con
        .req_packed_command(
            &redis::cmd("login.set")
                .arg(user_id)
                .arg(timeout_mins)
                .clone(),
        )
        .await.map_err(|e|{
            NanoServiceError::new(
                e.to_string(), 
                NanoServiceErrorStatus::Unknown
            )
        })?;
    let result_string = unpack_result_string(result)?;
    if result_string != "OK" {
        return Err(NanoServiceError::new(
            result_string, 
            NanoServiceErrorStatus::Unknown
        ));
    }
    Ok(())
}


#[derive(Debug)]
pub enum UserSessionStatus {
    Ok,
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
    
    let result = match result {
        Value::Status(s) => {
            s
        },
        Value::Okay => {
            return Ok(UserSessionStatus::Ok)
        },
        _ => {
            return Err(NanoServiceError::new(
                "Error converting the result into a string".to_string(), 
                NanoServiceErrorStatus::Unknown
            ));
        }
    };
    match result.as_str() {
        "TIMEOUT" => {
            return Err(NanoServiceError::new(
                "Session has timed out".to_string(), 
                NanoServiceErrorStatus::Unauthorized
            ));
        },
        "OK" => {
            return Ok(UserSessionStatus::Ok)
        },
        "REFRESH" => {
            return Ok(UserSessionStatus::Refresh)
        },
        _ => {
            return Err(NanoServiceError::new(
                "Unknown status".to_string(), 
                NanoServiceErrorStatus::Unknown
            ));
        }
    }
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
