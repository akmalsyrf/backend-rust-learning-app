use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::value_objects::{LessonId, LocalizedText, QuestionId, TopicId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lesson {
    pub id: LessonId,
    pub title: LocalizedText,
    pub topic_id: TopicId,
    pub summary: LocalizedText,
    pub questions: Vec<QuestionId>,
    pub attribution_url: String,
    pub order: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Lesson {
    pub fn new(
        title: LocalizedText,
        topic_id: TopicId,
        summary: LocalizedText,
        attribution_url: String,
        order: u32,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: LessonId::new(),
            title,
            topic_id,
            summary,
            questions: Vec::new(),
            attribution_url,
            order,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn add_question(&mut self, question_id: QuestionId) {
        if !self.questions.contains(&question_id) {
            self.questions.push(question_id);
            self.updated_at = Utc::now();
        }
    }

    pub fn remove_question(&mut self, question_id: &QuestionId) {
        self.questions.retain(|id| id != question_id);
        self.updated_at = Utc::now();
    }

    pub fn update_order(&mut self, order: u32) {
        self.order = order;
        self.updated_at = Utc::now();
    }
}
