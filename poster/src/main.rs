use axum::{
    // extract::State,
    // response::IntoResponse,
    routing::get,
    // Extension,
    Router,
    response::Redirect,
    extract::FromRef,
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
mod views;



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
    // add_test_user().await;



    let mut env = Environment::new();
    let mut source = Source::new();
    source
        .add_template(
            TEMPLATE_PATH.to_string() +  "base.html",
            utils::read_file(&(TEMPLATE_PATH.to_string() + "base.html"))
        ).unwrap();
    source
        .add_template(
            BASE_PATH.to_string() + "/",
            utils::read_file(&(TEMPLATE_PATH.to_string() + "welcome.html"))
        ).unwrap();
    env.set_source(source);

    let engine = Engine::from(env);
    // let app_state = AppState { engine };



    // build our application with a route
    let routes = Router::new()

            .route("/protected", get(views::protected_handler))
        // affects every route above it
        .route_layer(RequireAuthorizationLayer::<models::User>::login())

        // .route("/protected", get(|| async { Redirect::to(BASE_PATH) }))

        .route("/", get(views::root))
        .route("/joe", get(views::joe))

        
        .route("/login", get(views::login_handler))
        .route("/logout", get(views::logout_handler))

        
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


// async fn add_test_user() {
//     let user = models::User {
//         id: 1,
//         username: "username".to_string(),
//         display_name: "Joe's Account".to_string(),
//         password_hash: "password".to_string()
//     };

    
//     let db = sqlx::SqlitePool::connect(DB_PATH).await.unwrap();
    
//     sqlx::query(&utils::read_file(&(SQL_PATH.to_string() + "addUser.sql")))
//         .bind(user.username)
//         .bind(user.display_name)
//         .bind(user.password_hash)
//         .execute(&db)
//         .await
//         .unwrap();
// }

async fn create_database() {
    let db = sqlx::SqlitePool::connect(DB_PATH).await.unwrap();
    // sqlx::query(
    //     r#"
    //     CREATE TABLE IF NOT EXISTS temp_table (
    //         id INTEGER PRIMARY KEY AUTOINCREMENT,
    //         x INTEGER NOT NULL
    //     )
    //     "#,
    // )
    //     .execute(&db)
    //     .await
    //     .unwrap();

    sqlx::query(&utils::read_file(&(SQL_PATH.to_string() + "makeUsersTable.sql")))
        .execute(&db)
        .await
        .unwrap();
}