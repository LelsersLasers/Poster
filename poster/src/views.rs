use crate::*;


pub async fn logout(mut auth: AuthContext) -> impl IntoResponse {
    dbg!("Logging out user: {}", &auth.current_user);
    auth.logout().await;

    Redirect::to(&(BASE_PATH.to_string() + "/"))
}


pub async fn attempt_login(
    auth: &mut AuthContext,
    user: &models::User,
) -> bool {
    // NOTE: must always use this over 'auth.login' directly
    let user_exists = user.exists().await;
    if user_exists {
        auth.login(user).await.unwrap();
    }
    user_exists
}

#[derive(Deserialize)]
pub struct LoginForm {
    username: String,
    password: String,
}
pub async fn login_user(
    mut auth: AuthContext,
    Form(login_form): Form<LoginForm>,
) -> impl IntoResponse {

    let user = models::User::new(login_form.username, login_form.password);

    let login_result = attempt_login(&mut auth, &user).await; 
    if login_result {
        Redirect::to(BASE_PATH)
    } else {
        Redirect::to(&(BASE_PATH.to_string() + "/login_page"))
    }
}

#[derive(Deserialize)]
pub struct SignupForm {
    display_name: String,
    username: String,
    password1: String,
    password2: String,
}
pub async fn signup_handler(
    mut auth: AuthContext,
    Form(signup_form): Form<SignupForm>,
) -> impl IntoResponse {

    if signup_form.password1 != signup_form.password2 {
        return Redirect::to(&(BASE_PATH.to_string() + "/signup_page"));
    }

    let unique_username = !models::User::username_exists(&signup_form.username).await;
    if !unique_username {
        return Redirect::to(&(BASE_PATH.to_string() + "/signup_page"));
    }

    let unique_display_name = !models::Account::display_name_exists(&signup_form.display_name).await;
    if !unique_display_name {
        return Redirect::to(&(BASE_PATH.to_string() + "/signup_page"));
    }

    let user = models::User::new(signup_form.username, signup_form.password1);
    let account = models::Account::new(signup_form.display_name, user.id.clone());

    user.add_to_db().await;
    account.add_to_db().await;


    let login_result = attempt_login(&mut auth, &user).await; 
    if login_result {
        Redirect::to(BASE_PATH)
    } else {
        Redirect::to(&(BASE_PATH.to_string() + "/signup_page"))
    }
}



#[derive(Deserialize)]
pub struct CreatePostForm {
    title: String,
    content: String,
}
pub async fn create_post(
    auth: AuthContext,
    Form(signup_form): Form<CreatePostForm>,
) -> impl IntoResponse {
    if let Some(user) = auth.current_user {

        let title = signup_form.title.trim().to_string();
        let content = signup_form.content.trim().to_string();

        // padding the date with 0s to make it sortable
        let date = utils::current_time_as_padded_string();

        let account = models::Account::from_user(&user).await;
        
        let post = models::Post::new(
            title,
            content,
            date,
            account.id,
        );
        
        post.add_to_db().await;
    }
    Redirect::to(BASE_PATH)
}


pub async fn simple_page(
    engine: AppEngine,
    Key(key): Key,
) -> impl IntoResponse {
    println!("KEY: {}", key);
    RenderHtml(key, engine, ())
}


#[derive(Deserialize)]
pub struct AddCommentForm {
    content: String,
}
pub async fn add_comment_to_post(
    auth: AuthContext,
    Path(post_id): Path<u32>,
    Form(add_comment_form): Form<AddCommentForm>,
) -> impl IntoResponse {
    let maybe_post = models::Post::maybe_from_id(post_id).await;
    if maybe_post.is_none() {
        return Redirect::to(BASE_PATH).into_response();
    }

    if let Some(user) = auth.current_user {
        let account = models::Account::from_user(&user).await;

        let content = add_comment_form.content.trim().to_string();
        let date = utils::current_time_as_padded_string();
        let comment = models::Comment::new(
            content,
            date,
            account.id,
            post_id,
            Option::None,
        );

        comment.add_to_db().await;
    }
    Redirect::to(&(BASE_PATH.to_string() + "/post/" + &post_id.to_string())).into_response()
}
pub async fn add_comment_to_comment(
    auth: AuthContext,
    Path((post_id, comment_id)): Path<(u32, u32)>,
    Form(add_comment_form): Form<AddCommentForm>,
) -> impl IntoResponse {
    let maybe_post = models::Post::maybe_from_id(post_id).await;
    if maybe_post.is_none() {
        return Redirect::to(BASE_PATH).into_response();
    }

    let maybe_comment = models::Comment::maybe_from_id(comment_id, post_id).await;
    if maybe_comment.is_none() {
        return Redirect::to(&(BASE_PATH.to_string() + "/post/" + &post_id.to_string())).into_response();
    }

    if let Some(user) = auth.current_user {
        let account = models::Account::from_user(&user).await;

        let content = add_comment_form.content.trim().to_string();
        let date = utils::current_time_as_padded_string();
        let comment = models::Comment::new(
            content,
            date,
            account.id,
            post_id,
            Option::Some(comment_id),
        );

        comment.add_to_db().await;
    }
    Redirect::to(&(BASE_PATH.to_string() + "/post/" + &post_id.to_string())).into_response()
}


