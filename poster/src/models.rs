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
            password_hash: utils::hash_password(&password),
        }
    }
    pub async fn exists(&self, pool: &SqlitePool) -> bool {
        let result = sqlx::query(sql::FIND_USER_SQL)
            .bind(&self.id)
            .bind(&self.password_hash)
            .map(|row: SqliteRow| {
                let count: u32 = row.try_get("found").unwrap();
                count
            })
            .fetch_one(pool)
            .await
            .unwrap();

        result == 1
    }
    pub async fn username_exists(username: &str, pool: &SqlitePool) -> bool {
        let result = sqlx::query(sql::FIND_USER_USERNAME_SQL)
            .bind(username)
            .map(|row: SqliteRow| {
                let count: u32 = row.try_get("found").unwrap();
                count
            })
            .fetch_one(pool)
            .await
            .unwrap();

        result == 1
    }
    pub async fn add_to_db(&self, pool: &SqlitePool) {
        let already_exists = self.exists(pool).await;
        if !already_exists {
            sqlx::query(sql::ADD_USER_SQL)
                .bind(&self.id)
                .bind(&self.password_hash)
                .execute(pool)
                .await
                .unwrap();
        }
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
    pub async fn display_name_exists(display_name: &str, pool: &SqlitePool) -> bool {
        let result = sqlx::query(sql::FIND_ACCOUNT_DISPLAY_NAME_SQL)
            .bind(display_name)
            .map(|row: SqliteRow| {
                let count: u32 = row.try_get("found").unwrap();
                count
            })
            .fetch_one(pool)
            .await
            .unwrap();

        result == 1
    }
    pub async fn add_to_db(&self, pool: &SqlitePool) {
        let already_exists = Account::display_name_exists(&self.display_name, pool).await;
        if !already_exists {
            sqlx::query(sql::ADD_ACCOUNT_SQL)
                .bind(&self.display_name)
                .bind(&self.user_id)
                .execute(pool)
                .await
                .unwrap();
        }
    }
    pub async fn from_user(user: &User, pool: &SqlitePool) -> Account {
        sqlx::query(sql::GET_ACCOUNT_FROM_USER_ID_SQL)
            .bind(&user.id)
            .map(|row: SqliteRow| Account {
                id: row.try_get("id").unwrap(),
                display_name: row.try_get("display_name").unwrap(),
                user_id: row.try_get("user_id").unwrap(),
            })
            .fetch_one(pool)
            .await
            .unwrap()
    }
    pub async fn from_id(id: u32, pool: &SqlitePool) -> Account {
        sqlx::query(sql::GET_ACCOUNT_FROM_ID_SQL)
            .bind(id)
            .map(|row: SqliteRow| Account {
                id: row.try_get("id").unwrap(),
                display_name: row.try_get("display_name").unwrap(),
                user_id: row.try_get("user_id").unwrap(),
            })
            .fetch_one(pool)
            .await
            .unwrap()
    }
}

#[derive(Serialize)]
pub struct PostData {
    pub post: models::Post,
    pub account: models::Account,
    pub vote_value: i32,
    pub comment_count: u32,
    pub date_string: String,
}

#[derive(Serialize)]
pub struct Post {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub date: String, // sec since epoch as String

