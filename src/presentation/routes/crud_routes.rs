use axum::routing::{get, post};
use axum::Router;

use crate::application::state::AppState;
use crate::presentation::web::code_practices::{
    code_practices_handler, create_code_practice_handler, create_code_practice_post_handler,
    delete_code_practice_handler, edit_code_practice_handler, update_code_practice_post_handler,
};
use crate::presentation::web::lessons::{
    create_lesson_handler, create_lesson_post_handler, delete_lesson_handler, edit_lesson_handler,
    lessons_handler, update_lesson_post_handler,
};
use crate::presentation::web::questions::{
    create_question_handler, create_question_post_handler, delete_question_handler,
    edit_question_handler, questions_handler, update_question_post_handler,
};
use crate::presentation::web::topics::{
    create_topic_handler, create_topic_post_handler, delete_topic_handler, edit_topic_handler,
    topics_handler, update_topic_post_handler,
};
use crate::presentation::web::users::{
    create_user_handler, create_user_post_handler, delete_user_handler, edit_user_handler,
    update_user_post_handler, users_handler,
};

pub fn crud_routes() -> Router<AppState> {
    Router::new()
        // Topics CRUD routes
        .route("/admin/topics", get(topics_handler))
        .route("/admin/topics/new", get(create_topic_handler))
        .route("/admin/topics", post(create_topic_post_handler))
        .route("/admin/topics/:id/edit", get(edit_topic_handler))
        .route("/admin/topics/:id/update", post(update_topic_post_handler))
        .route("/admin/topics/:id/delete", get(delete_topic_handler))

        // Lessons CRUD routes
        .route("/admin/lessons", get(lessons_handler))
        .route("/admin/lessons/new", get(create_lesson_handler))
        .route("/admin/lessons", post(create_lesson_post_handler))
        .route("/admin/lessons/:id/edit", get(edit_lesson_handler))
        .route("/admin/lessons/:id/update", post(update_lesson_post_handler))
        .route("/admin/lessons/:id/delete", get(delete_lesson_handler))

        // Questions CRUD routes
        .route("/admin/questions", get(questions_handler))
        .route("/admin/questions/new", get(create_question_handler))
        .route("/admin/questions", post(create_question_post_handler))
        .route("/admin/questions/:id/edit", get(edit_question_handler))
        .route("/admin/questions/:id/update", post(update_question_post_handler))
        .route("/admin/questions/:id/delete", get(delete_question_handler))

        // User Management routes
        .route("/admin/users", get(users_handler))
        .route("/admin/users/new", get(create_user_handler))
        .route("/admin/users", post(create_user_post_handler))
        .route("/admin/users/:id/edit", get(edit_user_handler))
        .route("/admin/users/:id/update", post(update_user_post_handler))
        .route("/admin/users/:id/delete", get(delete_user_handler))

        // Code Practices routes
        .route("/admin/code-practices", get(code_practices_handler))
        .route("/admin/code-practices/new", get(create_code_practice_handler))
        .route("/admin/code-practices", post(create_code_practice_post_handler))
        .route("/admin/code-practices/:id/edit", get(edit_code_practice_handler))
        .route("/admin/code-practices/:id/update", post(update_code_practice_post_handler))
        .route("/admin/code-practices/:id/delete", get(delete_code_practice_handler))
}
