//! Defines the migration functions for databases.
use crate::connections::sqlx_postgres::SQLX_POSTGRES_POOL;


/// Runs a set of embedded migrations for a specific database.
pub async fn run_migrations() {
    println!("Migrating auth database...");
    let mut migrations = sqlx::migrate!("./migrations");
    migrations.ignore_missing = true;
    let result = migrations.run(&*SQLX_POSTGRES_POOL).await.unwrap();
    println!("auth database migrations completed: {:?}", result);
}