use crate::*;

pub async fn connect_to_db() -> sqlx::Pool<sqlx::Sqlite> {
    sqlx::SqlitePool::connect(DB_PATH).await.unwrap()
}