    pub account_id: u32, // 1 Account : many Post

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
    pub async fn add_to_db(&self, pool: &SqlitePool) -> u32 {
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
            .fetch_one(pool)
            .await
            .unwrap();

        // auto upvote
        sqlx::query(sql::ADD_POST_VOTE_SQL)
            .bind(new_id)
            .bind(self.account_id)
            .bind(1)
            .execute(pool)
            .await
            .unwrap();

        sqlx::query(sql::UPDATE_POST_SCORE_SQL)
            .bind(1)
            .bind(self.id)
            .execute(pool)
            .await
            .unwrap();

        new_id
    }
    pub async fn maybe_from_id(id: u32, pool: &SqlitePool) -> Option<Post> {
        sqlx::query(sql::GET_POST_FROM_ID_SQL)
            .bind(id)
            .map(|row: SqliteRow| Post {
                id: row.try_get("id").unwrap(),
                title: row.try_get("title").unwrap(),
                content: row.try_get("content").unwrap(),
                date: row.try_get("date").unwrap(),
                score: row.try_get("score").unwrap(),
                account_id: row.try_get("account_id").unwrap(),
            })
            .fetch_optional(pool)
            .await
            .unwrap()
    }
    pub async fn maybe_newest_post(pool: &SqlitePool) -> Option<Post> {
        sqlx::query(sql::GET_NEWEST_POST_SQL)
            .map(|row: SqliteRow| Post {
                id: row.try_get("id").unwrap(),
                title: row.try_get("title").unwrap(),
                content: row.try_get("content").unwrap(),
                date: row.try_get("date").unwrap(),
                score: row.try_get("score").unwrap(),
                account_id: row.try_get("account_id").unwrap(),
            })
            .fetch_optional(pool)
            .await
            .unwrap()
    }
    pub async fn maybe_best_post(pool: &SqlitePool) -> Option<Post> {
        sqlx::query(sql::GET_BEST_POST_SQL)
            .map(|row: SqliteRow| Post {
                id: row.try_get("id").unwrap(),
                title: row.try_get("title").unwrap(),
                content: row.try_get("content").unwrap(),
                date: row.try_get("date").unwrap(),
                score: row.try_get("score").unwrap(),
                account_id: row.try_get("account_id").unwrap(),
            })
            .fetch_optional(pool)
            .await
            .unwrap()
    }
    pub async fn count_comments(&self, pool: &SqlitePool) -> u32 {
        sqlx::query(sql::COUNT_COMMENTS_ON_POST_SQL)
            .bind(self.id)
            .map(|row: SqliteRow| {
                let count: u32 = row.try_get("count").unwrap();
                count
            })
            .fetch_one(pool)
            .await
            .unwrap()
    }
    pub async fn vote(id: u32, account_id: u32, vote_value: i32, pool: &SqlitePool) {
        let existed = sqlx::query(sql::POST_VOTE_EXISTS_SQL)
            .bind(id)
            .bind(account_id)
            .bind(vote_value)
            .map(|row: SqliteRow| {
                let count: i32 = row.try_get("count").unwrap();
                count != 0
            })
            .fetch_one(pool)
            .await
            .unwrap();

        sqlx::query(sql::DELETE_POST_VOTE_SQL)
            .bind(id)
            .bind(account_id)
            .execute(pool)
            .await
            .unwrap();

        if !existed {
            // if existed => untoggle vote
            sqlx::query(sql::ADD_POST_VOTE_SQL)
                .bind(id)
                .bind(account_id)
                .bind(vote_value)
                .execute(pool)
                .await
                .unwrap();
        }

        let new_post_score = sqlx::query(sql::CALCULATE_POST_SCORE_SQL)
            .bind(id)
            .map(|row: SqliteRow| {
                let score: i32 = row.try_get("score").unwrap();
                score
            })
            .fetch_one(pool)
            .await
            .unwrap();

        sqlx::query(sql::UPDATE_POST_SCORE_SQL)
            .bind(new_post_score)
            .bind(id)
            .execute(pool)
            .await
            .unwrap();
    }
    pub async fn maybe_account_vote(&self, account_id: u32, pool: &SqlitePool) -> Option<i32> {
        sqlx::query(sql::GET_POST_VOTE_SQL)
            .bind(self.id)
            .bind(account_id)
            .map(|row: SqliteRow| {
                let vote_value: i32 = row.try_get("vote_value").unwrap();
                vote_value
            })
            .fetch_optional(pool)
            .await
            .unwrap()
    }
    pub async fn into_post_data(self, session: &WritableSession, pool: &SqlitePool) -> PostData {
        let account = models::Account::from_id(self.account_id, pool).await;
        let comment_count = self.count_comments(pool).await;
        let vote_value = if let Some(user) = session.get::<models::User>("current_user") {
            let account = models::Account::from_user(&user, pool).await;
            let maybe_vote_value = self.maybe_account_vote(account.id, pool).await;
            if let Some(vote_value) = maybe_vote_value {
                vote_value
            } else {
                0
            }
        } else {
            -2 // not -1, 0, 1
        };

        let date_string = utils::padded_time_to_date_string(&self.date, "%b %-d, %Y, at %-k:%M");

        PostData {
            post: self,
            account,
            comment_count,
            vote_value,
            date_string,
        }
    }
}

#[derive(Serialize)]
pub struct CommentTreeNode {
    pub comment: Comment,
    pub account: Account,
    pub children: Vec<CommentTreeNode>,
    pub vote_value: i32,
    pub date_string: String,
}

#[derive(Serialize)]
pub struct Comment {
    pub id: u32,
    pub content: String,
    pub date: String, // sec since epoch

    pub account_id: u32,                // 1 Account : many Comment
    pub post_id: u32,                   // 1 Post : many Comment
    pub parent_comment_id: Option<u32>, // 1 Comment : many Comment