pub async fn upvote_post(
    auth: AuthContext,
    Path(post_id): Path<u32>,
) -> Json<Value> {
    let maybe_post = models::Post::maybe_from_id(post_id).await;
    if maybe_post.is_none() {
        return Json(json!({"score": -1}));
    }
    let post = maybe_post.unwrap();

    if let Some(user) = auth.current_user {
        let account = models::Account::from_user(&user).await;

        let score = models::Post::vote(post_id, account.id, 1).await;
        return Json(json!({"score": score}));
    }
    Json(json!({"score": post.score}))
}
pub async fn downvote_post(
    auth: AuthContext,
    Path(post_id): Path<u32>,
) -> Json<Value> {
    let maybe_post = models::Post::maybe_from_id(post_id).await;
    if maybe_post.is_none() {
        return Json(json!({"score": -1}));
    }
    let post = maybe_post.unwrap();

    if let Some(user) = auth.current_user {
        let account = models::Account::from_user(&user).await;

        let score = models::Post::vote(post_id, account.id, -1).await;
        return Json(json!({"score": score}));
    }
    Json(json!({"score": post.score}))
}



pub async fn post_page(
    auth: AuthContext,
    engine: AppEngine,
    Key(key): Key,
    Path(post_id): Path<u32>,
) -> impl IntoResponse {
    
    let maybe_post = models::Post::maybe_from_id(post_id).await;
    if let Some(mut post) = maybe_post {

        #[derive(Serialize)]
        struct PostPageData {
            post: models::Post,
            account: models::Account,
            logged_in: bool,
            comment_tree_nodes: Vec<models::CommentTreeNode>,
        }

        post.content = post.content.replace('\n', "<br />");

        let account = models::Account::from_id(post.account_id).await;
        
        let comments = models::Comment::top_level_comments_from_post_id(post.id).await;
        
        
        let mut comment_tree_nodes = Vec::new();
        for comment in comments {
            let comment_tree_node = models::Comment::build_comment_tree(comment).await;
            comment_tree_nodes.push(comment_tree_node);
        }

        let data = PostPageData {
            post,
            account,
            logged_in: auth.current_user.is_some(),
            comment_tree_nodes,
        };
        
        RenderHtml(key, engine, data).into_response()
    } else {
        Redirect::to(BASE_PATH).into_response()
    }
}


pub async fn root(
    // State(pool): State<SqlitePool>
    auth: AuthContext,
    engine: AppEngine,
    Key(key): Key,
 ) ->  impl IntoResponse {
    println!("GET /");
    println!("key: {:?}", key);

    #[derive(Serialize)]
    pub struct RootData {
        post_datas: Vec<PostData>,
        logged_in: bool,
    }
    #[derive(Serialize)]
    pub struct PostData {
        post: models::Post,
        account: models::Account,
        comment_count: u32,
    }


    let db = sql::connect_to_db().await;

    let mut data = RootData {
        post_datas: vec![],
        logged_in: auth.current_user.is_some(),
    };

    let mut stream = sqlx::query(sql::GET_ALL_POSTS_SQL)
        .map(|row: SqliteRow| {
            let id: u32 = row.get(0);
            let title: String = row.get(1);

            let full_content: String = row.get(2);

            let first_four_lines = full_content.lines().take(4).collect::<Vec<&str>>().join("<br />");
            let content_str = &first_four_lines[0..first_four_lines.len().min(500)];
            
            let content = if content_str.len() < full_content.len() {
                content_str.to_string() + "..."
            } else {
                content_str.to_string()
            };
            
            let date: String = row.get(3);
            let score: i32 = row.get(4);
            let account_id: u32 = row.get(5);

            models::Post {
                id,
                title,
                content,
                date,
                score,
                account_id,
            }
        })
        .fetch(&db);

    while let Some(row) = stream.next().await {
        let post = row.unwrap();
        let account = models::Account::from_id(post.account_id).await;
        let comment_count = post.count_comments().await;
        data.post_datas.push(PostData { post, account, comment_count });
    }


    RenderHtml(key, engine, data)
}