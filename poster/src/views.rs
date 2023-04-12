use crate::*;


pub async fn logout(
    mut session: WritableSession,
) -> impl IntoResponse {    
    session.remove("current_user");
    session.remove("login_context");
    session.remove("signup_context");

    Redirect::to(&(BASE_PATH.to_string() + "/back"))
}


pub async fn attempt_login(
    pool: &SqlitePool,
    session: &mut WritableSession,
    user: &models::User,
) -> bool {
    let user_exists = user.exists(pool).await;
    if user_exists {
        session.insert("current_user", user).unwrap();
    }
    user_exists
}

#[derive(Serialize, Deserialize)]
pub struct LoginContext {
    error: String,
    attempted_username: String,
    attempted_password: String,
}
#[derive(Deserialize)]
pub struct LoginForm {
    username: String,
    password: String,
}
pub async fn login_handler(
    State(app_state): State<AppState>,
    mut session: WritableSession,
    Form(login_form): Form<LoginForm>,
) -> impl IntoResponse {
    let pool = &app_state.pool;

    if session.get_raw("current_user").is_some() {
        return Redirect::to(BASE_PATH);
    }

    let user = models::User::new(login_form.username.clone(), login_form.password.clone());

    let login_result = attempt_login(pool, &mut session, &user).await; 
    if login_result {
        Redirect::to(&(BASE_PATH.to_string() + "/back"))
    } else {
        let login_context = LoginContext {
            error: "Invalid username or password".to_string(),
            attempted_username: login_form.username,
            attempted_password: login_form.password,
        };
        session.insert("login_context", login_context).unwrap();
        Redirect::to(&(BASE_PATH.to_string() + "/login_page"))
    }
}

#[derive(Serialize, Deserialize)]
pub struct SignupContext {
    error: String,
    attempted_display_name: String,
    attempted_username: String,
    attempted_password1: String,
    attempted_password2: String,
}
#[derive(Deserialize)]
pub struct SignupForm {
    display_name: String,
    username: String,
    password1: String,
    password2: String,
}
pub async fn signup_handler(
    State(app_state): State<AppState>,
    mut session: WritableSession,
    Form(signup_form): Form<SignupForm>,
) -> impl IntoResponse {
    let pool = &app_state.pool;

    if session.get_raw("current_user").is_some() {
        return Redirect::to(BASE_PATH);
    }

    let mut signup_context = SignupContext {
        error: "".to_string(),
        attempted_display_name: signup_form.display_name.clone(),
        attempted_username: signup_form.username.clone(),
        attempted_password1: signup_form.password1.clone(),
        attempted_password2: signup_form.password2.clone(),
    };

    if signup_form.password1 != signup_form.password2 {
        signup_context.error = "Passwords do not match".to_string();
        session.insert("signup_context", signup_context).unwrap();
        return Redirect::to(&(BASE_PATH.to_string() + "/signup_page"));
    }

    let unique_username = !models::User::username_exists(&signup_form.username, pool).await;
    if !unique_username {
        signup_context.error = "Username already exists".to_string();
        session.insert("signup_context", signup_context).unwrap();
        return Redirect::to(&(BASE_PATH.to_string() + "/signup_page"));
    }

    let unique_display_name = !models::Account::display_name_exists(&signup_form.display_name, pool).await;
    if !unique_display_name {
        signup_context.error = "Display name already exists".to_string();
        session.insert("signup_context", signup_context).unwrap();
        return Redirect::to(&(BASE_PATH.to_string() + "/signup_page"));
    }

    let user = models::User::new(signup_form.username, signup_form.password1);
    let account = models::Account::new(signup_form.display_name, user.id.clone());

    user.add_to_db(pool).await;
    account.add_to_db(pool).await;


    let _login_result = attempt_login(pool, &mut session, &user).await; // should always be true
    Redirect::to(&(BASE_PATH.to_string() + "/back"))
}



