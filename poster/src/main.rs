use axum::{
    routing::get, Router, extract::State,
};

// use serde::{Deserialize, Serialize};

use std::net::SocketAddr;

use sqlx::{sqlite::{SqlitePoolOptions, SqliteRow}, SqlitePool, Row};
use futures_util::StreamExt;

const PORT: u16 = 3000;

const DB_PATH: &str = "db.sqlite";

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // database
    // let db = sqlx::SqlitePool::connect(DB_PATH).await.unwrap();
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(DB_PATH)
        .await
        .unwrap();

    create_database().await;

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        // .route("/users", post(create_user))

        .route("/joe", get(joe))



        .with_state(pool);



    let addr = SocketAddr::from(([127, 0, 0, 1], PORT));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn create_database() {
    let db = sqlx::SqlitePool::connect(DB_PATH).await.unwrap();
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS temp_table (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            x INTEGER NOT NULL
        )
        "#,
    )
    .execute(&db)
    .await
    .unwrap();
}

async fn joe(
    State(pool): State<SqlitePool>,
) -> &'static str {
    println!("GET /joe");

    sqlx::query("INSERT INTO temp_table (x) VALUES (?)")
        .bind(7)
        .execute(&pool).await.unwrap();

    "Joe"
}

// basic handler that responds with a static string
async fn root(
    State(pool): State<SqlitePool>,
) -> String {
    println!("GET /");

    let mut output = "".to_string();

    let mut stream = sqlx::query("SELECT * FROM temp_table")
        .map(|row: SqliteRow| {
            // map the row into a user-defined domain type
            let id: i64 = row.try_get("id").unwrap();
            let x: i64 = row.try_get("x").unwrap();

            format!("{}: {}", id, x)

        })
        .fetch(&pool);

    while let Some(row) = stream.next().await {
        output = format!("{}\n{}", output, row.unwrap());
    }

    "Hello, World!".to_string() + &output
}

// async fn create_user(
//     // this argument tells axum to parse the request body
//     // as JSON into a `CreateUser` type
//     Json(payload): Json<CreateUser>,
// ) -> (StatusCode, Json<User>) {
//     // insert your application logic here
//     let user = User {
//         id: 1337,
//         username: payload.username,
//     };

//     // this will be converted into a JSON response
//     // with a status code of `201 Created`
//     (StatusCode::CREATED, Json(user))
// }

// // the input to our `create_user` handler
// #[derive(Deserialize)]
// struct CreateUser {
//     username: String,
// }

// // the output to our `create_user` handler
// #[derive(Serialize)]
// struct User {
//     id: u64,
//     username: String,
// }