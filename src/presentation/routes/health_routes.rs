use axum::routing::get;
use axum::Router;

use crate::application::state::AppState;

async fn health_check() -> &'static str {
    "OK"
}

pub fn health_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(health_check))
        .route("/health", get(health_check))
}
