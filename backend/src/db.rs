use sqlx::mysql::MySqlPool;

pub async fn create_pool(database_url: &str) -> MySqlPool {
    MySqlPool::connect(database_url)
        .await
        .expect("Failed to create database pool")
}
