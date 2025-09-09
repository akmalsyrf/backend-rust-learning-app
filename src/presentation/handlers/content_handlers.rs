use axum::{
    extract::{Query, State},
    response::Json,
};
use serde::Deserialize;

use crate::application::state::AppState;
use crate::application::use_cases::content_use_cases::{
    CodePracticeResponse, LessonResponse, QuestionResponse, TopicResponse,
};
use crate::shared::errors::Result;

#[derive(Debug, Deserialize)]
pub struct TopicQuery {
    pub topic_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct QuestionQuery {
    pub topic_id: Option<String>,
    pub difficulty: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CodePracticeQuery {
    pub topic_id: Option<String>,
    pub lesson_id: Option<String>,
}

pub async fn list_topics_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<TopicResponse>>> {
    let topics = state.content_use_cases.get_topics().await?;
    Ok(Json(topics))
}

pub async fn list_lessons_handler(
    State(state): State<AppState>,
    Query(params): Query<TopicQuery>,
) -> Result<Json<Vec<LessonResponse>>> {
    let lessons = state.content_use_cases.get_lessons(params.topic_id).await?;
    Ok(Json(lessons))
}

pub async fn list_questions_handler(
    State(state): State<AppState>,
    Query(params): Query<QuestionQuery>,
) -> Result<Json<Vec<QuestionResponse>>> {
    let questions = state
        .content_use_cases
        .get_questions(params.topic_id, params.difficulty)
        .await?;
    Ok(Json(questions))
}

pub async fn list_code_practices_handler(
    State(state): State<AppState>,
    Query(params): Query<CodePracticeQuery>,
) -> Result<Json<Vec<CodePracticeResponse>>> {
    let code_practices = state
        .content_use_cases
        .get_code_practices(params.topic_id, params.lesson_id)
        .await?;
    Ok(Json(code_practices))
}
