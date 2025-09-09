use anyhow::Result;
use async_trait::async_trait;

use crate::domain::entities::Topic;
use crate::domain::value_objects::TopicId;

#[async_trait]
pub trait TopicRepository: Send + Sync + 'static {
    async fn create(&self, topic: &Topic) -> Result<()>;
    async fn find_by_id(&self, id: &TopicId) -> Result<Option<Topic>>;
    async fn update(&self, topic: &Topic) -> Result<()>;
    async fn delete(&self, id: &TopicId) -> Result<()>;
    async fn list(&self, limit: u32, offset: u32) -> Result<Vec<Topic>>;
    async fn list_by_order(&self) -> Result<Vec<Topic>>;
}
