use redis_module::{
    Context, RedisString, RedisError, RedisResult, RedisValue
};
use chrono::{DateTime, Utc, NaiveDateTime};


pub struct UserSession {
    pub user_id: String,
    pub key: String,
    pub session_datetime: DateTime<Utc>,
}


impl UserSession {
    pub fn from_id(user_id: String) -> UserSession {
        UserSession {
            user_id: user_id.clone(),
            key: format!("user_session_{}", user_id),
            session_datetime: Utc::now(),
        }
    }

    pub fn check_timeout(&mut self, ctx: &Context) 
    -> RedisResult {
        let key_string = RedisString::create(None, self.key.clone());
        let key = ctx.open_key_writable(&key_string);
        let last_interacted_string = match key.hash_get("last_interacted")? {
            Some(v) => {
                match NaiveDateTime::parse_from_str(
                    &v.to_string(), "%Y-%m-%d %H:%M:%S"
                ) {
                    Ok(v) => v,
                    Err(e) => {
                        println!("Could not parse date: {:?}", e);
                        return Err(RedisError::Str("Could not parse date"))
                    }
                }
            },
            None => return Err(
                RedisError::Str("Last interacted field does not exist")
            )
        };
        let timeout_mins = match key.hash_get("timeout_mins")? {
            Some(v) => v.to_string().parse::<i32>().unwrap(),
            None => return Err(
                RedisError::Str("Timeout mins field does not exist")
            )
        };
        let time_diff = self.session_datetime
                            .naive_utc()
                            .signed_duration_since(last_interacted_string)
                            .num_minutes();
        if time_diff > timeout_mins.into() {
            match key.delete(){
                Ok(_) => {},
                Err(_) => return Err(
                    RedisError::Str("Could not delete key")
                )
            };
            return Ok(RedisValue::SimpleStringStatic("TIMEOUT"));
        }
        let mut counter = match self.get_counter(ctx)? {
            RedisValue::Integer(v) => v,
            _ => return Err(RedisError::Str("Could not get counter"))

        };
        counter += 1;
        key.hash_set("counter", ctx.create_string(counter.to_string()));
        if counter > 20 {
            return Ok(RedisValue::SimpleStringStatic("REFRESH"));
        }
        Ok(RedisValue::SimpleStringStatic("OK"))
    }

    pub fn update_last_interacted(&self, ctx: &Context) -> RedisResult {
        let key_string = RedisString::create(None, self.key.clone());
        let key = ctx.open_key_writable(&key_string);
        let formatted_date_string = self.session_datetime.format(
            "%Y-%m-%d %H:%M:%S"
        ).to_string();
        let last_interacted_string = RedisString::create(
            None, formatted_date_string
        );
        key.hash_set("last_interacted", ctx.create_string(
            last_interacted_string)
        );
        Ok(RedisValue::SimpleStringStatic("OK"))
    }

    pub fn get_counter(&self, ctx: &Context) -> RedisResult {
        let key_string = RedisString::create(None, self.key.clone());
        let key = ctx.open_key_writable(&key_string);
        match key.hash_get("counter")? {
            Some(v) => {
                let v = v.to_string().parse::<i64>().unwrap();
                Ok(RedisValue::Integer(v))
            },
            None => Err(RedisError::Str(
                "Counter field does not exist"
                )
            )
        }
    }

}
