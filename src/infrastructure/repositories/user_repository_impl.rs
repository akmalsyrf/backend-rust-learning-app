use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::entities::User;
use crate::domain::repositories::UserRepository;
use crate::domain::value_objects::{Email, UserId};
use crate::infrastructure::database::models::UserModel;

pub struct UserRepositoryImpl {
    pool: PgPool,
}

impl UserRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create(&self, user: &User) -> Result<()> {
        let user_model = UserModel {
            id: user.id.0,
            email: user.email.as_str().to_string(),
            password_hash: user.password.as_str().to_string(),
            display_name: user.display_name.clone(),
            total_xp: user.total_xp.value() as i32,
            current_streak_days: user.current_streak_days as i32,
            highest_streak_days: user.highest_streak_days as i32,
            last_active_date: user.last_active_date,
            created_at: user.created_at,
            updated_at: user.updated_at,
        };

        sqlx::query!(
            r#"
            INSERT INTO users (id, email, password_hash, display_name, total_xp, current_streak_days, highest_streak_days, last_active_date, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
            user_model.id,
            user_model.email,
            user_model.password_hash,
            user_model.display_name,
            user_model.total_xp,
            user_model.current_streak_days,
            user_model.highest_streak_days,
            user_model.last_active_date,
            user_model.created_at,
            user_model.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;

        Ok(())
    }

    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>> {
        let user_model = sqlx::query_as!(UserModel, "SELECT * FROM users WHERE id = $1", id.0)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;

        Ok(user_model.map(User::from))
    }

    async fn find_by_email(&self, email: &Email) -> Result<Option<User>> {
        let user_model = sqlx::query_as!(
            UserModel,
            "SELECT * FROM users WHERE email = $1",
            email.as_str()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;

        Ok(user_model.map(User::from))
    }

    async fn update(&self, user: &User) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE users
            SET email = $2, password_hash = $3, display_name = $4, total_xp = $5,
                current_streak_days = $6, highest_streak_days = $7, last_active_date = $8, updated_at = $9
            WHERE id = $1
            "#,
            user.id.0,
            user.email.as_str(),
            user.password.as_str(),
            user.display_name,
            user.total_xp.value() as i32,
            user.current_streak_days as i32,
            user.highest_streak_days as i32,
            user.last_active_date,
            user.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;

        Ok(())
    }

    async fn delete(&self, id: &UserId) -> Result<()> {
        sqlx::query!("DELETE FROM users WHERE id = $1", id.0)
            .execute(&self.pool)
            .await
            .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;

        Ok(())
    }

    async fn list(&self, limit: u32, offset: u32) -> Result<Vec<User>> {
        let user_models = sqlx::query_as!(
            UserModel,
            "SELECT * FROM users ORDER BY created_at DESC LIMIT $1 OFFSET $2",
            limit as i64,
            offset as i64
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;

        Ok(user_models.into_iter().map(User::from).collect())
    }
}
