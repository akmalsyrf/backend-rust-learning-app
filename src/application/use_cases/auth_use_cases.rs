use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::domain::entities::User;
use crate::domain::services::AuthService;
use crate::domain::value_objects::{Email, Password};

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub display_name: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub user: UserResponse,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub display_name: String,
    pub total_xp: u32,
    pub current_streak_days: u32,
    pub highest_streak_days: u32,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id.to_string(),
            email: user.email.as_str().to_string(),
            display_name: user.display_name,
            total_xp: user.total_xp.value(),
            current_streak_days: user.current_streak_days,
            highest_streak_days: user.highest_streak_days,
        }
    }
}

#[derive(Clone)]
pub struct AuthUseCases {
    auth_service: Arc<dyn AuthService>,
}

impl AuthUseCases {
    pub fn new(auth_service: Arc<dyn AuthService>) -> Self {
        Self { auth_service }
    }

    pub async fn register(&self, request: RegisterRequest) -> Result<AuthResponse> {
        let email = Email::new(request.email).map_err(|e| anyhow::anyhow!(e))?;
        let password = Password::new(&request.password).map_err(|e| anyhow::anyhow!(e))?;

        let user = self
            .auth_service
            .register(email.clone(), password.clone(), request.display_name)
            .await?;
        let token = self.auth_service.login(email, password).await?;

        Ok(AuthResponse {
            access_token: token.access_token,
            token_type: token.token_type,
            expires_in: token.expires_in,
            user: UserResponse::from(user),
        })
    }

    pub async fn login(&self, request: LoginRequest) -> Result<AuthResponse> {
        let email = Email::new(request.email.clone()).map_err(|e| anyhow::anyhow!(e))?;
        let password = Password::new(&request.password).map_err(|e| anyhow::anyhow!(e))?;

        let token = self.auth_service.login(email, password).await?;

        // TODO: Get user from token or separate call
        // For now, return a placeholder user
        let user = UserResponse {
            id: "placeholder".to_string(),
            email: request.email,
            display_name: "User".to_string(),
            total_xp: 0,
            current_streak_days: 0,
            highest_streak_days: 0,
        };

        Ok(AuthResponse {
            access_token: token.access_token,
            token_type: token.token_type,
            expires_in: token.expires_in,
            user,
        })
    }
}
