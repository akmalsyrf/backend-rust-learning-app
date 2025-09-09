use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

use crate::domain::entities::LeaderboardEntry;
use crate::domain::value_objects::UserId;

#[async_trait]
pub trait LeaderboardService: Send + Sync + 'static {
    async fn get_weekly_leaderboard(&self, limit: u32) -> Result<Vec<LeaderboardEntry>>;
    async fn get_all_time_leaderboard(&self, limit: u32) -> Result<Vec<LeaderboardEntry>>;
    async fn get_user_rank(&self, user_id: &UserId) -> Result<Option<u32>>;
    async fn update_leaderboard(&self) -> Result<()>;
}

pub struct LeaderboardServiceImpl {
    leaderboard_repository: Arc<dyn crate::domain::repositories::LeaderboardRepository>,
}

impl LeaderboardServiceImpl {
    pub fn new(
        leaderboard_repository: Arc<dyn crate::domain::repositories::LeaderboardRepository>,
    ) -> Self {
        Self {
            leaderboard_repository,
        }
    }
}

#[async_trait]
impl LeaderboardService for LeaderboardServiceImpl {
    async fn get_weekly_leaderboard(&self, limit: u32) -> Result<Vec<LeaderboardEntry>> {
        self.leaderboard_repository
            .get_weekly_leaderboard(limit)
            .await
    }

    async fn get_all_time_leaderboard(&self, limit: u32) -> Result<Vec<LeaderboardEntry>> {
        self.leaderboard_repository
            .get_all_time_leaderboard(limit)
            .await
    }

    async fn get_user_rank(&self, user_id: &UserId) -> Result<Option<u32>> {
        self.leaderboard_repository.get_user_rank(user_id).await
    }

    async fn update_leaderboard(&self) -> Result<()> {
        // For now, this is a no-op since the leaderboard is calculated on-demand
        // In a real implementation, you might want to:
        // 1. Calculate and cache leaderboard data
        // 2. Update rankings periodically
        // 3. Send notifications for rank changes
        // 4. Update weekly/monthly leaderboards

        // The current implementation relies on the repository to calculate
        // leaderboard data dynamically from user progress
        Ok(())
    }
}
