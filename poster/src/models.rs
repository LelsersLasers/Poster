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
