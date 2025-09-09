use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::value_objects::{LessonId, LocalizedText, TopicId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Topic {
    pub id: TopicId,
    pub title: LocalizedText,
    pub description: LocalizedText,
    pub order: u32,
    pub lessons: Vec<LessonId>,
    pub required_skills: LocalizedText,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Topic {
    pub fn new(
        title: LocalizedText,
        description: LocalizedText,
        order: u32,
        required_skills: LocalizedText,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: TopicId::new(),
            title,
            description,
            order,
            lessons: Vec::new(),
            required_skills,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn add_lesson(&mut self, lesson_id: LessonId) {
        if !self.lessons.contains(&lesson_id) {
            self.lessons.push(lesson_id);
            self.updated_at = Utc::now();
        }
    }

    pub fn remove_lesson(&mut self, lesson_id: &LessonId) {
        self.lessons.retain(|id| id != lesson_id);
        self.updated_at = Utc::now();
    }

    pub fn update_order(&mut self, order: u32) {
        self.order = order;
        self.updated_at = Utc::now();
    }
}
