use anyhow::Result;
use async_trait::async_trait;

use crate::domain::entities::UserProgress;
use crate::domain::value_objects::UserId;

#[async_trait]
pub trait UserProgressRepository: Send + Sync + 'static {
    async fn create(&self, progress: &UserProgress) -> Result<()>;
    async fn find_by_user_id(&self, user_id: &UserId) -> Result<Option<UserProgress>>;
    async fn update(&self, progress: &UserProgress) -> Result<()>;
    async fn delete(&self, user_id: &UserId) -> Result<()>;
    async fn list(&self, limit: u32, offset: u32) -> Result<Vec<UserProgress>>;
}
