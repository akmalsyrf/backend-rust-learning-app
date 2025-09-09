use anyhow::Result;
use axum::{extract::State, response::Json};
use serde_json::json;

use crate::application::state::AppState;
use crate::shared::errors::AppError;

pub async fn list_topics_handler(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    // Get all topics using repository
    let topics = state.topic_repository
        .list(100, 0) // Get first 100 topics
        .await
        .map_err(|e| AppError::Validation(e.to_string()))?;

    Ok(Json(json!({
        "message": "Topics retrieved successfully",
        "topics": topics
    })))
}

pub async fn list_lessons_handler(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    // Get all lessons using repository
    let lessons = state.lesson_repository
        .list(100, 0) // Get first 100 lessons
        .await
        .map_err(|e| AppError::Validation(e.to_string()))?;

    Ok(Json(json!({
        "message": "Lessons retrieved successfully",
        "lessons": lessons
    })))
}

pub async fn list_questions_handler(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    // Get all questions using repository
    let questions = state.question_repository
        .list(100, 0) // Get first 100 questions
        .await
        .map_err(|e| AppError::Validation(e.to_string()))?;

    Ok(Json(json!({
        "message": "Questions retrieved successfully",
        "questions": questions
    })))
}

pub async fn list_code_practices_handler(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    // Get all code practices using repository
    let code_practices = state.code_practice_repository
        .list(100, 0) // Get first 100 code practices
        .await
        .map_err(|e| AppError::Validation(e.to_string()))?;

    Ok(Json(json!({
        "message": "Code practices retrieved successfully",
        "code_practices": code_practices
    })))
}
