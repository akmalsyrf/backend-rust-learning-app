use anyhow::Result;
use async_trait::async_trait;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::domain::entities::User;
use crate::domain::repositories::UserRepository;
use crate::domain::value_objects::{Email, Password};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user_id
    pub email: String,
    pub exp: i64,
    pub iat: i64,
}

#[derive(Debug)]
pub struct AuthToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

#[async_trait]
pub trait AuthService: Send + Sync + 'static {
    async fn register(
        &self,
        email: Email,
        password: Password,
        display_name: String,
    ) -> Result<User>;
    async fn login(&self, email: Email, password: Password) -> Result<AuthToken>;
    async fn verify_token(&self, token: &str) -> Result<Claims>;
    async fn refresh_token(&self, refresh_token: &str) -> Result<AuthToken>;
}

pub struct JwtAuthService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    jwt_secret: String,
    user_repository: Arc<dyn UserRepository>,
}

impl JwtAuthService {
    pub fn new(jwt_secret: String, user_repository: Arc<dyn UserRepository>) -> Self {
        let encoding_key = EncodingKey::from_secret(jwt_secret.as_ref());
        let decoding_key = DecodingKey::from_secret(jwt_secret.as_ref());

        Self {
            encoding_key,
            decoding_key,
            jwt_secret,
            user_repository,
        }
    }

    fn generate_token(&self, user: &User) -> Result<AuthToken> {
        let now = Utc::now();
        let exp = now + Duration::hours(24); // 24 hours expiration

        let claims = Claims {
            sub: user.id.to_string(),
            email: user.email.as_str().to_string(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
        };

        let token = encode(&Header::new(Algorithm::HS256), &claims, &self.encoding_key)?;

        Ok(AuthToken {
            access_token: token,
            token_type: "Bearer".to_string(),
            expires_in: 24 * 60 * 60, // 24 hours in seconds
        })
    }
}

#[async_trait]
impl AuthService for JwtAuthService {
    async fn register(
        &self,
        email: Email,
        password: Password,
        display_name: String,
    ) -> Result<User> {
        // Check if user already exists
        if let Some(_) = self.user_repository.find_by_email(&email).await? {
            return Err(anyhow::anyhow!("User with this email already exists"));
        }

        // Create new user
        let user = User::new(email, password, display_name);

        // Save user to database
        self.user_repository.create(&user).await?;

        Ok(user)
    }

    async fn login(&self, email: Email, password: Password) -> Result<AuthToken> {
        // Find user by email
        let user = self
            .user_repository
            .find_by_email(&email)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Invalid email or password"))?;

        // Verify password
        if !user
            .verify_password(password.as_str())
            .map_err(|e| anyhow::anyhow!("Password verification failed: {}", e))?
        {
            return Err(anyhow::anyhow!("Invalid email or password"));
        }

        // Generate token
        self.generate_token(&user)
    }

    async fn verify_token(&self, token: &str) -> Result<Claims> {
        let token_data = decode::<Claims>(
            token,
            &self.decoding_key,
            &Validation::new(Algorithm::HS256),
        )?;

        // Check if token is expired
        let now = Utc::now().timestamp();
        if token_data.claims.exp < now {
            return Err(anyhow::anyhow!("Token has expired"));
        }

        Ok(token_data.claims)
    }

    async fn refresh_token(&self, refresh_token: &str) -> Result<AuthToken> {
        // For now, we'll just verify the token and generate a new one
        // In a real implementation, you might want to use refresh tokens
        let claims = self.verify_token(refresh_token).await?;

        // Find user by ID from claims
        let user_id = crate::domain::value_objects::UserId::from_string(claims.sub.clone())
            .map_err(|e| anyhow::anyhow!("Invalid user ID: {}", e))?;
        let user = self
            .user_repository
            .find_by_id(&user_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("User not found"))?;

        // Generate new token
        self.generate_token(&user)
    }
}
