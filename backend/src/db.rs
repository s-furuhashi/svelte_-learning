use sqlx::sqlite::{SqlitePool, SqliteConnectOptions};
use std::str::FromStr;

pub async fn create_pool(database_url: &str) -> SqlitePool {
    let options = SqliteConnectOptions::from_str(database_url)
        .expect("Invalid DATABASE_URL")
        .create_if_missing(true);

    SqlitePool::connect_with(options)
        .await
        .expect("Failed to create database pool")
}
