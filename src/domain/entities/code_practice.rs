use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::value_objects::{
    CodePracticeId, Difficulty, LessonId, LocalizedText, Points, TopicId,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodePractice {
    pub id: CodePracticeId,
    pub title: LocalizedText,
    pub description: LocalizedText,
    pub initial_code: String,
    pub expected_output: Option<String>,
    pub solution: String,
    pub hints: Vec<LocalizedText>,
    pub difficulty: Difficulty,
    pub category: String,
    pub lesson_id: LessonId,
    pub topic_id: TopicId,
    pub points: Points,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl CodePractice {
    pub fn new(
        title: LocalizedText,
        description: LocalizedText,
        initial_code: String,
        solution: String,
        difficulty: Difficulty,
        category: String,
        lesson_id: LessonId,
        topic_id: TopicId,
        points: Points,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: CodePracticeId::new(),
            title,
            description,
            initial_code,
            expected_output: None,
            solution,
            hints: Vec::new(),
            difficulty,
            category,
            lesson_id,
            topic_id,
            points,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn add_hint(&mut self, hint: LocalizedText) {
        self.hints.push(hint);
        self.updated_at = Utc::now();
    }

    pub fn set_expected_output(&mut self, output: String) {
        self.expected_output = Some(output);
        self.updated_at = Utc::now();
    }

    pub fn update_difficulty(&mut self, difficulty: Difficulty) {
        self.difficulty = difficulty;
        self.updated_at = Utc::now();
    }

    pub fn update_points(&mut self, points: Points) {
        self.points = points;
        self.updated_at = Utc::now();
    }
}
