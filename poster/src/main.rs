use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, post},
    Router,
    response::{Redirect, Json, Response},
    extract::FromRef,
    ServiceExt,
    Form, middleware::{self, Next},
    http::Request
};
use tower::layer::Layer;
use tower_http::normalize_path::NormalizePathLayer;

use serde_json::{Value, json};


use axum_sessions::{
    async_session::MemoryStore,
    extractors::WritableSession,
    SessionLayer,
};

use rand::Rng;


use axum_template::{engine::Engine, Key, RenderHtml};
use minijinja::{Environment, Source};
type AppEngine = Engine<Environment<'static>>;
use serde::{Deserialize, Serialize};


use std::net::SocketAddr;

use futures_util::StreamExt;
use sqlx::{
    sqlite::{
        SqlitePoolOptions,
        SqliteRow
    },
    Row,
    SqlitePool,
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
pub struct AppState {
    engine: AppEngine,
    pool: SqlitePool,
}




#[tokio::main]
async fn main() {
    // database
    // let db = sql::connect_to_db().await;

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(DB_PATH)
        .await
        .unwrap();

    let store = MemoryStore::new();
    let secret = rand::thread_rng().gen::<[u8; 128]>();
    let session_layer = SessionLayer::new(store, &secret).with_secure(false);


    create_tables(&pool).await;


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


    let routes = Router::new()

        .route("/", get(views::root)) // all
        .route("/get_posts", get(views::get_posts)) // all

        
        .route("/login_page", get(views::login_page)) // if not logged in
        .route("/signup_page", get(views::signup_page)) // if not logged in

        .route("/login_handler", post(views::login_handler)) // if not logged in
        .route("/signup_handler", post(views::signup_handler)) // if not logged in

        .route("/logout", get(views::logout)) // if logged in


        .route("/create_post_page", get(views::create_post_page)) // if logged in
        .route("/create_post", post(views::create_post)) // if logged in
        

        .route("/post/:post_id", get(views::post_page)) // all


        .route("/add_comment_to_post/:post_id", post(views::add_comment_to_post)) // if logged in
        .route("/add_comment_to_comment/:post_id/:comment_id", post(views::add_comment_to_comment)) // if logged in


        .route("/upvote_post/:post_id", get(views::upvote_post)) // if logged in
        .route("/downvote_post/:post_id", get(views::downvote_post)) // if logged in

        .route("/upvote_comment/:post_id/:comment_id", get(views::upvote_comment)) // if logged in
        .route("/downvote_comment/:post_id/:comment_id", get(views::downvote_comment)) // if logged in

        .route("/back", get(views::back)) // all
        
        
        .layer(session_layer)
        .with_state(AppState { engine, pool });

    let all_routes = Router::new()
        .route("/", get(|| async { Redirect::to(BASE_PATH) })) // TODO: trim trailing slash -> "" instead of "/"
        .nest(BASE_PATH, routes)
        .fallback(get(|| async { Redirect::to(BASE_PATH) })) // TODO: I don't think this is what I want
        .layer(middleware::from_fn(logging_middleware))
        ;

    let app = NormalizePathLayer::trim_trailing_slash().layer(all_routes);

    let addr = SocketAddr::from(([127, 0, 0, 1], PORT));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


async fn logging_middleware<B>(
    request: Request<B>,
    next: Next<B>,
) -> Response {

    let date = chrono::Local::now();
    let formated_date = date.format("%Y-%m-%d %H:%M:%S");
    println!("{}\t{:6}\t{}", formated_date, request.method(), request.uri());

    next.run(request).await
}


async fn create_tables(pool: &SqlitePool) {
    // let db = sql::connect_to_db().await;
    sqlx::query(&utils::read_file(&(SQL_PATH.to_string() + "makeTables.sql")))
        .execute(pool)
        .await
        .unwrap();
}