#[derive(Deserialize)]
pub struct CreatePostForm {
    title: String,
    content: String,
}
pub async fn create_post(
    State(app_state): State<AppState>,
    session: WritableSession,
    Form(signup_form): Form<CreatePostForm>,
) -> impl IntoResponse {
    let pool = &app_state.pool;

    if let Some(user) = session.get::<models::User>("current_user") {

        let title = signup_form.title.trim().to_string();
        let content = signup_form.content.trim().to_string();

        // padding the date with 0s to make it sortable
        let date = utils::current_time_as_padded_string();

        let account = models::Account::from_user(&user, pool).await;
        
        let post = models::Post::new(
            title,
            content,
            date,
            account.id,
        );
        
        let post_id = post.add_to_db(pool).await;
        Redirect::to(&(BASE_PATH.to_string() + "/post/" + &post_id.to_string()))
    } else {
        Redirect::to(BASE_PATH)
    }
}


pub async fn create_post_page(
    engine: AppEngine,
    Key(key): Key,
    session: WritableSession
) -> impl IntoResponse {
    if session.get_raw("current_user").is_none() {
        Redirect::to(BASE_PATH).into_response().into_response()
    } else {
        RenderHtml(key, engine, ()).into_response()
    }
}

pub async fn signup_page(
    session: WritableSession,
    engine: AppEngine,
    Key(key): Key,
) -> impl IntoResponse {

    if session.get_raw("current_user").is_some() {
        Redirect::to(BASE_PATH).into_response().into_response()
    } else {
        let signup_context = session.get::<SignupContext>("signup_context").unwrap_or(SignupContext {
            error: "".to_string(),
            attempted_display_name: "".to_string(),
            attempted_username: "".to_string(),
            attempted_password1: "".to_string(),
            attempted_password2: "".to_string(),
        });
        RenderHtml(key, engine, signup_context).into_response()
    }
}

pub async fn login_page(
    session: WritableSession,
    engine: AppEngine,
    Key(key): Key,
) -> impl IntoResponse {

    if session.get_raw("current_user").is_some() {
        Redirect::to(BASE_PATH).into_response().into_response()
    } else {
        let login_context = session.get::<LoginContext>("login_context").unwrap_or(LoginContext {
            error: "".to_string(),
            attempted_username: "".to_string(),
            attempted_password: "".to_string(),
        });
        RenderHtml(key, engine, login_context).into_response()
    }
}


#[derive(Deserialize)]
pub struct AddCommentForm {
    content: String,
}
pub async fn add_comment_to_post(
    State(app_state): State<AppState>,
    session: WritableSession,
    Path(post_id): Path<u32>,
    Form(add_comment_form): Form<AddCommentForm>,
) -> impl IntoResponse {
    let pool = &app_state.pool;

    let maybe_post = models::Post::maybe_from_id(post_id, pool).await;
    if maybe_post.is_none() {
        return Redirect::to(BASE_PATH).into_response();
    }

    if let Some(user) = session.get::<models::User>("current_user") {
        let account = models::Account::from_user(&user, pool).await;

        let content = add_comment_form.content.trim().to_string();
        let date = utils::current_time_as_padded_string();
        let comment = models::Comment::new(
            content,
            date,
            account.id,
            post_id,
            Option::None,
        );

        comment.add_to_db(pool).await;
    }
    Redirect::to(&(BASE_PATH.to_string() + "/post/" + &post_id.to_string())).into_response()
}
pub async fn add_comment_to_comment(
    State(app_state): State<AppState>,
    session: WritableSession,
    Path((post_id, comment_id)): Path<(u32, u32)>,
    Form(add_comment_form): Form<AddCommentForm>,
) -> impl IntoResponse {
    let pool = &app_state.pool;

    let maybe_post = models::Post::maybe_from_id(post_id, pool).await;
    if maybe_post.is_none() {
        return Redirect::to(BASE_PATH).into_response();
    }

    let maybe_comment = models::Comment::maybe_from_id(comment_id, post_id, pool).await;
    if maybe_comment.is_none() {
        return Redirect::to(&(BASE_PATH.to_string() + "/post/" + &post_id.to_string())).into_response();
    }

    if let Some(user) = session.get::<models::User>("current_user") {
        let account = models::Account::from_user(&user, pool).await;

        let content = add_comment_form.content.trim().to_string();
        let date = utils::current_time_as_padded_string();
        let comment = models::Comment::new(
            content,
            date,
            account.id,
            post_id,
            Option::Some(comment_id),
        );

        comment.add_to_db(pool).await;
    }
    Redirect::to(&(BASE_PATH.to_string() + "/post/" + &post_id.to_string())).into_response()
}


