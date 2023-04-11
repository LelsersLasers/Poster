use crate::*;


#[derive(Serialize, Deserialize)]
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
// impl AuthUser for User {
//     fn get_id(&self) -> String {
//         self.id.clone()
//     }

//     fn get_password_hash(&self) -> SecretVec<u8> {
//         SecretVec::new(self.password_hash.clone().into())
//     }
// }


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
pub struct PostData {
    post: models::Post,
    account: models::Account,
    vote_value: i32,
    comment_count: u32,
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

    pub score: i32, // upvotes - downvotes
}
impl Post {
    pub fn new(title: String, content: String, date: String, account_id: u32) -> Self {
        Self {
            id: 0, // will be set by database
            title,
            content,
            date,
            score: 1,
            account_id,
        }
    }
    pub async fn add_to_db(&self) -> u32 {
        let db = sql::connect_to_db().await;
        let new_id = sqlx::query(sql::ADD_POST_SQL)
            .bind(&self.title)
            .bind(&self.content)
            .bind(&self.date)
            .bind(self.score)
            .bind(self.account_id)
            .map(|row: SqliteRow| {
                let id: u32 = row.try_get("id").unwrap();
                id
            })
            .fetch_one(&db)
            .await
            .unwrap();

        // auto upvote
        sqlx::query(sql::ADD_POST_VOTE_SQL)
            .bind(new_id)
            .bind(self.account_id)
            .bind(1)
            .execute(&db)
            .await
            .unwrap();

        sqlx::query(sql::UPDATE_POST_SCORE_SQL)
            .bind(1)
            .bind(self.id)
            .execute(&db)
            .await
            .unwrap();

        new_id
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
                    score: row.try_get("score").unwrap(),
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
    pub async fn vote(id: u32, account_id: u32, vote_value: i32) {
        let db = sql::connect_to_db().await;

        let existed = sqlx::query(sql::POST_VOTE_EXISTS_SQL)
            .bind(id)
            .bind(account_id)
            .bind(vote_value)
            .map(|row: SqliteRow| {
                let count: i32 = row.try_get("count").unwrap();
                count != 0
            })
            .fetch_one(&db)
            .await
            .unwrap();

        sqlx::query(sql::DELETE_POST_VOTE_SQL)
            .bind(id)
            .bind(account_id)
            .execute(&db)
            .await
            .unwrap();

        if !existed { // if existed => untoggle vote
            sqlx::query(sql::ADD_POST_VOTE_SQL)
                .bind(id)
                .bind(account_id)
                .bind(vote_value)
                .execute(&db)
                .await
                .unwrap();
        }

        let new_post_score = sqlx::query(sql::CALCULATE_POST_SCORE_SQL)
            .bind(id)
            .map(|row: SqliteRow| {
                let score: i32 = row.try_get("score").unwrap();
                score
            })
            .fetch_one(&db)
            .await
            .unwrap();

        sqlx::query(sql::UPDATE_POST_SCORE_SQL)
            .bind(new_post_score)
            .bind(id)
            .execute(&db)
            .await
            .unwrap();
    }
    pub async fn maybe_account_vote(&self, account_id: u32) -> Option<i32> {
        let db = sql::connect_to_db().await;
        sqlx::query(sql::GET_POST_VOTE_SQL)
            .bind(self.id)
            .bind(account_id)
            .map(|row: SqliteRow| {
                let vote_value: i32 = row.try_get("vote_value").unwrap();
                vote_value
            })
            .fetch_optional(&db)
            .await
            .unwrap()
    }
    pub async fn into_post_data(self,
        // auth: &AuthContext
        context_session: &ReadableSession,
    ) -> PostData {
        let account = models::Account::from_id(self.account_id).await;
        let comment_count = self.count_comments().await;
        let vote_value = if let Some(user) = context_session.get::<models::User>("current_user") {
            // let user = auth.current_user.as_ref().unwrap();
            let account = models::Account::from_user(&user).await;
            let maybe_vote_value = self.maybe_account_vote(account.id).await;
            if let Some(vote_value) = maybe_vote_value {
                vote_value
            } else {
                0
            }
        } else {
            -2 // not -1, 0, 1
        };

        PostData { post: self, account, comment_count, vote_value }
    }
}

#[derive(Serialize)]
pub struct CommentTreeNode {
    pub comment: Comment,
    pub account: Account,
    pub children: Vec<CommentTreeNode>,
    pub vote_value: i32,
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

