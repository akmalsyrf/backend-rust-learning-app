use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct LeaderboardDto {
    pub entries: Vec<LeaderboardEntryDto>,
}

#[derive(Debug, Serialize)]
pub struct LeaderboardEntryDto {
    pub user_id: String,
    pub display_name: String,
    pub xp_this_week: u32,
    pub total_xp: u32,
    pub rank: u32,
}

#[derive(Debug, Deserialize)]
pub struct SubmitQuestionResultDto {
    pub question_id: String,
    pub user_answer: String,
    pub time_spent_ms: u64,
}

#[derive(Debug, Deserialize)]
pub struct SubmitCodePracticeDto {
    pub code_practice_id: String,
    pub user_code: String,
}
