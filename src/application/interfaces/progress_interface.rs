use anyhow::Result;
use axum::{extract::State, response::Json};
use serde_json::json;

use crate::application::state::AppState;
use crate::shared::errors::AppError;

pub async fn get_leaderboard_handler(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    // Get weekly leaderboard (top 10)
    let weekly_leaderboard = state
        .leaderboard_service
        .get_weekly_leaderboard(10)
        .await
        .map_err(|e| AppError::Validation(e.to_string()))?;

    // Get all-time leaderboard (top 10)
    let all_time_leaderboard = state
        .leaderboard_service
        .get_all_time_leaderboard(10)
        .await
        .map_err(|e| AppError::Validation(e.to_string()))?;

    Ok(Json(json!({
        "message": "Leaderboard retrieved successfully",
        "weekly_leaderboard": weekly_leaderboard,
        "all_time_leaderboard": all_time_leaderboard
    })))
}
