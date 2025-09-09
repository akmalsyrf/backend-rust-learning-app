use anyhow::Result;
use serde::Serialize;
use std::sync::Arc;

use crate::domain::entities::LeaderboardEntry;
use crate::domain::repositories::LeaderboardRepository;
use crate::domain::value_objects::UserId;

#[derive(Debug, Serialize)]
pub struct LeaderboardResponse {
    pub entries: Vec<LeaderboardEntryResponse>,
}

#[derive(Debug, Serialize)]
pub struct LeaderboardEntryResponse {
    pub user_id: String,
    pub display_name: String,
    pub xp_this_week: u32,
    pub total_xp: u32,
    pub rank: u32,
}

impl From<LeaderboardEntry> for LeaderboardEntryResponse {
    fn from(entry: LeaderboardEntry) -> Self {
        Self {
            user_id: entry.user_id.to_string(),
            display_name: entry.display_name,
            xp_this_week: entry.xp_this_week.value(),
            total_xp: entry.total_xp.value(),
            rank: entry.rank,
        }
    }
}

#[derive(Clone)]
pub struct ProgressUseCases {
    leaderboard_repository: Arc<dyn LeaderboardRepository>,
}

impl ProgressUseCases {
    pub fn new(leaderboard_repository: Arc<dyn LeaderboardRepository>) -> Self {
        Self {
            leaderboard_repository,
        }
    }

    pub async fn get_leaderboard(&self, limit: Option<u32>) -> Result<LeaderboardResponse> {
        let entries = self
            .leaderboard_repository
            .get_all_time_leaderboard(limit.unwrap_or(50))
            .await?;
        Ok(LeaderboardResponse {
            entries: entries
                .into_iter()
                .map(LeaderboardEntryResponse::from)
                .collect(),
        })
    }

    pub async fn get_user_rank(&self, user_id: &str) -> Result<Option<u32>> {
        let user_id = UserId::from_str(user_id).map_err(|e| anyhow::anyhow!(e))?;
        self.leaderboard_repository.get_user_rank(&user_id).await
    }
}