pub async fn upvote_post(
    State(app_state): State<AppState>,
    session: WritableSession,
    Path(post_id): Path<u32>,
) -> Json<Option<models::PostData>> {
    let pool = &app_state.pool;

    let maybe_post = models::Post::maybe_from_id(post_id, pool).await;
    if maybe_post.is_none() {
        return Json(None);
    }
    let unchanged_post = maybe_post.unwrap();

    if let Some(user) = &session.get::<models::User>("current_user") {
        let account = models::Account::from_user(user, pool).await;

        models::Post::vote(post_id, account.id, 1, pool).await;
        let post = models::Post::maybe_from_id(post_id, pool).await.unwrap();
        let post_data = post.into_post_data(&session, pool).await;
        return Json(Some(post_data));
    }

    let post_data = unchanged_post.into_post_data(&session, pool).await;
    Json(Some(post_data))
}
pub async fn downvote_post(
    State(app_state): State<AppState>,
    session: WritableSession,
    Path(post_id): Path<u32>,
) -> Json<Option<models::PostData>> {
    let pool = &app_state.pool;

    let maybe_post = models::Post::maybe_from_id(post_id, pool).await;
    if maybe_post.is_none() {
        return Json(None);
    }
    let unchanged_post = maybe_post.unwrap();

    if let Some(user) = &session.get::<models::User>("current_user") {
        let account = models::Account::from_user(user, pool).await;

        models::Post::vote(post_id, account.id, -1, pool).await;
        let post = models::Post::maybe_from_id(post_id, pool).await.unwrap();
        let post_data = post.into_post_data(&session, pool).await;
        return Json(Some(post_data));
    }

    let post_data = unchanged_post.into_post_data(&session, pool).await;
    Json(Some(post_data))
}

pub async fn upvote_comment(
    State(app_state): State<AppState>,
    session: WritableSession,
    Path((post_id, comment_id)): Path<(u32, u32)>,
) -> Json<Value> {
    let pool = &app_state.pool;

    let maybe_post = models::Post::maybe_from_id(post_id, pool).await;
    let maybe_comment = models::Comment::maybe_from_id(comment_id, post_id, pool).await;
    if maybe_post.is_none() || maybe_comment.is_none() {
        return Json(json!({
            "score": -1,
            "vote_value": -2
        }));
    }
    let comment = maybe_comment.unwrap();

    if let Some(user) = session.get::<models::User>("current_user") {
        let account = models::Account::from_user(&user, pool).await;

        let score = models::Comment::vote(comment_id, post_id, account.id, 1, pool).await;
        let vote_value = models::Comment::get_vote_value(comment_id, post_id, &session, pool).await;
        return Json(json!( {
            "score": score,
            "vote_value": vote_value
        }));
    }
    Json(json!({
        "score": comment.score,
        "vote_value": -2
    }))
}
pub async fn downvote_comment(
    State(app_state): State<AppState>,
    session: WritableSession,
    Path((post_id, comment_id)): Path<(u32, u32)>,
) -> Json<Value> {
    let pool = &app_state.pool;

    let maybe_post = models::Post::maybe_from_id(post_id, pool).await;
    let maybe_comment = models::Comment::maybe_from_id(comment_id, post_id, pool).await;
    if maybe_post.is_none() || maybe_comment.is_none() {
        return Json(json!({
            "score": -1,
            "vote_value": -2
        }));
    }
    let comment = maybe_comment.unwrap();

    if let Some(user) = session.get::<models::User>("current_user") {
        let account = models::Account::from_user(&user, pool).await;

        let score = models::Comment::vote(comment_id, post_id, account.id, -1, pool).await;
        let vote_value = models::Comment::get_vote_value(comment_id, post_id, &session, pool).await;
        return Json(json!( {
            "score": score,
            "vote_value": vote_value
        }));
    }
    Json(json!({
        "score": comment.score,
        "vote_value": -2
    }))
}



