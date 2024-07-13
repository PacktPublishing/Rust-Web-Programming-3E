use redis_module::{
    Context, NextArg, RedisError, RedisResult, RedisString, RedisValue
};

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
    if args.len() < 4 {
        return Err(RedisError::WrongArity);
    }
    let mut args = args.into_iter().skip(1);
    let user_id = args.next_arg()?.to_string();
    let timeout_mins = args.next_arg()?;
    let perm_user_id = args.next_arg()?.to_string();

    let user_session = UserSession::from_id(user_id);
    user_session.update_last_interacted(ctx)?;

    let key_string = RedisString::create(None, user_session.key);

    let key = ctx.open_key_writable(&key_string);
    key.hash_set("timeout_mins", ctx.create_string(timeout_mins));
    key.hash_set("counter", ctx.create_string("0"));
    key.hash_set("perm_user_id", ctx.create_string(perm_user_id));

    Ok(RedisValue::SimpleStringStatic("OK"))
}