use axum::{extract::State, response::IntoResponse, routing::get, Extension, Router};

// use axum_login::{
//     axum_sessions::{async_session::MemoryStore as SessionMemoryStore, SessionLayer},
//     secrecy::SecretVec,
//     AuthLayer, AuthUser, RequireAuthorizationLayer, SqliteStore,
// };
// use rand::Rng;
// use std::{collections::HashMap, sync::Arc};
// use tokio::sync::RwLock;

use axum_login::{
    axum_sessions::{async_session::MemoryStore, SessionLayer},
    secrecy::SecretVec,
    AuthLayer, AuthUser, RequireAuthorizationLayer, SqliteStore,
};
use rand::Rng;

// use serde::{Deserialize, Serialize};

use std::net::SocketAddr;

use futures_util::StreamExt;
use sqlx::{
    sqlite::{SqlitePoolOptions, SqliteRow},
    Row, SqlitePool,
};

mod models;

// type AuthContext = axum_login::extractors::AuthContext<i64, models::User, SqliteStore<models::User>>;

type AuthContext = axum_login::extractors::AuthContext<models::User, SqliteStore<models::User>>;

const PORT: u16 = 3000;

const DB_PATH: &str = "db.sqlite";

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // database
    // let db = sqlx::SqlitePool::connect(DB_PATH).await.unwrap();
    // let pool = SqlitePoolOptions::new()
    //     .max_connections(5)
    //     .connect(DB_PATH)
    //     .await
    //     .unwrap();

    let secret = rand::thread_rng().gen::<[u8; 64]>();

    let session_store = MemoryStore::new();
    let session_layer = SessionLayer::new(session_store, &secret).with_secure(false);

    let pool = SqlitePoolOptions::new()
        .connect(DB_PATH)
        .await
        .unwrap();

    let user_store = SqliteStore::<models::User>::new(pool);
    let auth_layer = AuthLayer::new(user_store, &secret);

    create_database().await;

    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .route("/joe", get(joe))


        // .route_layer(RequireAuthorizationLayer::<models::User>::login())
        
        .layer(auth_layer)
        .layer(session_layer)
        ;
        // .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], PORT));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn logout_handler(mut auth: AuthContext) {
    dbg!("Logging out user: {}", &auth.current_user);
    auth.logout().await;
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

async fn joe() -> &'static str {
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

// basic handler that responds with a static string
async fn root(
    // State(pool): State<SqlitePool>
) -> String {
    println!("GET /");

    let db = sqlx::SqlitePool::connect(DB_PATH).await.unwrap();

    let mut output = "".to_string();

    let mut stream = sqlx::query("SELECT * FROM temp_table")
        .map(|row: SqliteRow| {
            // map the row into a user-defined domain type
            let id: i64 = row.try_get("id").unwrap();
            let x: i64 = row.try_get("x").unwrap();

            format!("{}: {}", id, x)
        })
        .fetch(&db);

    while let Some(row) = stream.next().await {
        output = format!("{}\n{}", output, row.unwrap());
    }

    "Hello, World!".to_string() + &output
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