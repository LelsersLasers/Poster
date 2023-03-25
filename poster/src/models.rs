use crate::*;


#[derive(Debug, Default, Clone, sqlx::FromRow, PartialEq, Eq, PartialOrd)]
pub struct User {
    pub id: String, // database primary key = username
    pub password_hash: String, 
}
impl User {
    pub fn new(username: String, password: String) -> Self {
        User {
            id: username,
            password_hash: utils::hash_password(&password)
        }
    }
    // pub async fn try_find(&self) -> Option<Self> {
    //     let db = sql::connect_to_db().await;
    //     let stream = sqlx::query("SELECT * FROM users WHERE id = ? AND password_hash = ?")
    //         .bind(&self.id)
    //         .bind(&self.password_hash)
    //         .map(|row: SqliteRow| {
    //             Self {
    //                 id: row.try_get("id").unwrap(),
    //                 password_hash: row.try_get("password_hash").unwrap()
    //             }
    //         })
    //         .fetch_optional(&db);
    
    //     stream.await.unwrap()        
    // }
    pub async fn exists(&self) -> bool {
        let db = sql::connect_to_db().await;
        let result = sqlx::query(sql::FIND_USER_SQL)
            .bind(&self.id)
            .bind(&self.password_hash)
            .map(|row: SqliteRow| {
                let count: u32 = row.try_get("found").unwrap();
                count
            })
            .fetch_one(&db)
            .await
            .unwrap();

        result == 1
    }
    pub async fn username_exists(username: &str) -> bool {
        let db = sql::connect_to_db().await;
        let result = sqlx::query(sql::FIND_USER_USERNAME_SQL)
            .bind(username)
            .map(|row: SqliteRow| {
                let count: u32 = row.try_get("found").unwrap();
                count
            })
            .fetch_one(&db)
            .await
            .unwrap();

        result == 1
    }
    pub async fn add_to_db(&self) {
        let already_exists = self.exists().await;
        if !already_exists {
            let db = sql::connect_to_db().await;
            sqlx::query(sql::ADD_USER_SQL)
                .bind(&self.id)
                .bind(&self.password_hash)
                .execute(&db)
                .await
                .unwrap();
        }
    }
}
impl AuthUser for User {
    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_password_hash(&self) -> SecretVec<u8> {
        SecretVec::new(self.password_hash.clone().into())
    }
}


#[derive(Serialize)]
pub struct Account {
    pub id: u32,
    pub display_name: String,

    pub user_id: String, // 1 Account : 1 User
}
impl Account {
    pub fn new(display_name: String, user_id: String) -> Self {
        Account {
            id: 0, // will be set by database
            display_name,
            user_id,
        }
    }
    pub async fn display_name_exists(display_name: &str) -> bool {
        let db = sql::connect_to_db().await;
        let result = sqlx::query(sql::FIND_ACCOUNT_DISPLAY_NAME_SQL)
            .bind(display_name)
            .map(|row: SqliteRow| {
                let count: u32 = row.try_get("found").unwrap();
                count
            })
            .fetch_one(&db)
            .await
            .unwrap();

        result == 1
    }
    pub async fn add_to_db(&self) {
        let already_exists = Account::display_name_exists(&self.display_name).await;
        if !already_exists {
            let db = sql::connect_to_db().await;
            sqlx::query(sql::ADD_ACCOUNT_SQL)
                .bind(&self.display_name)
                .bind(&self.user_id)
                .execute(&db)
                .await
                .unwrap();
        }
    }
    pub async fn from_user(user: &User) -> Account {
        let db = sql::connect_to_db().await;
        sqlx::query(sql::GET_ACCOUNT_FROM_USER_ID_SQL)
            .bind(&user.id)
            .map(|row: SqliteRow| {
                Account {
                    id: row.try_get("id").unwrap(),
                    display_name: row.try_get("display_name").unwrap(),
                    user_id: row.try_get("user_id").unwrap(),
                }
            })
            .fetch_one(&db)
            .await
            .unwrap()
    }
    pub async fn from_id(id: u32) -> Account {
        let db = sql::connect_to_db().await;
        sqlx::query(sql::GET_ACCOUNT_FROM_ID_SQL)
            .bind(id)
            .map(|row: SqliteRow| {
                Account {
                    id: row.try_get("id").unwrap(),
                    display_name: row.try_get("display_name").unwrap(),
                    user_id: row.try_get("user_id").unwrap(),
                }
            })
            .fetch_one(&db)
            .await
            .unwrap()
    }
}


#[derive(Serialize)]
pub struct Post {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub date: String, // sec since epoch as String

    pub account_id: u32, // 1 Account : many Post

