use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub server_host: String,
    pub server_port: u16,
    pub cors_origins: Vec<String>,
    pub gemini_api_key: String,
    pub gemini_api_url: String,
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub from_email: String,
    pub from_name: String,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        // Required environment variables - no defaults for security
        let database_url = env::var("DATABASE_URL")
            .map_err(|_| "DATABASE_URL environment variable is required")?;

        let jwt_secret =
            env::var("JWT_SECRET").map_err(|_| "JWT_SECRET environment variable is required")?;

        let gemini_api_key = env::var("GEMINI_API_KEY")
            .map_err(|_| "GEMINI_API_KEY environment variable is required")?;

        let smtp_username = env::var("SMTP_USERNAME")
            .map_err(|_| "SMTP_USERNAME environment variable is required")?;

        let smtp_password = env::var("SMTP_PASSWORD")
            .map_err(|_| "SMTP_PASSWORD environment variable is required")?;

        let from_email =
            env::var("FROM_EMAIL").map_err(|_| "FROM_EMAIL environment variable is required")?;

        // Optional environment variables with safe defaults
        let server_host = env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

        let server_port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .map_err(|_| "Invalid SERVER_PORT format")?;

        let cors_origins = env::var("CORS_ORIGINS")
            .unwrap_or_else(|_| "http://localhost:3000,http://localhost:3001".to_string())
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        let gemini_api_url = env::var("GEMINI_API_URL")
            .unwrap_or_else(|_| "https://generativelanguage.googleapis.com/v1beta".to_string());

        let smtp_host = env::var("SMTP_HOST").unwrap_or_else(|_| "smtp.gmail.com".to_string());

        let smtp_port = env::var("SMTP_PORT")
            .unwrap_or_else(|_| "587".to_string())
            .parse()
            .map_err(|_| "Invalid SMTP_PORT format")?;

        let from_name =
            env::var("FROM_NAME").unwrap_or_else(|_| "Rust Learning Platform".to_string());

        // Validate JWT secret strength
        if jwt_secret.len() < 32 {
            return Err("JWT_SECRET must be at least 32 characters long for security".to_string());
        }

        // Validate email format
        if !from_email.contains('@') {
            return Err("FROM_EMAIL must be a valid email address".to_string());
        }

        Ok(Self {
            database_url,
            jwt_secret,
            server_host,
            server_port,
            cors_origins,
            gemini_api_key,
            gemini_api_url,
            smtp_host,
            smtp_port,
            smtp_username,
            smtp_password,
            from_email,
            from_name,
        })
    }
}
