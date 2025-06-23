use sqlx::postgres::{PgPool, PgPoolOptions};
use std::sync::LazyLock as Lazy;
use std::env;


pub static SQLX_POSTGRES_POOL: Lazy<PgPool> = Lazy::new(|| {
    let connection_string = env::var("TO_DO_DB_URL").unwrap();
    let max_connections = match std::env::var(
        "TO_DO_MAX_CONNECTIONS"
    ) {
        Ok(val) => val,
        Err(_) => "5".to_string()
    }.trim().parse::<u32>().map_err(|_e|{
        "Could not parse max connections".to_string()
    }).unwrap();
    let pool = PgPoolOptions::new()
            .max_connections(max_connections);
    pool.connect_lazy(&connection_string)
        .expect("Failed to create pool")
});
