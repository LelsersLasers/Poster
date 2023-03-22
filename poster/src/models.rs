use crate::*;


#[derive(Debug, Default, Clone, sqlx::FromRow, PartialEq, PartialOrd)]
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



pub struct Account {
    pub id: i64,
    pub display_name: String,

    pub user: User, // 1 Account : 1 User
}

pub struct Post {
    pub id: i64,
    pub title: String,
    pub context: String,
    pub date: u64, // sec since epoch

    pub author: Account, // 1 Account : many Post
    // many Post : many User
    pub upvotes: Vec<Account>,
    pub downvotes: Vec<Account>,

    pub score: u32, // upvotes - downvotes
}

pub struct Comment {
    pub id: i64,
    pub context: String,
    pub date: u64, // sec since epoch

    pub author: Account, // 1 Account : many Post
    // many Post : many User
    pub upvotes: Vec<Account>,
    pub downvotes: Vec<Account>,

    pub parent: Box<PostOrComment>,

    pub score: u32, // upvotes - downvotes
}

pub enum PostOrComment {
    Post(Post),
    Comment(Comment),
}
