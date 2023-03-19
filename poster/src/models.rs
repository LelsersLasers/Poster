pub struct User {
    pub id: u64,
    pub username: String,
    pub display_name: String,
    pub password_hash: String,
}

pub struct Post {
    pub id: u64,
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
    pub id: u64,
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
