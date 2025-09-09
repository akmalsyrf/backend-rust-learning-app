use serde::{Deserialize, Serialize};

use crate::domain::value_objects::{Points, UserId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardEntry {
    pub user_id: UserId,
    pub display_name: String,
    pub xp_this_week: Points,
    pub total_xp: Points,
    pub rank: u32,
}

impl LeaderboardEntry {
    pub fn new(
        user_id: UserId,
        display_name: String,
        xp_this_week: Points,
        total_xp: Points,
        rank: u32,
    ) -> Self {
        Self {
            user_id,
            display_name,
            xp_this_week,
            total_xp,
            rank,
        }
    }
}
