use anyhow::Result;
use async_trait::async_trait;

use crate::domain::entities::LeaderboardEntry;

#[async_trait]
pub trait LeaderboardRepository: Send + Sync + 'static {
    async fn get_weekly_leaderboard(&self, limit: u32) -> Result<Vec<LeaderboardEntry>>;
    async fn get_all_time_leaderboard(&self, limit: u32) -> Result<Vec<LeaderboardEntry>>;
    async fn get_user_rank(
        &self,
        user_id: &crate::domain::value_objects::UserId,
    ) -> Result<Option<u32>>;
}
