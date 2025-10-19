use redis_module::{
    Context, NextArg, RedisError, RedisResult, RedisString, RedisValue
};

use crate::user_session::UserSession;


/// Updates the last interacted field of a user session if the session has not timed out.
/// 
/// # Arguments
/// * `ctx` - The Redis context
/// * `args` - The arguments passed to the Redis command
/// 
/// # Returns
/// * `RedisResult` - The result of the Redis command
pub fn update(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    if args.len() < 2 {
        return Err(RedisError::WrongArity);
    }
    let mut args = args.into_iter().skip(1);
    let user_id = args.next_arg()?.to_string();

    let mut user_session = UserSession::from_id(user_id);
    let key_string = RedisString::create(None, user_session.key.clone());
    let key = ctx.open_key_writable(&key_string);

    if key.is_empty() {
        return Ok(RedisValue::SimpleStringStatic("NOT_FOUND"));
    }

    match &user_session.check_timeout(ctx)? {
        RedisValue::SimpleStringStatic("TIMEOUT") => {
            return Ok(RedisValue::SimpleStringStatic("TIMEOUT"));
        },
        RedisValue::SimpleStringStatic("REFRESH") => {
            user_session.update_last_interacted(ctx)?;
            return Ok(RedisValue::SimpleStringStatic("REFRESH"));
        },
        RedisValue::SimpleStringStatic("OK") => {
            user_session.update_last_interacted(ctx)?;
            let perm_user_id = match key.hash_get("perm_user_id")? {
                Some(perm_user_id) => perm_user_id,
                None => {
                    return Err(RedisError::Str(
                            "Could not get perm_user_id"
                        )
                    );
                }
            };  
            return Ok(RedisValue::SimpleString(perm_user_id.to_string()));
        },
        _ => {
            return Err(RedisError::Str("Could not check timeout"));
        }
    };
}