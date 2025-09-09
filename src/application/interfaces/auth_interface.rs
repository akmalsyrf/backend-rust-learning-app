use anyhow::Result;
use axum::{extract::State, response::Json};
use serde_json::json;

use crate::application::dtos::auth_dtos::{LoginDto, RegisterDto};
use crate::application::state::AppState;
use crate::domain::value_objects::{Email, Password};
use crate::shared::errors::AppError;

pub async fn register_handler(
    State(state): State<AppState>,
    Json(payload): Json<RegisterDto>,
) -> Result<Json<serde_json::Value>, AppError> {
    // Validate input
    let email = Email::new(payload.email.clone())
        .map_err(|e| AppError::Validation(format!("Invalid email: {e}")))?;
    let password = Password::new(&payload.password)
        .map_err(|e| AppError::Validation(format!("Invalid password: {e}")))?;

    // Register user using AuthService
    let user = state
        .auth_service
        .register(email, password, payload.display_name)
        .await
        .map_err(|e| AppError::Validation(e.to_string()))?;

    Ok(Json(json!({
        "message": "User registered successfully",
        "user_id": user.id.to_string(),
        "email": user.email.into_string(),
        "display_name": user.display_name
    })))
}

pub async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginDto>,
) -> Result<Json<serde_json::Value>, AppError> {
    // Validate input
    let email = Email::new(payload.email.clone())
        .map_err(|e| AppError::Validation(format!("Invalid email: {e}")))?;
    let password = Password::new(&payload.password)
        .map_err(|e| AppError::Validation(format!("Invalid password: {e}")))?;

    // Login using AuthService
    let auth_token = state
        .auth_service
        .login(email, password)
        .await
        .map_err(|e| AppError::Authentication(e.to_string()))?;

    Ok(Json(json!({
        "message": "Login successful",
        "access_token": auth_token.access_token,
        "token_type": auth_token.token_type,
        "expires_in": auth_token.expires_in
    })))
}
