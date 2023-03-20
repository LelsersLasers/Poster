use axum::{
    // extract::State,
    response::IntoResponse, routing::get, Extension, Router,
    response::Redirect,
    extract::{FromRef, Path},
    ServiceExt
};
use tower::layer::Layer;
use tower_http::normalize_path::NormalizePathLayer;



use axum_login::{
    axum_sessions::{async_session::MemoryStore, SessionLayer},
    // secrecy::SecretVec,
    AuthLayer,
    // AuthUser,
    RequireAuthorizationLayer, SqliteStore,
};
use rand::Rng;
type AuthContext = axum_login::extractors::AuthContext<models::User, SqliteStore<models::User>>;


use axum_template::{engine::Engine, Key, RenderHtml};
use minijinja::{Environment, Source};
use serde::Serialize;
type AppEngine = Engine<Environment<'static>>;
// use serde::{Deserialize, Serialize};


use std::net::SocketAddr;

use futures_util::StreamExt;
use sqlx::{
    sqlite::{SqlitePoolOptions, SqliteRow},
    Row,
    // SqlitePool,
};



mod models;
mod utils;

const PORT: u16 = 3000;
const DB_PATH: &str = "db.sqlite";

const BASE_PATH: &str = "/poster";
const TEMPLATE_PATH: &str = "templates/";
const SQL_PATH: &str = "sql/";


#[derive(Clone, FromRef)]
struct AppState {
    engine: AppEngine,
}




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


    // Set up the Handlebars engine with the same route paths as the Axum router
    // let mut jinja = Environment::new();
    // jinja
    //     .add_template("/:name", "<h1>Hello Minijinja!</h1><p>{{name}}</p>")
    //     .unwrap();

    let mut env = Environment::new();
    let mut source = Source::new();
    source
        .add_template(
            BASE_PATH.to_string() + "/",
            utils::read_file(&(TEMPLATE_PATH.to_string() + "index.html"))
        ).unwrap();
    env.set_source(source);

    let engine = Engine::from(env);
    // let app_state = AppState { engine };



    // build our application with a route
    let routes = Router::new()

            .route("/protected", get(protected_handler))
        // affects every route above it
        .route_layer(RequireAuthorizationLayer::<models::User>::login())

        // .route("/protected", get(|| async { Redirect::to(BASE_PATH) }))

        .route("/", get(root))
        .route("/joe", get(joe))
        
        .route("/logout", get(logout_handler))

        
        .layer(auth_layer)
        .layer(session_layer)
        .with_state(AppState { engine })
        ;
        // .with_state(pool);

    let all_routes = Router::new()
        .nest(BASE_PATH, routes)
        .route("/", get(|| async { Redirect::to(BASE_PATH) }));

    let app = NormalizePathLayer::trim_trailing_slash().layer(all_routes);

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

async fn protected_handler(Extension(user): Extension<models::User>) -> impl IntoResponse {
    format!("Logged in as: {}", user.display_name)
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

async fn root(
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