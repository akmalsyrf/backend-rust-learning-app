use anyhow::Result;
use async_trait::async_trait;

use crate::domain::entities::User;
use crate::domain::value_objects::{Email, UserId};

#[async_trait]
pub trait UserRepository: Send + Sync + 'static {
    async fn create(&self, user: &User) -> Result<()>;
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>>;
    async fn find_by_email(&self, email: &Email) -> Result<Option<User>>;
    async fn update(&self, user: &User) -> Result<()>;
    async fn delete(&self, id: &UserId) -> Result<()>;
    async fn list(&self, limit: u32, offset: u32) -> Result<Vec<User>>;
}