    // pub upvotes: Vec<Account>,
    // pub downvotes: Vec<Account>,

    // pub score: u32, // upvotes - downvotes
}
impl Post {
    pub fn new(title: String, content: String, date: String, account_id: u32) -> Self {
        Self {
            id: 0, // will be set by database
            title,
            content,
            date,
            account_id,
        }
    }
    pub async fn add_to_db(&self) {
        let db = sql::connect_to_db().await;
        sqlx::query(sql::ADD_POST_SQL)
            .bind(&self.title)
            .bind(&self.content)
            .bind(&self.date)
            .bind(&self.account_id)
            .execute(&db)
            .await
            .unwrap();
    }
    // pub async fn from_id(id: u32) -> Post {
    //     let db = sql::connect_to_db().await;
    //     sqlx::query(sql::GET_POST_FROM_ID_SQL)
    //         .bind(id)
    //         .map(|row: SqliteRow| {
    //             Post {
    //                 id: row.try_get("id").unwrap(),
    //                 title: row.try_get("title").unwrap(),
    //                 content: row.try_get("content").unwrap(),
    //                 date: row.try_get("date").unwrap(),
    //                 account_id: row.try_get("account_id").unwrap(),
    //             }
    //         })
    //         .fetch_one(&db)
    //         .await
    //         .unwrap()
    // }
    pub async fn maybe_from_id(id: u32) -> Option<Post> {
        let db = sql::connect_to_db().await;
        sqlx::query(sql::GET_POST_FROM_ID_SQL)
            .bind(id)
            .map(|row: SqliteRow| {
                Post {
                    id: row.try_get("id").unwrap(),
                    title: row.try_get("title").unwrap(),
                    content: row.try_get("content").unwrap(),
                    date: row.try_get("date").unwrap(),
                    account_id: row.try_get("account_id").unwrap(),
                }
            })
            .fetch_optional(&db)
            .await
            .unwrap()
    }
    pub async fn count_comments(&self) -> u32 {
        let db = sql::connect_to_db().await;
        sqlx::query(sql::COUNT_COMMENTS_ON_POST_SQL)
            .bind(self.id)
            .map(|row: SqliteRow| {
                let count: u32 = row.try_get("count").unwrap();
                count
            })
            .fetch_one(&db)
            .await
            .unwrap()
    }
}

#[derive(Serialize)]
pub struct Comment {
    pub id: u32,
    pub content: String,
    pub date: String, // sec since epoch

    pub account_id: u32,    // 1 Account : many Comment
    pub post_id: u32,       // 1 Post : many Comment
    pub parent_comment_id: Option<u32>, // 1 Comment : many Comment


    // many Post : many User
    // pub upvotes: Vec<Account>,
    // pub downvotes: Vec<Account>,

    // pub parent: Box<PostOrComment>,

    // pub score: u32, // upvotes - downvotes
}
impl Comment {
    pub fn new(content: String, date: String, account_id: u32, post_id: u32, parent_comment_id: Option<u32>) -> Self {
        Self {
            id: 0, // will be set by database
            content,
            date,
            account_id,
            post_id,
            parent_comment_id,
        }
    }
    pub async fn top_level_comments_from_post_id(post_id: u32) -> Vec<Self> {
        let db = sql::connect_to_db().await;
        sqlx::query(sql::GET_TOP_LEVEL_COMMENTS_ON_POST_SQL)
            .bind(post_id)
            .map(|row: SqliteRow| {
                Comment {
                    id: row.try_get("id").unwrap(),
                    content: row.try_get("content").unwrap(),
                    date: row.try_get("date").unwrap(),
                    account_id: row.try_get("account_id").unwrap(),
                    post_id: row.try_get("post_id").unwrap(),
                    // parent_comment_id: row.try_get("parent_comment_id").unwrap(),
                    parent_comment_id: None,
                }
            })
            .fetch_all(&db)
            .await
            .unwrap()
    }
    pub async fn add_to_db(&self) {
        let db = sql::connect_to_db().await;
        if let Some(parent_comment_id) = self.parent_comment_id {
            sqlx::query(sql::ADD_COMMENT_TO_COMMENT_SQL)
                .bind(&self.content)
                .bind(&self.date)
                .bind(&self.account_id)
                .bind(&self.post_id)
                .bind(&parent_comment_id)
                .execute(&db)
                .await
                .unwrap();
        } else {
            sqlx::query(sql::ADD_COMMENT_TO_POST_SQL)
                .bind(&self.content)
                .bind(&self.date)
                .bind(&self.account_id)
                .bind(&self.post_id)
                .execute(&db)
                .await
                .unwrap();
        }
    }
}

// pub enum PostOrComment {
//     Post(Post),
//     Comment(Comment),
// }
