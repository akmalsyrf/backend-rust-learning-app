use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::value_objects::{CodePracticeId, LessonId, Points, QuestionId, UserId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionResult {
    pub question_id: QuestionId,
    pub correct: bool,
    pub user_answer: String,
    pub time_spent_ms: u64,
    pub points: Points,
    pub completed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LessonResult {
    pub lesson_id: LessonId,
    pub question_results: Vec<QuestionResult>,
    pub completed_at: DateTime<Utc>,
    pub xp_earned: Points,
    pub perfect_score: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletedCodePractice {
    pub id: CodePracticeId,
    pub completed_at: DateTime<Utc>,
    pub user_code: String,
    pub is_correct: bool,
    pub xp_earned: Points,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProgress {
    pub user_id: UserId,
    pub completed_questions: Vec<QuestionResult>,
    pub lesson_stars: Vec<(LessonId, u32)>, // 0-3 stars
    pub completed_code_practices: Vec<CompletedCodePractice>,
    pub total_xp: Points,
    pub current_streak_days: u32,
    pub highest_streak_days: u32,
    pub last_active_date: NaiveDate,
    pub daily_xp_cap: u32,
    pub last_xp_reset_date: NaiveDate,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserProgress {
    pub fn new(user_id: UserId) -> Self {
        let now = Utc::now();
        let today = now.date_naive();
        Self {
            user_id,
            completed_questions: Vec::new(),
            lesson_stars: Vec::new(),
            completed_code_practices: Vec::new(),
            total_xp: Points::new(0),
            current_streak_days: 0,
            highest_streak_days: 0,
            last_active_date: today,
            daily_xp_cap: 1000, // Default daily XP cap
            last_xp_reset_date: today,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn add_question_result(&mut self, result: QuestionResult) {
        let points = result.points;
        self.completed_questions.push(result);
        self.total_xp = self.total_xp.add(points);
        self.updated_at = Utc::now();
    }

    pub fn add_code_practice_completion(&mut self, completion: CompletedCodePractice) {
        let xp_earned = completion.xp_earned;
        self.completed_code_practices.push(completion);
        self.total_xp = self.total_xp.add(xp_earned);
        self.updated_at = Utc::now();
    }

    pub fn update_lesson_stars(&mut self, lesson_id: LessonId, stars: u32) {
        if let Some(existing) = self
            .lesson_stars
            .iter_mut()
            .find(|(id, _)| *id == lesson_id)
        {
            existing.1 = stars;
        } else {
            self.lesson_stars.push((lesson_id, stars));
        }
        self.updated_at = Utc::now();
    }

    pub fn update_streak(&mut self, days: u32) {
        self.current_streak_days = days;
        if days > self.highest_streak_days {
            self.highest_streak_days = days;
        }
        self.updated_at = Utc::now();
    }

    pub fn update_last_active(&mut self) {
        self.last_active_date = Utc::now().date_naive();
        self.updated_at = Utc::now();
    }
}
