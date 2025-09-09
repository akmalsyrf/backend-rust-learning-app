use axum::Router;

use crate::application::state::AppState;
use crate::presentation::api::{
    auth_routes::auth_routes, content_routes::content_routes, progress_routes::progress_routes,
};

pub fn api_routes() -> Router<AppState> {
    Router::new()
        // Authentication API routes
        .nest("/api/auth", auth_routes())
        // Content API routes
        .nest("/api/content", content_routes())
        // Progress API routes
        .nest("/api/progress", progress_routes())
}
