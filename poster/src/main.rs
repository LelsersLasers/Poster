use axum::{
    extract::Path,
    response::IntoResponse,
    routing::{get, post},
    // Extension,
    Router,
    response::{Redirect, Json},
    extract::FromRef,
    ServiceExt,
    Form,
};
use tower::layer::Layer;
use tower_http::normalize_path::NormalizePathLayer;

use serde_json::{Value, json};


use axum_login::{
    axum_sessions::{async_session::MemoryStore, SessionLayer},
    secrecy::SecretVec,
    AuthLayer,
    AuthUser,
    // RequireAuthorizationLayer,
    SqliteStore,
};
use rand::Rng;
type AuthContext = axum_login::extractors::AuthContext<models::User, SqliteStore<models::User>>;


use axum_template::{engine::Engine, Key, RenderHtml};
use minijinja::{Environment, Source};
type AppEngine = Engine<Environment<'static>>;
use serde::{Deserialize, Serialize};


use std::net::SocketAddr;

use futures_util::StreamExt;
use sqlx::{
    sqlite::{SqlitePoolOptions, SqliteRow},
    Row,
    // SqlitePool,
};


use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

use std::time::{SystemTime, UNIX_EPOCH};

use async_recursion::async_recursion;


mod models;
mod utils;
mod views;
mod sql;



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
    // database
    // let db = sql::connect_to_db().await;
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


    create_tables().await;



    let mut env = Environment::new();
    let mut source = Source::new();
    source
        .add_template(
            TEMPLATE_PATH.to_string() +  "base.html",
            utils::read_file(&(TEMPLATE_PATH.to_string() + "base.html"))
        ).unwrap();
    source
        .add_template(
            TEMPLATE_PATH.to_string() +  "base_form.html",
            utils::read_file(&(TEMPLATE_PATH.to_string() + "base_form.html"))
        ).unwrap();
    source
        .add_template(
            TEMPLATE_PATH.to_string() +  "base_navbar.html",
            utils::read_file(&(TEMPLATE_PATH.to_string() + "base_navbar.html"))
        ).unwrap();
    source
        .add_template(
            TEMPLATE_PATH.to_string() +  "base_vote.html",
            utils::read_file(&(TEMPLATE_PATH.to_string() + "base_vote.html"))
        ).unwrap();

    source
        .add_template(
            BASE_PATH.to_string() + "/",
            utils::read_file(&(TEMPLATE_PATH.to_string() + "index_page.html"))
        ).unwrap();
    source
        .add_template(
            BASE_PATH.to_string() + "/login_page",
            utils::read_file(&(TEMPLATE_PATH.to_string() + "login_page.html"))
        ).unwrap();
    source
        .add_template(
            BASE_PATH.to_string() + "/signup_page",
            utils::read_file(&(TEMPLATE_PATH.to_string() + "signup_page.html"))
        ).unwrap();
    source
        .add_template(
            BASE_PATH.to_string() + "/create_post_page",
            utils::read_file(&(TEMPLATE_PATH.to_string() + "create_post_page.html"))
        ).unwrap();
    source
        .add_template(
            BASE_PATH.to_string() + "/post/:post_id",
            utils::read_file(&(TEMPLATE_PATH.to_string() + "post_page.html"))
        ).unwrap();
    env.set_source(source);

    let engine = Engine::from(env);
    // let app_state = AppState { engine };



    // build our application with a route
    let routes = Router::new()

        // affects every route above it
        // .route_layer(RequireAuthorizationLayer::<models::User>::login())

        // .route("/protected", get(|| async { Redirect::to(BASE_PATH) }))

        .route("/", get(views::root))
        .route("/get_posts", get(views::get_posts))

        
        .route("/login_page", get(views::simple_page))
        .route("/signup_page", get(views::simple_page))

        .route("/login_user", post(views::login_user))
        .route("/signup_handler", post(views::signup_handler))

        .route("/logout", get(views::logout))


        .route("/create_post_page", get(views::simple_page))
        .route("/create_post", post(views::create_post))
        

        .route("/post/:post_id", get(views::post_page))


        .route("/add_comment_to_post/:post_id", post(views::add_comment_to_post))
        .route("/add_comment_to_comment/:post_id/:comment_id", post(views::add_comment_to_comment))


        .route("/upvote_post/:post_id", get(views::upvote_post))
        .route("/downvote_post/:post_id", get(views::downvote_post))

        .route("/upvote_comment/:post_id/:comment_id", get(views::upvote_comment))
        .route("/downvote_comment/:post_id/:comment_id", get(views::downvote_comment))
        
        .layer(auth_layer)
        .layer(session_layer)
        .with_state(AppState { engine })
        ;
        // .with_state(pool);

    let all_routes = Router::new()
        .route("/", get(|| async { Redirect::to(BASE_PATH) })) // TODO
        .nest(BASE_PATH, routes)
        .fallback(get(|| async { Redirect::to(BASE_PATH) })) // TODO: I don't think this is what I want
        ;

    let app = NormalizePathLayer::trim_trailing_slash().layer(all_routes);

    let addr = SocketAddr::from(([127, 0, 0, 1], PORT));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}



async fn create_tables() {
    let db = sql::connect_to_db().await;
    sqlx::query(&utils::read_file(&(SQL_PATH.to_string() + "makeTables.sql")))
        .execute(&db)
        .await
        .unwrap();
}