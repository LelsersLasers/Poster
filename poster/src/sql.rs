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
        (title, content, date, score, account_id)
    VALUES (?, ?, ?, ?, ?)
    RETURNING id
;"#;

pub const GET_POSTS_BASE_SQL: &str = r#"
    SELECT
        *
    FROM posts"#;

pub const GET_NEWEST_POST_SQL: &str = r#"
    SELECT
        *
    FROM posts
    ORDER BY date DESC
    LIMIT 1
;"#;

pub const GET_BEST_POST_SQL: &str = r#"
    SELECT
        *
    FROM posts
    ORDER BY score DESC, date DESC
    LIMIT 1
;"#;

pub const GET_POST_FROM_ID_SQL: &str = r#"
    SELECT
        *
    FROM posts
    WHERE
        id = ?
;"#;

pub const GET_COMMENT_FROM_IDS_SQL: &str = r#"
    SELECT
        *
    FROM comments
    WHERE
        id = ? AND post_id = ?
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
        (content, date, score, account_id, post_id)
    VALUES (?, ?, ?, ?, ?)
    RETURNING id
;"#;

pub const ADD_COMMENT_TO_COMMENT_SQL: &str = r#"
    INSERT INTO comments
        (content, date, score, account_id, post_id, parent_comment_id)
    VALUES (?, ?, ?, ?, ?, ?)
    RETURNING id
;"#;

pub const GET_TOP_LEVEL_COMMENTS_ON_POST_SQL: &str = r#"
    SELECT
        *
    FROM comments
    WHERE
        post_id = ? AND parent_comment_id IS NULL
    ORDER BY score DESC, date DESC
;"#;

pub const GET_COMMENTS_ON_COMMENT_SQL: &str = r#"
    SELECT
        *
    FROM comments
    WHERE
        parent_comment_id = ?
    ORDER BY score DESC, date DESC
;"#;

//----------------------------------------------------------------------------//
pub const DELETE_POST_VOTE_SQL: &str = r#"
    DELETE FROM post_votes
    WHERE
        post_id = ? AND account_id = ?
;"#;
pub const ADD_POST_VOTE_SQL: &str = r#"
    INSERT INTO post_votes
        (post_id, account_id, vote)
    VALUES (?, ?, ?)
;"#;
pub const CALCULATE_POST_SCORE_SQL: &str = r#"
    SELECT
        SUM(vote) AS score
    FROM post_votes
    WHERE
        post_id = ?
;"#;
pub const UPDATE_POST_SCORE_SQL: &str = r#"
    UPDATE posts
    SET score = ?
    WHERE
        id = ?
;"#;
pub const POST_VOTE_EXISTS_SQL: &str = r#"
    SELECT
        COUNT(*) AS count
    FROM post_votes
    WHERE
        post_id = ? AND account_id = ? and vote = ?
;"#;
//----------------------------------------------------------------------------//
pub const DELETE_COMMENT_VOTE_SQL: &str = r#"
    DELETE FROM comment_votes
    WHERE
        comment_id = ? AND account_id = ?
;"#;
pub const ADD_COMMENT_VOTE_SQL: &str = r#"
    INSERT INTO comment_votes
        (comment_id, post_id, account_id, vote)
    VALUES (?, ?, ?, ?)
;"#;
pub const CALCULATE_COMMENT_SCORE_SQL: &str = r#"
    SELECT
        SUM(vote) AS score
    FROM comment_votes
    WHERE
        comment_id = ?
;"#;
pub const UPDATE_COMMENT_SCORE_SQL: &str = r#"
    UPDATE comments
    SET score = ?
    WHERE
        id = ?
;"#;
pub const COMMENT_VOTE_EXISTS_SQL: &str = r#"
    SELECT
        COUNT(*) AS count
    FROM comment_votes
    WHERE
        comment_id = ? AND account_id = ? and vote = ?
;"#;
//----------------------------------------------------------------------------//
pub const GET_POST_VOTE_SQL: &str = r#"
    SELECT
        vote AS vote_value
    FROM post_votes
    WHERE
        post_id = ? AND account_id = ?
;"#;
pub const GET_COMMENT_VOTE_SQL: &str = r#"
    SELECT
        vote AS vote_value
    FROM comment_votes
    WHERE
        comment_id = ? AND post_id = ? AND account_id = ?
;"#;
