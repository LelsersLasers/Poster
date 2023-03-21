use crate::*;


pub async fn logout_handler(mut auth: AuthContext) -> impl IntoResponse {
    dbg!("Logging out user: {}", &auth.current_user);
    auth.logout().await;

    Redirect::to(&(BASE_PATH.to_string() + "/"))
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

    let db = sqlx::SqlitePool::connect(DB_PATH).await.unwrap();

    let user = models::User {
        id: login_form.username,
        password_hash: login_form.password,
    };

    let stream = sqlx::query("SELECT * FROM users WHERE id = ? AND password_hash = ?")
        .bind(&user.id)
        .bind(&user.password_hash)
        .fetch_optional(&db);
    
    if let Some(_row) = stream.await.unwrap() {
        auth.login(&user).await.unwrap();
        Redirect::to(&(BASE_PATH.to_string() + "/protected"))
    } else {
        Redirect::to(&(BASE_PATH.to_string() + "/login"))
    }

    // Redirect::to(&(BASE_PATH.to_string() + "/protected"))
    // RenderHtml(key, engine, ())
}

pub async fn login(
    engine: AppEngine,
    Key(key): Key,
) -> impl IntoResponse {
    RenderHtml(key, engine, ())
}

pub async fn protected_handler(Extension(user): Extension<models::User>) -> impl IntoResponse {
    format!("Logged in as: {}", user.id)
}

pub async fn joe() -> &'static str {
    // State(pool): State<SqlitePool>
    println!("GET /joe");

    let db = sqlx::SqlitePool::connect(DB_PATH).await.unwrap();

    sqlx::query("INSERT INTO temp_table (x) VALUES (?)")
        .bind(7)
        .execute(&db)
        .await
        .unwrap();

    "Joe"
}


#[derive(Debug, Serialize)]
pub struct RootData {
    // ids_and_xs: Vec<(i64, i64)>,
    // xs: Vec<i64>
    ids_and_xs: Vec<IdAndX>,
}


#[derive(Debug, Serialize)]
pub struct IdAndX {
    id: i64,
    x: i64,
}

pub async fn root(
    // State(pool): State<SqlitePool>
    engine: AppEngine,
    Key(key): Key,
 ) ->  impl IntoResponse {
    println!("GET /");
    println!("key: {:?}", key);

    let db = sqlx::SqlitePool::connect(DB_PATH).await.unwrap();

    let mut data = RootData { ids_and_xs: Vec::new() };

    let mut stream = sqlx::query("SELECT * FROM temp_table")
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

    // "Hello, World!".to_string() + &output

    RenderHtml(key, engine, data)
}

// async fn create_user(
//     // this argument tells axum to parse the request body
//     // as JSON into a `Createmodels::User` type
//     Json(payload): Json<Createmodels::User>,
// ) -> (StatusCode, Json<models::User>) {
//     // insert your application logic here
//     let user = models::User {
//         id: 1337,
//         username: payload.username,
//     };

//     // this will be converted into a JSON response
//     // with a status code of `201 Created`
//     (StatusCode::CREATED, Json(user))
// }

// // the input to our `create_user` handler
// #[derive(Deserialize)]
// struct Createmodels::User {
//     username: String,
// }

// // the output to our `create_user` handler
// #[derive(Serialize)]
// struct models::User {
//     id: u64,
//     username: String,
// }