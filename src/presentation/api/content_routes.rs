use axum::{routing::get, Router};

use crate::application::state::AppState;
use crate::presentation::handlers::content_handlers::{
    list_code_practices_handler, list_lessons_handler, list_questions_handler, list_topics_handler,
};

pub fn content_routes() -> Router<AppState> {
    Router::new()
        .route("/topics", get(list_topics_handler))
        .route("/lessons", get(list_lessons_handler))
        .route("/questions", get(list_questions_handler))
        .route("/code-practices", get(list_code_practices_handler))
}
