use anyhow::Result;
use async_trait::async_trait;

use crate::domain::entities::Lesson;
use crate::domain::value_objects::{LessonId, TopicId};

#[async_trait]
pub trait LessonRepository: Send + Sync + 'static {
    async fn create(&self, lesson: &Lesson) -> Result<()>;
    async fn find_by_id(&self, id: &LessonId) -> Result<Option<Lesson>>;
    async fn find_by_topic_id(&self, topic_id: &TopicId) -> Result<Vec<Lesson>>;
    async fn update(&self, lesson: &Lesson) -> Result<()>;
    async fn delete(&self, id: &LessonId) -> Result<()>;
    async fn list(&self, limit: u32, offset: u32) -> Result<Vec<Lesson>>;
}
