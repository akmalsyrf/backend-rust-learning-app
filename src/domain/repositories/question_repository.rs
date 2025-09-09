use anyhow::Result;
use async_trait::async_trait;

use crate::domain::entities::Question;
use crate::domain::value_objects::{Difficulty, QuestionId, TopicId};

#[async_trait]
pub trait QuestionRepository: Send + Sync + 'static {
    async fn create(&self, question: &Question) -> Result<()>;
    async fn find_by_id(&self, id: &QuestionId) -> Result<Option<Question>>;
    async fn find_by_topic_id(&self, topic_id: &TopicId) -> Result<Vec<Question>>;
    async fn find_by_difficulty(&self, difficulty: &Difficulty) -> Result<Vec<Question>>;
    async fn update(&self, question: &Question) -> Result<()>;
    async fn delete(&self, id: &QuestionId) -> Result<()>;
    async fn list(&self, limit: u32, offset: u32) -> Result<Vec<Question>>;
}
