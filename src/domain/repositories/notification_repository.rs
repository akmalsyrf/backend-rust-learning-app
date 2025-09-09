use crate::domain::entities::Notification;
use crate::domain::value_objects::{NotificationId, UserId};
use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait NotificationRepository: Send + Sync {
    async fn create(&self, notification: &Notification)
        -> Result<(), Box<dyn Error + Send + Sync>>;
    async fn get_by_id(
        &self,
        id: &NotificationId,
    ) -> Result<Option<Notification>, Box<dyn Error + Send + Sync>>;
    async fn get_by_user_id(
        &self,
        user_id: &UserId,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<Notification>, Box<dyn Error + Send + Sync>>;
    async fn get_unread_by_user_id(
        &self,
        user_id: &UserId,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<Notification>, Box<dyn Error + Send + Sync>>;
    async fn mark_as_read(&self, id: &NotificationId) -> Result<(), Box<dyn Error + Send + Sync>>;
    async fn mark_all_as_read(&self, user_id: &UserId) -> Result<(), Box<dyn Error + Send + Sync>>;
    async fn delete(&self, id: &NotificationId) -> Result<(), Box<dyn Error + Send + Sync>>;
    async fn delete_old_notifications(
        &self,
        days_old: u32,
    ) -> Result<u32, Box<dyn Error + Send + Sync>>;
    async fn get_unread_count(&self, user_id: &UserId)
        -> Result<u32, Box<dyn Error + Send + Sync>>;
    async fn list(
        &self,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<Notification>, Box<dyn Error + Send + Sync>>;
}
