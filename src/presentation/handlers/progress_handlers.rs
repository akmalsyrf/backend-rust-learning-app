use axum::{
    extract::{Query, State},
    response::Json,
};
use serde::Deserialize;

use crate::application::state::AppState;
use crate::application::use_cases::progress_use_cases::LeaderboardResponse;
use crate::shared::errors::Result;

#[derive(Debug, Deserialize)]
pub struct LeaderboardQuery {
    pub limit: Option<u32>,
}

pub async fn get_leaderboard_handler(
    State(state): State<AppState>,
    Query(params): Query<LeaderboardQuery>,
) -> Result<Json<LeaderboardResponse>> {
    let response = state
        .progress_use_cases
        .get_leaderboard(params.limit)
        .await?;
    Ok(Json(response))
}
