//! Defines the migration functions for databases.
use crate::connections::sqlx_postgres::SQLX_POSTGRES_POOL;


/// Runs a set of embedded migrations for a specific database.
pub async fn run_migrations() {
    println!("Migrating to-do database...");
    let mut migrations = sqlx::migrate!("./migrations");
    migrations.ignore_missing = true;
    let result = migrations.run(&*SQLX_POSTGRES_POOL).await.unwrap();
    println!("to-do database migrations completed: {:?}", result);
}