    pub score: i32, // upvotes - downvotes
}
impl Comment {
    pub fn new(content: String, date: String, account_id: u32, post_id: u32, parent_comment_id: Option<u32>) -> Self {
        Self {
            id: 0, // will be set by database
            content,
            date,
            score: 1,
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
                    score: row.try_get("score").unwrap(),
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
    #[async_recursion]
    pub async fn build_comment_tree(
        self,
        // auth: &AuthContext
        context_session: &ReadableSession,
    ) -> CommentTreeNode {
        let mut children = Vec::new();
        let db = sql::connect_to_db().await;
        let child_comments = sqlx::query(sql::GET_COMMENTS_ON_COMMENT_SQL)
            .bind(self.id)
            .map(|row: SqliteRow| {
                Comment {
                    id: row.try_get("id").unwrap(),
                    content: row.try_get("content").unwrap(),
                    date: row.try_get("date").unwrap(),
                    score: row.try_get("score").unwrap(),
                    account_id: row.try_get("account_id").unwrap(),
                    post_id: row.try_get("post_id").unwrap(),
                    parent_comment_id: row.try_get("parent_comment_id").unwrap(),
                }
            })
            .fetch_all(&db)
            .await
            .unwrap();
        for child_comment in child_comments {
            children.push(Comment::build_comment_tree(child_comment, context_session).await);
        }
        
        let vote_value = Comment::get_vote_value(self.id, self.post_id, context_session).await;

        let account = Account::from_id(self.account_id).await;
        CommentTreeNode {
            comment: self,
            account,
            children,
            vote_value,
        }
    }
    pub async fn maybe_from_id(id: u32, post_id: u32) -> Option<Self> {
        let db = sql::connect_to_db().await;
        sqlx::query(sql::GET_COMMENT_FROM_IDS_SQL)
            .bind(id)
            .bind(post_id)
            .map(|row: SqliteRow| {
                Comment {
                    id: row.try_get("id").unwrap(),
                    content: row.try_get("content").unwrap(),
                    date: row.try_get("date").unwrap(),
                    score: row.try_get("score").unwrap(),
                    account_id: row.try_get("account_id").unwrap(),
                    post_id: row.try_get("post_id").unwrap(),
                    parent_comment_id: row.try_get("parent_comment_id").unwrap(),
                }
            })
            .fetch_optional(&db)
            .await
            .unwrap()
    }
    pub async fn vote(id: u32, post_id: u32, account_id: u32, vote_value: i32) -> i32 {
        let db = sql::connect_to_db().await;

        let existed = sqlx::query(sql::COMMENT_VOTE_EXISTS_SQL)
            .bind(id)
            .bind(account_id)
            .bind(vote_value)
            .map(|row: SqliteRow| {
                let count: i32 = row.try_get("count").unwrap();
                count != 0
            })
            .fetch_one(&db)
            .await
            .unwrap();

        sqlx::query(sql::DELETE_COMMENT_VOTE_SQL)
            .bind(id)
            .bind(account_id)
            .execute(&db)
            .await
            .unwrap();

        if !existed { // if existed => untoggle vote
            sqlx::query(sql::ADD_COMMENT_VOTE_SQL)
                .bind(id)
                .bind(post_id)
                .bind(account_id)
                .bind(vote_value)
                .execute(&db)
                .await
                .unwrap();
        }

        let new_comment_score = sqlx::query(sql::CALCULATE_COMMENT_SCORE_SQL)
            .bind(id)
            .map(|row: SqliteRow| {
                let score: i32 = row.try_get("score").unwrap();
                score
            })
            .fetch_one(&db)
            .await
            .unwrap();

        sqlx::query(sql::UPDATE_COMMENT_SCORE_SQL)
            .bind(new_comment_score)
            .bind(id)
            .execute(&db)
            .await
            .unwrap();


        new_comment_score
    }
    pub async fn add_to_db(&self) {
        let db = sql::connect_to_db().await;
        let new_id = if let Some(parent_comment_id) = self.parent_comment_id {
            sqlx::query(sql::ADD_COMMENT_TO_COMMENT_SQL)
                .bind(&self.content)
                .bind(&self.date)
                .bind(self.score)
                .bind(self.account_id)
                .bind(self.post_id)
                .bind(parent_comment_id)
                .map(|row: SqliteRow| {
                    let id: u32 = row.try_get("id").unwrap();
                    id
                })
                .fetch_one(&db)
                .await
                .unwrap()
        } else {
            sqlx::query(sql::ADD_COMMENT_TO_POST_SQL)
                .bind(&self.content)
                .bind(&self.date)
                .bind(self.score)
                .bind(self.account_id)
                .bind(self.post_id)
                .map(|row: SqliteRow| {
                    let id: u32 = row.try_get("id").unwrap();
                    id
                })
                .fetch_one(&db)
                .await
                .unwrap()
        };

        // TODO: auto upvote
        sqlx::query(sql::ADD_COMMENT_VOTE_SQL)
            .bind(new_id)
            .bind(self.post_id)
            .bind(self.account_id)
            .bind(1)
            .execute(&db)
            .await
            .unwrap();

        sqlx::query(sql::UPDATE_COMMENT_SCORE_SQL)
            .bind(1)
            .bind(self.id)
            .execute(&db)
            .await
            .unwrap();
    }
    pub async fn get_vote_value(
        id: u32, post_id: u32,
        // auth: &AuthContext
        context_session: &ReadableSession
    ) -> i32 {
        if let Some(user) = &context_session.get::<User>("current_user") {
            // let user = auth.current_user.as_ref().unwrap();
            let account = models::Account::from_user(user).await;
            let maybe_vote_value = {
                let db = sql::connect_to_db().await;
                sqlx::query(sql::GET_COMMENT_VOTE_SQL)
                    .bind(id)
                    .bind(post_id)
                    .bind(account.id)
                    .map(|row: SqliteRow| {
                        let vote_value: i32 = row.try_get("vote_value").unwrap();
                        vote_value
                    })
                    .fetch_optional(&db)
                    .await
                    .unwrap()
            };
            if let Some(vote_value) = maybe_vote_value {
                vote_value
            } else {
                0
            }
        } else {
            -2 // not -1, 0, 1
        }
    }
}

// pub enum PostOrComment {
//     Post(Post),
//     Comment(Comment),
// }
