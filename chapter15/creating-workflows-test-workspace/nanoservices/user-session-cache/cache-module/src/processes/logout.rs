use redis_module::{
    Context, NextArg, RedisError, RedisResult, RedisString, RedisValue
};

use crate::user_session::UserSession;


/// Deletes the record for a user session.
/// 
/// # Arguments
/// * `ctx` - The Redis context
/// * `args` - The arguments passed to the command
/// 
/// # Returns
/// * `RedisResult` - The result of the command
pub fn logout(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    if args.len() < 2 {
        return Err(RedisError::WrongArity);
    }
    let mut args = args.into_iter().skip(1);
    let user_id = args.next_arg()?.to_string();
    
    let user_session = UserSession::from_id(user_id);

    let key_string = RedisString::create(None, user_session.key);
    let key = ctx.open_key_writable(&key_string);

    if key.is_empty() {
        return Ok(RedisValue::SimpleStringStatic("NOT_FOUND"));
    }
    match key.delete(){
        Ok(_) => {},
        Err(_) => return Err(RedisError::Str("Could not delete key"))
    };
    Ok(RedisValue::SimpleStringStatic("OK"))
}
