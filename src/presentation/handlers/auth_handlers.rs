use axum::{extract::State, response::Json};

use crate::application::state::AppState;
use crate::application::use_cases::auth_use_cases::{AuthResponse, LoginRequest, RegisterRequest};
use crate::shared::errors::Result;

pub async fn register_handler(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>> {
    let response = state.auth_use_cases.register(payload).await?;
    Ok(Json(response))
}

pub async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>> {
    let response = state.auth_use_cases.login(payload).await?;
    Ok(Json(response))
}
