use crate::*;

pub const FIND_USER_SQL: &str = r#"
    SELECT
        COUNT(id) AS found
    FROM users
    WHERE
        id = ? AND password_hash = ?
;"#;

pub const FIND_USER_USERNAME_SQL: &str = r#"
    SELECT
        COUNT(id) AS found
    FROM users
    WHERE
        id = ?
;"#;

pub const ADD_USER_SQL: &str = r#"
    INSERT INTO users 
        (id, password_hash)
    VALUES (?, ?)  
;"#;

pub const FIND_ACCOUNT_DISPLAY_NAME_SQL: &str = r#"
    SELECT
        COUNT(display_name) AS found
    FROM accounts
    WHERE
        display_name = ?
;"#;

pub const ADD_ACCOUNT_SQL: &str = r#"
    INSERT INTO accounts 
        (display_name, user_id)
    VALUES (?, ?)
;"#;

pub const GET_ACCOUNT_FROM_USER_ID_SQL: &str = r#"
    SELECT
        *
    FROM accounts
    WHERE
        user_id = ?
;"#;

pub const GET_ACCOUNT_FROM_ID_SQL: &str = r#"
    SELECT
        *
    FROM accounts
    WHERE
        id = ?
;"#;

pub const ADD_POST_SQL: &str = r#"
    INSERT INTO posts
        (title, content, date, account_id)
    VALUES (?, ?, ?, ?)
;"#;

pub const GET_ALL_POSTS_SQL: &str = r#"
    SELECT
        *
    FROM posts
;"#;

pub const GET_POST_FROM_ID_SQL: &str = r#"
    SELECT
        *
    FROM posts
    WHERE
        id = ?
;"#;

pub const COUNT_COMMENTS_ON_POST_SQL: &str = r#"
    SELECT
        COUNT(id) AS count
    FROM comments
    WHERE
        post_id = ?
;"#;

pub const ADD_COMMENT_TO_POST_SQL: &str = r#"
    INSERT INTO comments
        (content, date, account_id, post_id)
    VALUES (?, ?, ?, ?)
;"#;

pub const ADD_COMMENT_TO_COMMENT_SQL: &str = r#"
    INSERT INTO comments
        (content, date, account_id, post_id, parent_comment_id)
    VALUES (?, ?, ?, ?, ?)
;"#;

pub const GET_COMMENTS_FROM_POST_SQL: &str = r#"
    SELECT
        *
    FROM comments
    WHERE
        post_id = ?
;"#;

pub const GET_TOP_LEVEL_COMMENTS_ON_POST_SQL: &str = r#"
    SELECT
        *
    FROM comments
    WHERE
        post_id = ? AND parent_comment_id IS NULL
;"#;

pub const GET_CHILD_COMMENTS_ON_COMMENT_SQL: &str = r#"
    SELECT
        *
    FROM comments
    WHERE
        parent_comment_id = ?
;"#;


pub async fn connect_to_db() -> sqlx::Pool<sqlx::Sqlite> {
    sqlx::SqlitePool::connect(DB_PATH).await.unwrap()
}