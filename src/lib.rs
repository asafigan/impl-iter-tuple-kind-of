use axum::{response::IntoResponse, routing::get, Router};

pub fn build_app() -> Router {
    Router::new().route("/", get(index))
}

async fn index() -> impl IntoResponse {
    "Hello, World"
}
