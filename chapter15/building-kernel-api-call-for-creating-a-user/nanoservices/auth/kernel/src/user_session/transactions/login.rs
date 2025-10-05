use std::future::Future;
use glue::errors::NanoServiceError;
use crate::user_session::descriptors::RedisSessionDescriptor;
use cache_client::login as cache_login;


pub trait LoginUserSession {
    fn login_user_session(
        address: &str,
        user_id: &str, 
        timeout_mins: usize,
        perm_user_id: i32
    ) 
    -> impl Future<Output = Result<(), NanoServiceError>>;
}

impl LoginUserSession for RedisSessionDescriptor {

    fn login_user_session(
        address: &str,
        user_id: &str, 
        timeout_mins: usize,
        perm_user_id: i32
    ) 
    -> impl Future<Output = Result<(), NanoServiceError>> {
        cache_login(address, user_id, timeout_mins, perm_user_id)
    }
}