pub async fn post_page(
    mut session: WritableSession,
    State(app_state): State<AppState>,
    Key(key): Key,
    Path(post_id): Path<u32>,
) -> impl IntoResponse {
    let engine = app_state.engine;
    let pool = &app_state.pool;

    let maybe_post = models::Post::maybe_from_id(post_id, pool).await;
    if let Some(mut post) = maybe_post {

        #[derive(Serialize)]
        struct PostPageData {
            post_data: models::PostData,
            logged_in: bool,
            comment_tree_nodes: Vec<models::CommentTreeNode>,
        }

        post.content = post.content.replace('\n', "<br />");
        
        let comments = models::Comment::top_level_comments_from_post_id(post.id, pool).await;
        
        
        let mut comment_tree_nodes = Vec::new();
        for comment in comments {
            let comment_tree_node = models::Comment::build_comment_tree(comment, &session, pool).await;
            comment_tree_nodes.push(comment_tree_node);
        }

        comment_tree_nodes.sort_by(|a, b| {
            let a_score = a.comment.score;
            let b_score = b.comment.score;
            if a_score == b_score {
                b.comment.date.cmp(&a.comment.date)
            } else {
                b_score.cmp(&a_score)
            }
        });

        let post_data = post.into_post_data(&session, pool).await;


        let data = PostPageData {
            post_data,
            logged_in: session.get_raw("current_user").is_some(),
            comment_tree_nodes,
        };

        session.insert("back_url", "/post/".to_string() + &post_id.to_string()).unwrap();
        
        RenderHtml(key, engine, data).into_response()
    } else {
        Redirect::to(BASE_PATH).into_response()
    }
}



pub async fn get_posts(
    State(app_state): State<AppState>,
    session: WritableSession,
) -> Json<Vec<models::PostData>> {
    let pool = &app_state.pool;
    let mut post_datas = Vec::new();

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
        .fetch(pool);

    while let Some(row) = stream.next().await {
        let post = row.unwrap();
        let post_data = post.into_post_data(&session, pool).await;
        post_datas.push(post_data);
    }

    post_datas.sort_by(|a, b| {
        let a_score = a.post.score;
        let b_score = b.post.score;
        if a_score == b_score {
            b.post.date.cmp(&a.post.date)
        } else {
            b_score.cmp(&a_score)
        }
    });

    Json(post_datas)
}
pub async fn root(
    mut session: WritableSession,
    engine: AppEngine,
    Key(key): Key,
 ) ->  impl IntoResponse {

    #[derive(Serialize)]
    pub struct IndexData {
        logged_in: bool,
    }

    let data = IndexData {
        logged_in: session.get_raw("current_user").is_some(),
    };

    session.insert("back_url", "").unwrap();

    RenderHtml(key, engine, data)
}


pub async fn back(
    session: WritableSession,
) -> Redirect {
    if let Some(back_url) = session.get::<String>("back_url") {
        Redirect::to(&(BASE_PATH.to_string() + &back_url))
    } else {
        Redirect::to(BASE_PATH)
    }
}

