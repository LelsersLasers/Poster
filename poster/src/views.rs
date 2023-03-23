use crate::*;


pub async fn logout_handler(mut auth: AuthContext) -> impl IntoResponse {
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
pub async fn login_handler(
    mut auth: AuthContext,
    Form(login_form): Form<LoginForm>,
) -> impl IntoResponse {

    let user = models::User::new(login_form.username, login_form.password);

    let login_result = attempt_login(&mut auth, &user).await; 
    if login_result {
        Redirect::to(&(BASE_PATH.to_string() + "/protected"))
    } else {
        Redirect::to(&(BASE_PATH.to_string() + "/login"))
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
        return Redirect::to(&(BASE_PATH.to_string() + "/signup"));
    }

    let unique_username = !models::User::username_exists(&signup_form.username).await;
    if !unique_username {
        return Redirect::to(&(BASE_PATH.to_string() + "/signup"));
    }

    let unique_display_name = !models::Account::display_name_exists(&signup_form.display_name).await;
    if !unique_display_name {
        return Redirect::to(&(BASE_PATH.to_string() + "/signup"));
    }

    let user = models::User::new(signup_form.username, signup_form.password1);
    let account = models::Account::new(signup_form.display_name, user.id.clone());

    user.add_to_db().await;
    account.add_to_db().await;


    let login_result = attempt_login(&mut auth, &user).await; 
    if login_result {
        Redirect::to(&(BASE_PATH.to_string() + "/protected"))
    } else {
        Redirect::to(&(BASE_PATH.to_string() + "/signup"))
    }
}

// pub async fn login(
//     engine: AppEngine,
//     Key(key): Key,
// ) -> impl IntoResponse {
//     RenderHtml(key, engine, ())
// }
// pub async fn signup(
//     engine: AppEngine,
//     Key(key): Key,
// ) -> impl IntoResponse {
//     RenderHtml(key, engine, ())
// }

pub async fn simple_page(
    engine: AppEngine,
    Key(key): Key,
) -> impl IntoResponse {
    RenderHtml(key, engine, ())
}


pub async fn protected_handler(
    // Extension(user): Extension<models::User>
    auth: AuthContext
) -> impl IntoResponse {
    if let Some(user) = auth.current_user {
        let account = models::Account::from_user(&user).await;
        format!("Logged in as: {}", account.display_name).into_response()
    } else {
        Redirect::to(BASE_PATH).into_response()
    }
}

pub async fn joe() -> &'static str {
    // State(pool): State<SqlitePool>
    println!("GET /joe");

    let db = sql::connect_to_db().await;

    sqlx::query("INSERT INTO temp_table (x) VALUES (?);")
        .bind(7)
        .execute(&db)
        .await
        .unwrap();

    "Joe"
}

pub async fn root(
    // State(pool): State<SqlitePool>
    auth: AuthContext,
    engine: AppEngine,
    Key(key): Key,
 ) ->  impl IntoResponse {
    println!("GET /");
    println!("key: {:?}", key);

    #[derive(Debug, Serialize)]
    pub struct RootData {
        // ids_and_xs: Vec<(i64, i64)>,
        // xs: Vec<i64>
        ids_and_xs: Vec<IdAndX>,
        logged_in: bool,
    }


    #[derive(Debug, Serialize)]
    pub struct IdAndX {
        id: i64,
        x: i64,
    }
    let db = sql::connect_to_db().await;

    let mut data = RootData {
        ids_and_xs: Vec::new(),
        logged_in: auth.current_user.is_some() 
    };

    let mut stream = sqlx::query("SELECT * FROM temp_table;")
        .map(|row: SqliteRow| {
            // map the row into a user-defined domain type
            let id: i64 = row.try_get("id").unwrap();
            let x: i64 = row.try_get("x").unwrap();

            IdAndX { id, x }
        })
        .fetch(&db);

    while let Some(row) = stream.next().await {
        // let row = row.unwrap();
        data.ids_and_xs.push(row.unwrap());
        // data.ids.push(row.0);
        // data.xs.push(row.1);
    }


    RenderHtml(key, engine, data)
}