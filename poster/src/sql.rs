use crate::*;

pub const FIND_USER_SQL: &str = r#"
    SELECT
        COUNT(id) AS found
    FROM users
    WHERE
        id = ? AND password_hash = ?;
"#;

pub const FIND_USER_USERNAME_SQL: &str = r#"
    SELECT
        COUNT(id) AS found
    FROM users
    WHERE
        id = ?;
"#;

pub const ADD_USER_SQL: &str = r#"
    INSERT INTO users 
        (id, password_hash)
    VALUES (?, ?);   
"#;

pub async fn connect_to_db() -> sqlx::Pool<sqlx::Sqlite> {
    sqlx::SqlitePool::connect(DB_PATH).await.unwrap()
}