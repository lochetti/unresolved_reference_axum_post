mod foo;

use axum::http::StatusCode;
use axum::Router;
use axum::routing::{get, post};

pub async fn initialize() -> Router {
    Router::new()
        .route("/", get(get_ok))
        .route("/", post(post_ok))
}

async fn get_ok() -> &'static str {
    "get ok"
}

async fn post_ok() -> StatusCode {
    foo::post().await
}