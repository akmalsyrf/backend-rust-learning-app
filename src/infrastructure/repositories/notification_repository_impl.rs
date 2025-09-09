use crate::domain::entities::Notification;
use crate::domain::repositories::NotificationRepository;
use crate::domain::value_objects::{NotificationId, UserId};
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct NotificationRepositoryImpl {
    notifications: Arc<RwLock<HashMap<String, Notification>>>,
}

impl NotificationRepositoryImpl {
    pub fn new() -> Self {
        Self {
            notifications: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait::async_trait]
impl NotificationRepository for NotificationRepositoryImpl {
    async fn create(
        &self,
        notification: &Notification,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut notifications = self.notifications.write().await;
        let id = notification.id.to_string();
        notifications.insert(id.clone(), notification.clone());
        Ok(())
    }

    async fn get_by_id(
        &self,
        id: &NotificationId,
    ) -> Result<Option<Notification>, Box<dyn Error + Send + Sync>> {
        let notifications = self.notifications.read().await;
        Ok(notifications.get(&id.to_string()).cloned())
    }

    async fn get_by_user_id(
        &self,
        user_id: &UserId,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<Notification>, Box<dyn Error + Send + Sync>> {
        let notifications = self.notifications.read().await;
        let mut user_notifications: Vec<Notification> = notifications
            .values()
            .filter(|n| n.user_id == *user_id)
            .cloned()
            .collect();
        user_notifications.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        Ok(user_notifications
            .into_iter()
            .skip(offset as usize)
            .take(limit as usize)
            .collect())
    }

    async fn get_unread_by_user_id(
        &self,
        user_id: &UserId,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<Notification>, Box<dyn Error + Send + Sync>> {
        let notifications = self.notifications.read().await;
        let mut unread_notifications: Vec<Notification> = notifications
            .values()
            .filter(|n| n.user_id == *user_id && !n.is_read)
            .cloned()
            .collect();
        unread_notifications.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        Ok(unread_notifications
            .into_iter()
            .skip(offset as usize)
            .take(limit as usize)
            .collect())
    }

    async fn mark_as_read(&self, id: &NotificationId) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut notifications = self.notifications.write().await;
        if let Some(notification) = notifications.get_mut(&id.to_string()) {
            notification.is_read = true;
        }
        Ok(())
    }

    async fn mark_all_as_read(&self, user_id: &UserId) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut notifications = self.notifications.write().await;
        for notification in notifications.values_mut() {
            if notification.user_id == *user_id {
                notification.is_read = true;
            }
        }
        Ok(())
    }

    async fn delete(&self, id: &NotificationId) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut notifications = self.notifications.write().await;
        notifications.remove(&id.to_string());
        Ok(())
    }

    async fn delete_old_notifications(
        &self,
        days_old: u32,
    ) -> Result<u32, Box<dyn Error + Send + Sync>> {
        let mut notifications = self.notifications.write().await;
        let cutoff_date = chrono::Utc::now() - chrono::Duration::days(days_old as i64);
        let mut count = 0;
        notifications.retain(|_, notification| {
            if notification.created_at < cutoff_date {
                count += 1;
                false
            } else {
                true
            }
        });
        Ok(count)
    }

    async fn get_unread_count(
        &self,
        user_id: &UserId,
    ) -> Result<u32, Box<dyn Error + Send + Sync>> {
        let notifications = self.notifications.read().await;
        Ok(notifications
            .values()
            .filter(|n| n.user_id == *user_id && !n.is_read)
            .count() as u32)
    }

    async fn list(
        &self,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<Notification>, Box<dyn Error + Send + Sync>> {
        let notifications = self.notifications.read().await;
        let mut values: Vec<Notification> = notifications.values().cloned().collect();
        values.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        Ok(values
            .into_iter()
            .skip(offset as usize)
            .take(limit as usize)
            .collect())
    }
}
