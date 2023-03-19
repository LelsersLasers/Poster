use axum_login::{
    // axum_sessions::{async_session::MemoryStore, SessionLayer},
    secrecy::SecretVec,
    AuthUser,
    // AuthLayer, RequireAuthorizationLayer, SqliteStore,
};


#[derive(Debug, Default, Clone, sqlx::FromRow, PartialEq, PartialOrd)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub display_name: String,
    pub password_hash: String,
}
impl AuthUser for User {
    fn get_id(&self) -> String {
        format!("{}", self.id)
    }

    fn get_password_hash(&self) -> SecretVec<u8> {
        SecretVec::new(self.password_hash.clone().into())
    }
}
// impl User {
//     pub fn get_rusty_user() -> Self {
//         Self {
//             id: "1".to_string(),
//             username: "username".to_string(),
//             display_name: "Ferris the Crab".to_string(),
//             password_hash: "password".to_string(),
//         }
//     }
// }

pub struct Post {
    pub id: i64,
    pub title: String,
    pub context: String,
    pub date: u64, // sec since epoch

    pub author: User, // 1 User : many Post
    // many Post : many User
    pub upvotes: Vec<User>,
    pub downvotes: Vec<User>,

    pub score: u32, // upvotes - downvotes
}

pub struct Comment {
    pub id: i64,
    pub context: String,
    pub date: u64, // sec since epoch

    pub author: User, // 1 User : many Post
    // many Post : many User
    pub upvotes: Vec<User>,
    pub downvotes: Vec<User>,

    pub parent: Box<PostOrComment>,

    pub score: u32, // upvotes - downvotes
}

pub enum PostOrComment {
    Post(Post),
    Comment(Comment),
}
