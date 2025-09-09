use crate::domain::entities::Notification;
use crate::domain::repositories::NotificationRepository;
use crate::domain::value_objects::{NotificationId, UserId};
use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait NotificationService: Send + Sync {
    async fn create_notification(
        &self,
        user_id: &UserId,
        title: String,
        message: String,
        notification_type: crate::domain::entities::notification::NotificationType,
        priority: crate::domain::entities::notification::NotificationPriority,
        metadata: Option<serde_json::Value>,
    ) -> Result<Notification, Box<dyn Error + Send + Sync>>;

    async fn get_user_notifications(
        &self,
        user_id: &UserId,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<Notification>, Box<dyn Error + Send + Sync>>;

    async fn get_unread_notifications(
        &self,
        user_id: &UserId,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<Notification>, Box<dyn Error + Send + Sync>>;

    async fn mark_notification_as_read(
        &self,
        notification_id: &NotificationId,
    ) -> Result<(), Box<dyn Error + Send + Sync>>;

    async fn mark_all_as_read(&self, user_id: &UserId) -> Result<(), Box<dyn Error + Send + Sync>>;

    async fn get_unread_count(&self, user_id: &UserId)
        -> Result<u32, Box<dyn Error + Send + Sync>>;

    async fn delete_notification(
        &self,
        notification_id: &NotificationId,
    ) -> Result<(), Box<dyn Error + Send + Sync>>;

    // Specific notification creation methods
    async fn create_achievement_notification(
        &self,
        user_id: &UserId,
        achievement_title: String,
        achievement_message: String,
        xp_gained: u32,
    ) -> Result<Notification, Box<dyn Error + Send + Sync>>;

    async fn create_lesson_complete_notification(
        &self,
        user_id: &UserId,
        lesson_title: String,
        xp_gained: u32,
    ) -> Result<Notification, Box<dyn Error + Send + Sync>>;

    async fn create_question_correct_notification(
        &self,
        user_id: &UserId,
        question_title: String,
        xp_gained: u32,
    ) -> Result<Notification, Box<dyn Error + Send + Sync>>;

    async fn create_code_practice_complete_notification(
        &self,
        user_id: &UserId,
        practice_title: String,
        xp_gained: u32,
    ) -> Result<Notification, Box<dyn Error + Send + Sync>>;

    async fn create_welcome_notification(
        &self,
        user_id: &UserId,
        user_name: String,
    ) -> Result<Notification, Box<dyn Error + Send + Sync>>;

    async fn create_progress_notification(
        &self,
        user_id: &UserId,
        progress_message: String,
        current_xp: u32,
    ) -> Result<Notification, Box<dyn Error + Send + Sync>>;

    async fn create_system_notification(
        &self,
        title: String,
        message: String,
        priority: crate::domain::entities::notification::NotificationPriority,
    ) -> Result<Vec<Notification>, Box<dyn Error + Send + Sync>>;
}

pub struct NotificationServiceImpl {
    notification_repository: Box<dyn NotificationRepository>,
    user_repository: Box<dyn crate::domain::repositories::UserRepository>,
}

impl NotificationServiceImpl {
    pub fn new(
        notification_repository: Box<dyn NotificationRepository>,
        user_repository: Box<dyn crate::domain::repositories::UserRepository>,
    ) -> Self {
        Self {
            notification_repository,
            user_repository,
        }
    }
}

#[async_trait]
impl NotificationService for NotificationServiceImpl {
    async fn create_notification(
        &self,
        user_id: &UserId,
        title: String,
        message: String,
        notification_type: crate::domain::entities::notification::NotificationType,
        priority: crate::domain::entities::notification::NotificationPriority,
        metadata: Option<serde_json::Value>,
    ) -> Result<Notification, Box<dyn Error + Send + Sync>> {
        let notification = Notification::new(
            NotificationId::new(),
            user_id.clone(),
            title,
            message,
            notification_type,
            priority,
            metadata,
        );

        self.notification_repository.create(&notification).await?;
        Ok(notification)
    }

    async fn get_user_notifications(
        &self,
        user_id: &UserId,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<Notification>, Box<dyn Error + Send + Sync>> {
        self.notification_repository
            .get_by_user_id(user_id, limit, offset)
            .await
    }

    async fn get_unread_notifications(
        &self,
        user_id: &UserId,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<Notification>, Box<dyn Error + Send + Sync>> {
        self.notification_repository
            .get_unread_by_user_id(user_id, limit, offset)
            .await
    }

    async fn mark_notification_as_read(
        &self,
        notification_id: &NotificationId,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.notification_repository
            .mark_as_read(notification_id)
            .await
    }

    async fn mark_all_as_read(&self, user_id: &UserId) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.notification_repository.mark_all_as_read(user_id).await
    }

    async fn get_unread_count(
        &self,
        user_id: &UserId,
    ) -> Result<u32, Box<dyn Error + Send + Sync>> {
        self.notification_repository.get_unread_count(user_id).await
    }

    async fn delete_notification(
        &self,
        notification_id: &NotificationId,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.notification_repository.delete(notification_id).await
    }

    async fn create_achievement_notification(
        &self,
        user_id: &UserId,
        achievement_title: String,
        achievement_message: String,
        xp_gained: u32,
    ) -> Result<Notification, Box<dyn Error + Send + Sync>> {
        let metadata = serde_json::json!({
            "xp_gained": xp_gained,
            "achievement_type": "milestone"
        });

        self.create_notification(
            user_id,
            format!("🏆 Achievement Unlocked: {achievement_title}"),
            achievement_message,
            crate::domain::entities::notification::NotificationType::Achievement,
            crate::domain::entities::notification::NotificationPriority::High,
            Some(metadata),
        )
        .await
    }

    async fn create_lesson_complete_notification(
        &self,
        user_id: &UserId,
        lesson_title: String,
        xp_gained: u32,
    ) -> Result<Notification, Box<dyn Error + Send + Sync>> {
        let metadata = serde_json::json!({
            "xp_gained": xp_gained,
            "lesson_title": lesson_title
        });

        self.create_notification(
            user_id,
            format!("✅ Lesson Completed: {lesson_title}"),
            format!("Great job! You've completed '{lesson_title}' and earned {xp_gained} XP!"),
            crate::domain::entities::notification::NotificationType::LessonComplete,
            crate::domain::entities::notification::NotificationPriority::Medium,
            Some(metadata),
        )
        .await
    }

    async fn create_question_correct_notification(
        &self,
        user_id: &UserId,
        question_title: String,
        xp_gained: u32,
    ) -> Result<Notification, Box<dyn Error + Send + Sync>> {
        let metadata = serde_json::json!({
            "xp_gained": xp_gained,
            "question_title": question_title
        });

        self.create_notification(
            user_id,
            format!("🎯 Correct Answer: {question_title}"),
            format!("Excellent! You answered correctly and earned {xp_gained} XP!",),
            crate::domain::entities::notification::NotificationType::QuestionCorrect,
            crate::domain::entities::notification::NotificationPriority::Low,
            Some(metadata),
        )
        .await
    }

    async fn create_code_practice_complete_notification(
        &self,
        user_id: &UserId,
        practice_title: String,
        xp_gained: u32,
    ) -> Result<Notification, Box<dyn Error + Send + Sync>> {
        let metadata = serde_json::json!({
            "xp_gained": xp_gained,
            "practice_title": practice_title
        });

        self.create_notification(
            user_id,
            format!("💻 Code Practice Completed: {practice_title}"),
            format!("Well done! You've completed '{practice_title}' and earned {xp_gained} XP!"),
            crate::domain::entities::notification::NotificationType::CodePracticeComplete,
            crate::domain::entities::notification::NotificationPriority::Medium,
            Some(metadata),
        )
        .await
    }

    async fn create_welcome_notification(
        &self,
        user_id: &UserId,
        user_name: String,
    ) -> Result<Notification, Box<dyn Error + Send + Sync>> {
        self.create_notification(
            user_id,
            "👋 Welcome to Rust Learning Platform!".to_string(),
            format!("Hello {user_name}! Welcome to the Rust Learning Platform. Start your journey to master Rust programming!"),
            crate::domain::entities::notification::NotificationType::Welcome,
            crate::domain::entities::notification::NotificationPriority::High,
            None,
        )
        .await
    }

    async fn create_progress_notification(
        &self,
        user_id: &UserId,
        progress_message: String,
        current_xp: u32,
    ) -> Result<Notification, Box<dyn Error + Send + Sync>> {
        let metadata = serde_json::json!({
            "current_xp": current_xp
        });

        self.create_notification(
            user_id,
            "📈 Progress Update".to_string(),
            progress_message,
            crate::domain::entities::notification::NotificationType::Progress,
            crate::domain::entities::notification::NotificationPriority::Low,
            Some(metadata),
        )
        .await
    }

    async fn create_system_notification(
        &self,
        title: String,
        message: String,
        priority: crate::domain::entities::notification::NotificationPriority,
    ) -> Result<Vec<Notification>, Box<dyn Error + Send + Sync>> {
        // Get all users to send system notification to
        let users = self.user_repository.list(1000, 0).await?;
        let mut notifications = Vec::new();

        for user in users {
            let notification = self
                .create_notification(
                    &user.id,
                    title.clone(),
                    message.clone(),
                    crate::domain::entities::notification::NotificationType::SystemUpdate,
                    priority.clone(),
                    None,
                )
                .await?;
            notifications.push(notification);
        }

        Ok(notifications)
    }
}
