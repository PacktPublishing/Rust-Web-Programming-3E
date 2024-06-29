use redis_module::{
    Context, NextArg, RedisError, RedisResult, RedisString, RedisValue
};
use chrono::{DateTime, Utc};

use crate::user_session::UserSession;


/// Enters a record for a user session. 
/// 
/// # Arguments
/// * `ctx` - The Redis context
/// * `args` - The arguments passed to the command
/// 
/// # Returns
/// * `RedisResult` - The result of the command
pub fn login(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    // Add the role for the user session
    if args.len() < 3 {
        return Err(RedisError::WrongArity);
    }
    let now: DateTime<Utc> = Utc::now();

    let mut args = args.into_iter().skip(1);
    let user_id = args.next_arg()?.to_string();
    let timeout_mins = args.next_arg()?;
    let formatted_date_string = now.format("%Y-%m-%d %H:%M:%S").to_string();

    let user_session = UserSession::from_id(user_id);

    let key_string = RedisString::create(None, user_session.key);
    let last_interacted_string = RedisString::create(None, formatted_date_string);

    let key = ctx.open_key_writable(&key_string);
    key.hash_set("last_interacted", ctx.create_string(last_interacted_string));
    key.hash_set("timeout_mins", ctx.create_string(timeout_mins));

    Ok(RedisValue::SimpleStringStatic("OK"))
}