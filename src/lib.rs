mod v1;
mod v2;
mod v3;
mod v4;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, get_service},
    Router,
};
use tower_http::services::ServeDir;

pub fn build_app() -> Router {
    let serve_dir = get_service(ServeDir::new("public"));

    Router::new()
        .route("/", get(index))
        .nest("/v1", v1::router())
        .nest("/v2", v2::router())
        .nest("/v3", v3::router())
        .nest("/v4", v4::router())
        .nest_service("/public", serve_dir.handle_error(handle_error))
}

async fn handle_error(_err: std::io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}

async fn index() -> impl IntoResponse {
    "Hello, World"
}
