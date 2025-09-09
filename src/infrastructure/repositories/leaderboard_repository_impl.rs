use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::entities::LeaderboardEntry;
use crate::domain::repositories::LeaderboardRepository;
use crate::domain::value_objects::UserId;
use crate::infrastructure::database::models::LeaderboardEntryModel;

pub struct LeaderboardRepositoryImpl {
    pool: PgPool,
}

impl LeaderboardRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl LeaderboardRepository for LeaderboardRepositoryImpl {
    async fn get_weekly_leaderboard(&self, limit: u32) -> Result<Vec<LeaderboardEntry>> {
        let entries = sqlx::query_as!(
            LeaderboardEntryModel,
            r#"
            SELECT
                u.id as user_id,
                u.display_name,
                COALESCE(up.total_xp, 0) as "total_xp!",
                COALESCE(up.total_xp, 0) as "xp_this_week!",
                ROW_NUMBER() OVER (ORDER BY COALESCE(up.total_xp, 0) DESC) as "rank!"
            FROM users u
            LEFT JOIN user_progress up ON u.id = up.user_id
            ORDER BY COALESCE(up.total_xp, 0) DESC
            LIMIT $1
            "#,
            limit as i64
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;

        Ok(entries
            .into_iter()
            .map(|m| LeaderboardEntry::from(m))
            .collect())
    }

    async fn get_all_time_leaderboard(&self, limit: u32) -> Result<Vec<LeaderboardEntry>> {
        let entries = sqlx::query_as!(
            LeaderboardEntryModel,
            r#"
            SELECT
                u.id as user_id,
                u.display_name,
                COALESCE(up.total_xp, 0) as "total_xp!",
                COALESCE(up.total_xp, 0) as "xp_this_week!",
                ROW_NUMBER() OVER (ORDER BY COALESCE(up.total_xp, 0) DESC) as "rank!"
            FROM users u
            LEFT JOIN user_progress up ON u.id = up.user_id
            ORDER BY COALESCE(up.total_xp, 0) DESC
            LIMIT $1
            "#,
            limit as i64
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;

        Ok(entries
            .into_iter()
            .map(|m| LeaderboardEntry::from(m))
            .collect())
    }

    async fn get_user_rank(&self, user_id: &UserId) -> Result<Option<u32>> {
        let rank = sqlx::query_scalar!(
            r#"
            SELECT rank FROM (
                SELECT
                    u.id,
                    ROW_NUMBER() OVER (ORDER BY COALESCE(up.total_xp, 0) DESC) as rank
                FROM users u
                LEFT JOIN user_progress up ON u.id = up.user_id
            ) ranked_users
            WHERE id = $1
            "#,
            user_id.0
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;

        Ok(rank.map(|r| r.unwrap_or(0) as u32))
    }
}
