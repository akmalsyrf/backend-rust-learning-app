use anyhow::Result;
use async_trait::async_trait;

use crate::domain::entities::CodePractice;
use crate::domain::value_objects::{CodePracticeId, Difficulty, LessonId, TopicId};

#[async_trait]
pub trait CodePracticeRepository: Send + Sync + 'static {
    async fn create(&self, code_practice: &CodePractice) -> Result<()>;
    async fn find_by_id(&self, id: &CodePracticeId) -> Result<Option<CodePractice>>;
    async fn find_by_topic_id(&self, topic_id: &TopicId) -> Result<Vec<CodePractice>>;
    async fn find_by_lesson_id(&self, lesson_id: &LessonId) -> Result<Vec<CodePractice>>;
    async fn find_by_difficulty(&self, difficulty: &Difficulty) -> Result<Vec<CodePractice>>;
    async fn update(&self, code_practice: &CodePractice) -> Result<()>;
    async fn delete(&self, id: &CodePracticeId) -> Result<()>;
    async fn list(&self, limit: u32, offset: u32) -> Result<Vec<CodePractice>>;
}
