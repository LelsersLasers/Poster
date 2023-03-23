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

pub const FIND_ACCOUNT_DISPLAY_NAME_SQL: &str = r#"
    SELECT
        COUNT(display_name) AS found
    FROM accounts
    WHERE
        display_name = ?;
"#;

pub const ADD_ACCOUNT_SQL: &str = r#"
    INSERT INTO accounts 
        (display_name, user_id)
    VALUES (?, ?);
"#;

pub const SELECT_ACCOUNT_FROM_USER_ID_SQL: &str = r#"
    SELECT
        *
    FROM accounts
    WHERE
        user_id = ?;
"#;

pub const ADD_POST_SQL: &str = r#"
    INSERT INTO posts
        (title, content, date, account_id)
    VALUES (?, ?, ?, ?)
"#;


pub async fn connect_to_db() -> sqlx::Pool<sqlx::Sqlite> {
    sqlx::SqlitePool::connect(DB_PATH).await.unwrap()
}