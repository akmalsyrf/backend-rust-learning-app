use axum::{routing::post, Router};

use crate::application::state::AppState;
use crate::presentation::handlers::auth_handlers::{login_handler, register_handler};

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
}
