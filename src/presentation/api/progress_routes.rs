use axum::{routing::get, Router};

use crate::application::state::AppState;
use crate::presentation::handlers::progress_handlers::get_leaderboard_handler;

pub fn progress_routes() -> Router<AppState> {
    Router::new().route("/leaderboard", get(get_leaderboard_handler))
}
