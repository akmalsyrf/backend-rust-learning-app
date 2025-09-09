use crate::domain::value_objects::{NotificationId, UserId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationType {
    Achievement,          // User achieved something (XP milestone, streak, etc.)
    LessonComplete,       // User completed a lesson
    QuestionCorrect,      // User answered a question correctly
    CodePracticeComplete, // User completed a code practice
    SystemUpdate,         // System-wide notifications
    Reminder,             // Reminder notifications
    Welcome,              // Welcome message for new users
    Progress,             // Progress-related notifications
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationPriority {
    Low,
    Medium,
    High,
    Urgent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: NotificationId,
    pub user_id: UserId,
    pub title: String,
    pub message: String,
    pub notification_type: NotificationType,
    pub priority: NotificationPriority,
    pub is_read: bool,
    pub created_at: DateTime<Utc>,
    pub read_at: Option<DateTime<Utc>>,
    pub metadata: Option<serde_json::Value>, // Additional data like lesson_id, xp_gained, etc.
}

impl Notification {
    pub fn new(
        id: NotificationId,
        user_id: UserId,
        title: String,
        message: String,
        notification_type: NotificationType,
        priority: NotificationPriority,
        metadata: Option<serde_json::Value>,
    ) -> Self {
        Self {
            id,
            user_id,
            title,
            message,
            notification_type,
            priority,
            is_read: false,
            created_at: Utc::now(),
            read_at: None,
            metadata,
        }
    }

    pub fn mark_as_read(&mut self) {
        self.is_read = true;
        self.read_at = Some(Utc::now());
    }

    pub fn is_achievement(&self) -> bool {
        matches!(self.notification_type, NotificationType::Achievement)
    }

    pub fn is_high_priority(&self) -> bool {
        matches!(
            self.priority,
            NotificationPriority::High | NotificationPriority::Urgent
        )
    }

    pub fn get_icon(&self) -> &'static str {
        match self.notification_type {
            NotificationType::Achievement => "🏆",
            NotificationType::LessonComplete => "✅",
            NotificationType::QuestionCorrect => "🎯",
            NotificationType::CodePracticeComplete => "💻",
            NotificationType::SystemUpdate => "🔔",
            NotificationType::Reminder => "⏰",
            NotificationType::Welcome => "👋",
            NotificationType::Progress => "📈",
        }
    }

    pub fn get_priority_color(&self) -> &'static str {
        match self.priority {
            NotificationPriority::Low => "#6c757d",
            NotificationPriority::Medium => "#007bff",
            NotificationPriority::High => "#ffc107",
            NotificationPriority::Urgent => "#dc3545",
        }
    }
}
