use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::entities::UserProgress;
use crate::domain::repositories::UserProgressRepository;
use crate::domain::value_objects::UserId;
use crate::infrastructure::database::models::UserProgressModel;

pub struct UserProgressRepositoryImpl {
    pool: PgPool,
}

impl UserProgressRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserProgressRepository for UserProgressRepositoryImpl {
    async fn create(&self, progress: &UserProgress) -> Result<()> {
        let progress_model = UserProgressModel {
            user_id: progress.user_id.0,
            total_xp: progress.total_xp.value() as i32,
            current_streak_days: progress.current_streak_days as i32,
            highest_streak_days: progress.highest_streak_days as i32,
            last_active_date: progress.last_active_date,
            daily_xp_cap: 1000, // TODO: Make this configurable
            last_xp_reset_date: progress.last_active_date,
            completed_questions: serde_json::to_value(&progress.completed_questions)
                .unwrap_or(serde_json::Value::Array(vec![])),
            completed_code_practices: serde_json::to_value(&progress.completed_code_practices)
                .unwrap_or(serde_json::Value::Array(vec![])),
            lesson_stars: serde_json::to_value(&progress.lesson_stars)
                .unwrap_or(serde_json::Value::Object(serde_json::Map::new())),
            created_at: progress.created_at,
            updated_at: progress.updated_at,
        };

        sqlx::query!(
            r#"
            INSERT INTO user_progress (user_id, total_xp, current_streak_days, highest_streak_days, last_active_date, daily_xp_cap, last_xp_reset_date, completed_questions, completed_code_practices, lesson_stars, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            "#,
            progress_model.user_id,
            progress_model.total_xp,
            progress_model.current_streak_days,
            progress_model.highest_streak_days,
            progress_model.last_active_date,
            progress_model.daily_xp_cap,
            progress_model.last_xp_reset_date,
            progress_model.completed_questions,
            progress_model.completed_code_practices,
            progress_model.lesson_stars,
            progress_model.created_at,
            progress_model.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;

        Ok(())
    }

    async fn find_by_user_id(&self, user_id: &UserId) -> Result<Option<UserProgress>> {
        let progress_model = sqlx::query_as!(
            UserProgressModel,
            "SELECT * FROM user_progress WHERE user_id = $1",
            user_id.0
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;

        Ok(progress_model.map(|m| UserProgress::from(m)))
    }

    async fn update(&self, progress: &UserProgress) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE user_progress
            SET total_xp = $2, current_streak_days = $3, highest_streak_days = $4,
                last_active_date = $5, completed_questions = $6, completed_code_practices = $7,
                lesson_stars = $8, updated_at = $9
            WHERE user_id = $1
            "#,
            progress.user_id.0,
            progress.total_xp.value() as i32,
            progress.current_streak_days as i32,
            progress.highest_streak_days as i32,
            progress.last_active_date,
            serde_json::to_value(&progress.completed_questions)
                .unwrap_or(serde_json::Value::Array(vec![])),
            serde_json::to_value(&progress.completed_code_practices)
                .unwrap_or(serde_json::Value::Array(vec![])),
            serde_json::to_value(&progress.lesson_stars)
                .unwrap_or(serde_json::Value::Object(serde_json::Map::new())),
            progress.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;

        Ok(())
    }

    async fn delete(&self, user_id: &UserId) -> Result<()> {
        sqlx::query!("DELETE FROM user_progress WHERE user_id = $1", user_id.0)
            .execute(&self.pool)
            .await
            .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;

        Ok(())
    }

    async fn list(&self, limit: u32, offset: u32) -> Result<Vec<UserProgress>> {
        let rows = sqlx::query!(
            "SELECT * FROM user_progress ORDER BY updated_at DESC LIMIT $1 OFFSET $2",
            limit as i32,
            offset as i32
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;

        let mut progress_list = Vec::new();
        for row in rows {
            let progress = UserProgress {
                user_id: UserId::from_string(row.user_id.to_string()).unwrap(),
                total_xp: crate::domain::value_objects::Points::new(row.total_xp as u32),
                current_streak_days: row.current_streak_days as u32,
                highest_streak_days: row.highest_streak_days as u32,
                last_active_date: row.last_active_date,
                completed_questions: serde_json::from_value(row.completed_questions)
                    .unwrap_or_default(),
                completed_code_practices: serde_json::from_value(row.completed_code_practices)
                    .unwrap_or_default(),
                lesson_stars: serde_json::from_value(row.lesson_stars).unwrap_or_default(),
                daily_xp_cap: row.daily_xp_cap as u32,
                last_xp_reset_date: row.last_xp_reset_date,
                created_at: row.created_at,
                updated_at: row.updated_at,
            };
            progress_list.push(progress);
        }

        Ok(progress_list)
    }
}