    pub score: i32, // upvotes - downvotes
}
impl Comment {
    pub fn new(
        content: String,
        date: String,
        account_id: u32,
        post_id: u32,
        parent_comment_id: Option<u32>,
    ) -> Self {
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
    pub async fn top_level_comments_from_post_id(post_id: u32, pool: &SqlitePool) -> Vec<Self> {
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
            .fetch_all(pool)
            .await
            .unwrap()
    }
    #[async_recursion]
    pub async fn build_comment_tree(
        self,
        session: &WritableSession,
        pool: &SqlitePool,
    ) -> CommentTreeNode {
        let mut children = Vec::new();
        let child_comments = sqlx::query(sql::GET_COMMENTS_ON_COMMENT_SQL)
            .bind(self.id)
            .map(|row: SqliteRow| Comment {
                id: row.try_get("id").unwrap(),
                content: row.try_get("content").unwrap(),
                date: row.try_get("date").unwrap(),
                score: row.try_get("score").unwrap(),
                account_id: row.try_get("account_id").unwrap(),
                post_id: row.try_get("post_id").unwrap(),
                parent_comment_id: row.try_get("parent_comment_id").unwrap(),
            })
            .fetch_all(pool)
            .await
            .unwrap();
        for child_comment in child_comments {
            children.push(Comment::build_comment_tree(child_comment, session, pool).await);
        }

        let vote_value = Comment::get_vote_value(self.id, self.post_id, session, pool).await;
        let account = Account::from_id(self.account_id, pool).await;
        let date_string = utils::padded_time_to_date_string(&self.date, "%b %-d, %Y, at %-k:%M");

        CommentTreeNode {
            comment: self,
            account,
            children,
            vote_value,
            date_string,
        }
    }
    pub async fn maybe_from_id(id: u32, post_id: u32, pool: &SqlitePool) -> Option<Self> {
        sqlx::query(sql::GET_COMMENT_FROM_IDS_SQL)
            .bind(id)
            .bind(post_id)
            .map(|row: SqliteRow| Comment {
                id: row.try_get("id").unwrap(),
                content: row.try_get("content").unwrap(),
                date: row.try_get("date").unwrap(),
                score: row.try_get("score").unwrap(),
                account_id: row.try_get("account_id").unwrap(),
                post_id: row.try_get("post_id").unwrap(),
                parent_comment_id: row.try_get("parent_comment_id").unwrap(),
            })
            .fetch_optional(pool)
            .await
            .unwrap()
    }
    pub async fn vote(
        id: u32,
        post_id: u32,
        account_id: u32,
        vote_value: i32,
        pool: &SqlitePool,
    ) -> i32 {
        let existed = sqlx::query(sql::COMMENT_VOTE_EXISTS_SQL)
            .bind(id)
            .bind(account_id)
            .bind(vote_value)
            .map(|row: SqliteRow| {
                let count: i32 = row.try_get("count").unwrap();
                count != 0
            })
            .fetch_one(pool)
            .await
            .unwrap();

        sqlx::query(sql::DELETE_COMMENT_VOTE_SQL)
            .bind(id)
            .bind(account_id)
            .execute(pool)
            .await
            .unwrap();

        if !existed {
            // if existed => untoggle vote
            sqlx::query(sql::ADD_COMMENT_VOTE_SQL)
                .bind(id)
                .bind(post_id)
                .bind(account_id)
                .bind(vote_value)
                .execute(pool)
                .await
                .unwrap();
        }

        let new_comment_score = sqlx::query(sql::CALCULATE_COMMENT_SCORE_SQL)
            .bind(id)
            .map(|row: SqliteRow| {
                let score: i32 = row.try_get("score").unwrap();
                score
            })
            .fetch_one(pool)
            .await
            .unwrap();

        sqlx::query(sql::UPDATE_COMMENT_SCORE_SQL)
            .bind(new_comment_score)
            .bind(id)
            .execute(pool)
            .await
            .unwrap();

        new_comment_score
    }
    pub async fn add_to_db(&self, pool: &SqlitePool) {
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
                .fetch_one(pool)
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
                .fetch_one(pool)
                .await
                .unwrap()
        };

        // TODO: auto upvote
        sqlx::query(sql::ADD_COMMENT_VOTE_SQL)
            .bind(new_id)
            .bind(self.post_id)
            .bind(self.account_id)
            .bind(1)
            .execute(pool)
            .await
            .unwrap();

        sqlx::query(sql::UPDATE_COMMENT_SCORE_SQL)
            .bind(1)
            .bind(self.id)
            .execute(pool)
            .await
            .unwrap();
    }
    pub async fn get_vote_value(
        id: u32,
        post_id: u32,
        session: &WritableSession,
        pool: &SqlitePool,
    ) -> i32 {
        if let Some(user) = &session.get::<User>("current_user") {
            let account = models::Account::from_user(user, pool).await;
            let maybe_vote_value = {
                sqlx::query(sql::GET_COMMENT_VOTE_SQL)
                    .bind(id)
                    .bind(post_id)
                    .bind(account.id)
                    .map(|row: SqliteRow| {
                        let vote_value: i32 = row.try_get("vote_value").unwrap();
                        vote_value
                    })
                    .fetch_optional(pool)
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
