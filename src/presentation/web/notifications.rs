use crate::application::state::AppState;
use crate::domain::value_objects::NotificationId;
use crate::presentation::web::pagination::{
    generate_pagination_controls, generate_pagination_html, generate_pagination_info,
    get_pagination_css, PaginationParams,
};
use crate::shared::errors::Result;
use axum::{
    extract::{Path, Query, State},
    response::Html,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NotificationQuery {
    pub user_id: Option<String>,
    pub unread_only: Option<bool>,
}

pub async fn notifications_handler(
    State(state): State<AppState>,
    Query(pagination): Query<PaginationParams>,
    Query(query): Query<NotificationQuery>,
) -> Result<Html<String>> {
    let page = pagination.page();
    let limit = pagination.limit();
    let offset = pagination.offset();

    // For demo purposes, we'll show all notifications
    // In a real app, you'd filter by user_id from session/auth
    let notifications = state.notification_repository.list(limit, offset).await?;

    // Get total count for pagination
    let all_notifications = state.notification_repository.list(1000, 0).await?;
    let total_count = all_notifications.len();

    let mut notifications_html = String::new();
    for notification in &notifications {
        let priority_color = notification.get_priority_color();
        let icon = notification.get_icon();
        let read_class = if notification.is_read {
            "read"
        } else {
            "unread"
        };
        let read_badge = if notification.is_read {
            ""
        } else {
            r#"<span class="unread-badge">NEW</span>"#
        };

        let action_button = if notification.is_read {
            r#"<span class="btn btn-sm btn-secondary">Read</span>"#.to_string()
        } else {
            format!(
                r#"<a href="/admin/notifications/{}/mark-read" class="btn btn-sm btn-primary">Mark Read</a>"#,
                notification.id.to_string()
            )
        };

        notifications_html.push_str(&format!(
            r#"<tr class="notification-row {}">
                <td>
                    <div class="notification-content">
                        <div class="notification-header">
                            <span class="notification-icon">{}</span>
                            <span class="notification-title">{}</span>
                            {}
                        </div>
                        <div class="notification-message">{}</div>
                        <div class="notification-meta">
                            <span class="notification-time">{}</span>
                            <span class="notification-priority" style="color: {};">{}</span>
                        </div>
                    </div>
                </td>
                <td>
                    <div class="notification-actions">
                        {}
                        <a href="/admin/notifications/{}/delete" class="btn btn-sm btn-danger" onclick="return confirm('Are you sure?')">Delete</a>
                    </div>
                </td>
            </tr>"#,
            read_class,
            icon,
            notification.title,
            read_badge,
            notification.message,
            notification.created_at.format("%Y-%m-%d %H:%M"),
            priority_color,
            format!("{:?}", notification.priority),
            action_button,
            notification.id.to_string()
        ));
    }

    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Notifications Management</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; }}
        .header {{ display: flex; justify-content: space-between; align-items: center; margin-bottom: 30px; }}
        .btn {{ padding: 8px 16px; text-decoration: none; border-radius: 4px; font-size: 14px; }}
        .btn-primary {{ background-color: #007bff; color: white; }}
        .btn-danger {{ background-color: #dc3545; color: white; }}
        .btn-success {{ background-color: #28a745; color: white; }}
        .btn-secondary {{ background-color: #6c757d; color: white; }}
        .btn-sm {{ padding: 6px 12px; font-size: 12px; }}
        .stats {{ display: flex; gap: 20px; margin-bottom: 20px; }}
        .stat-card {{ background-color: #f8f9fa; padding: 20px; border-radius: 8px; border: 1px solid #dee2e6; }}
        .stat-number {{ font-size: 24px; font-weight: bold; color: #007bff; }}
        .stat-label {{ color: #6c757d; }}
        .notification-row.unread {{ background-color: #fff3cd; border-left: 4px solid #ffc107; }}
        .notification-row.read {{ background-color: #f8f9fa; }}
        .notification-content {{ padding: 10px 0; }}
        .notification-header {{ display: flex; align-items: center; gap: 10px; margin-bottom: 8px; }}
        .notification-icon {{ font-size: 18px; }}
        .notification-title {{ font-weight: bold; color: #495057; }}
        .notification-message {{ color: #6c757d; margin-bottom: 8px; }}
        .notification-meta {{ display: flex; justify-content: space-between; align-items: center; font-size: 12px; }}
        .notification-time {{ color: #6c757d; }}
        .notification-priority {{ font-weight: bold; }}
        .unread-badge {{ background-color: #dc3545; color: white; padding: 2px 6px; border-radius: 12px; font-size: 10px; font-weight: bold; }}
        .notification-actions {{ display: flex; gap: 5px; }}
        table {{ width: 100%; border-collapse: collapse; margin-top: 20px; }}
        th, td {{ border: 1px solid #ddd; padding: 12px; text-align: left; }}
        th {{ background-color: #f8f9fa; font-weight: bold; }}
        .filters {{ background-color: #f8f9fa; padding: 20px; border-radius: 8px; margin-bottom: 20px; }}
        .filter-form {{ display: flex; gap: 15px; align-items: center; }}
        .filter-group {{ display: flex; flex-direction: column; gap: 5px; }}
        .filter-group label {{ font-size: 12px; color: #6c757d; }}
        .filter-group select, .filter-group input {{ padding: 8px; border: 1px solid #ced4da; border-radius: 4px; }}
        {}
    </style>
</head>
<body>
    <div class="header">
        <h1>🔔 Notifications Management</h1>
        <div>
            <a href="/admin/notifications/create" class="btn btn-success">+ Create Notification</a>
            <a href="/admin/notifications/mark-all-read" class="btn btn-primary">Mark All Read</a>
        </div>
    </div>

    <div class="stats">
        <div class="stat-card">
            <div class="stat-number">{}</div>
            <div class="stat-label">Total Notifications</div>
        </div>
    </div>

    <div class="filters">
        <form class="filter-form" method="GET" action="/admin/notifications">
            <div class="filter-group">
                <label>User ID</label>
                <input type="text" name="user_id" placeholder="Filter by user ID" value="{}">
            </div>
            <div class="filter-group">
                <label>Status</label>
                <select name="unread_only">
                    <option value="">All Notifications</option>
                    <option value="true" {}>Unread Only</option>
                    <option value="false" {}>Read Only</option>
                </select>
            </div>
            <div class="filter-group">
                <label>&nbsp;</label>
                <button type="submit" class="btn btn-primary">Filter</button>
            </div>
        </form>
    </div>

    {}
    <div class="pagination-info">{}</div>

    <table>
        <thead>
            <tr>
                <th>Notification</th>
                <th>Actions</th>
            </tr>
        </thead>
        <tbody>
            {}
        </tbody>
    </table>

    <div class="pagination">
        {}
    </div>

    <div style="margin-top: 30px;">
        <a href="/admin" class="btn btn-primary">← Back to Dashboard</a>
    </div>
</body>
</html>"#,
        get_pagination_css(),
        total_count,
        query.user_id.unwrap_or_default(),
        if query.unread_only == Some(true) {
            "selected"
        } else {
            ""
        },
        if query.unread_only == Some(false) {
            "selected"
        } else {
            ""
        },
        generate_pagination_controls(page, limit, total_count),
        generate_pagination_info(page, limit, total_count),
        notifications_html,
        generate_pagination_html(page, limit, total_count, "/admin/notifications", "")
    );

    Ok(Html(html))
}

pub async fn create_notification_handler() -> Result<Html<String>> {
    let html = r#"<!DOCTYPE html>
<html>
<head>
    <title>Create Notification</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; }}
        .form-container {{ max-width: 600px; margin: 0 auto; }}
        .form-group {{ margin-bottom: 20px; }}
        .form-group label {{ display: block; margin-bottom: 5px; font-weight: bold; color: #495057; }}
        .form-group input, .form-group select, .form-group textarea {{ width: 100%; padding: 10px; border: 1px solid #ced4da; border-radius: 4px; font-size: 14px; }}
        .form-group textarea {{ height: 100px; resize: vertical; }}
        .btn {{ padding: 10px 20px; text-decoration: none; border-radius: 4px; font-size: 14px; border: none; cursor: pointer; }}
        .btn-primary {{ background-color: #007bff; color: white; }}
        .btn-secondary {{ background-color: #6c757d; color: white; }}
        .btn:hover {{ opacity: 0.8; }}
        .form-actions {{ display: flex; gap: 10px; margin-top: 30px; }}
    </style>
</head>
<body>
    <div class="form-container">
        <h1>🔔 Create New Notification</h1>

        <form method="POST" action="/admin/notifications">
            <div class="form-group">
                <label for="user_id">User ID</label>
                <input type="text" id="user_id" name="user_id" required placeholder="Enter user ID">
            </div>

            <div class="form-group">
                <label for="title">Title</label>
                <input type="text" id="title" name="title" required placeholder="Enter notification title">
            </div>

            <div class="form-group">
                <label for="message">Message</label>
                <textarea id="message" name="message" required placeholder="Enter notification message"></textarea>
            </div>

            <div class="form-group">
                <label for="type">Type</label>
                <select id="type" name="type" required>
                    <option value="">Select notification type</option>
                    <option value="Achievement">🏆 Achievement</option>
                    <option value="LessonComplete">✅ Lesson Complete</option>
                    <option value="QuestionCorrect">🎯 Question Correct</option>
                    <option value="CodePracticeComplete">💻 Code Practice Complete</option>
                    <option value="SystemUpdate">🔔 System Update</option>
                    <option value="Reminder">⏰ Reminder</option>
                    <option value="Welcome">👋 Welcome</option>
                    <option value="Progress">📈 Progress</option>
                </select>
            </div>

            <div class="form-group">
                <label for="priority">Priority</label>
                <select id="priority" name="priority" required>
                    <option value="">Select priority</option>
                    <option value="Low">Low</option>
                    <option value="Medium">Medium</option>
                    <option value="High">High</option>
                    <option value="Urgent">Urgent</option>
                </select>
            </div>

            <div class="form-actions">
                <button type="submit" class="btn btn-primary">Create Notification</button>
                <a href="/admin/notifications" class="btn btn-secondary">Cancel</a>
            </div>
        </form>
    </div>
</body>
</html>"#;

    Ok(Html(html.to_string()))
}

pub async fn mark_notification_read_handler(
    State(state): State<AppState>,
    Path(notification_id): Path<String>,
) -> Result<Html<String>> {
    let notification_id = NotificationId::from_string(notification_id);

    // Mark notification as read
    state
        .notification_repository
        .mark_as_read(&notification_id)
        .await?;

    let html = r#"<!DOCTYPE html>
<html>
<head>
    <title>Notification Marked as Read</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; text-align: center; }}
        .success-message {{ background-color: #d4edda; color: #155724; padding: 20px; border-radius: 8px; margin: 20px 0; }}
        .btn {{ padding: 10px 20px; text-decoration: none; border-radius: 4px; font-size: 14px; }}
        .btn-primary {{ background-color: #007bff; color: white; }}
    </style>
</head>
<body>
    <h1>✅ Notification Marked as Read</h1>
    <div class="success-message">
        The notification has been successfully marked as read.
    </div>
    <a href="/admin/notifications" class="btn btn-primary">← Back to Notifications</a>
</body>
</html>"#;

    Ok(Html(html.to_string()))
}

pub async fn delete_notification_handler(
    State(state): State<AppState>,
    Path(notification_id): Path<String>,
) -> Result<Html<String>> {
    let notification_id = NotificationId::from_string(notification_id);

    // Delete notification
    state
        .notification_repository
        .delete(&notification_id)
        .await?;

    let html = r#"<!DOCTYPE html>
<html>
<head>
    <title>Notification Deleted</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; text-align: center; }}
        .success-message {{ background-color: #d4edda; color: #155724; padding: 20px; border-radius: 8px; margin: 20px 0; }}
        .btn {{ padding: 10px 20px; text-decoration: none; border-radius: 4px; font-size: 14px; }}
        .btn-primary {{ background-color: #007bff; color: white; }}
    </style>
</head>
<body>
    <h1>🗑️ Notification Deleted</h1>
    <div class="success-message">
        The notification has been successfully deleted.
    </div>
    <a href="/admin/notifications" class="btn btn-primary">← Back to Notifications</a>
</body>
</html>"#;

    Ok(Html(html.to_string()))
}
