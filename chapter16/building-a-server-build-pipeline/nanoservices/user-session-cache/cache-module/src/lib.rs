use redis_module::redis_module;

mod processes;
mod user_session;

use processes::{
    login::login,
    update::update,
    logout::logout
};


redis_module! {
    name: "user_sessions",
    version: 1,
    allocator: (redis_module::alloc::RedisAlloc, redis_module::alloc::RedisAlloc),
    data_types: [],
    commands: [
        ["login.set", login, "write fast deny-oom", 1, 1, 1],
        ["logout.set", logout, "write fast deny-oom", 1, 1, 1],
        ["update.set", update, "write fast deny-oom", 1, 1, 1],
    ]
}