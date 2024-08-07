use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::route::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/all", get(self::get_all))
        .route("/new", get(self::new))
        .route("/new", post(self::create))
        .route("/edit/:id", get(self::edit))
        .route("/edit/:id", post(self::update))
        .route("/:id", get(self::get_one))
}

type BlogPost = serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NewBlogPost {}

pub async fn get_all() -> impl IntoResponse {
    Json(vec![blogpost1(), blogpost2()])
}

pub async fn get_one(State(state): State<AppState>, Path(id): Path<usize>) -> impl IntoResponse {
    Json(blogpost1())
}

pub async fn new() -> impl IntoResponse {
    Html("<h1>New post form</h1>")
}

pub async fn create(Json(post): Json<NewBlogPost>) -> impl IntoResponse {
    println!("Добавился пост: {post:?}");
    StatusCode::CREATED
}

pub async fn edit() -> impl IntoResponse {
    Html("<h1>Edit post form</h1>")
}

pub async fn update(Json(post): Json<NewBlogPost>) -> impl IntoResponse {
    println!("Обновился пост: {post:?}");
    StatusCode::OK
}

fn blogpost1() -> BlogPost {
    json!({
        "id": 1,
        "title": "Hello, world!",
        "body": "This is my first blog post."
    })
}

fn blogpost2() -> BlogPost {
    json!({
        "id": 2,
        "title": "Another blog post",
        "body": "This is my second blog post."
    })
}
