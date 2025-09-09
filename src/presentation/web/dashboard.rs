use axum::{extract::State, response::Html};

use crate::application::state::AppState;
use crate::shared::errors::Result;

#[derive(Debug, Clone)]
pub struct DashboardStats {
    pub topics_count: u32,
    pub lessons_count: u32,
    pub questions_count: u32,
    pub users_count: u32,
}

pub async fn dashboard_handler(State(state): State<AppState>) -> Result<Html<String>> {
    // Fetch real stats from database
    let topics = state.topic_repository.list(1000, 0).await?;
    let lessons = state.lesson_repository.list(1000, 0).await?;
    let questions = state.question_repository.list(1000, 0).await?;
    let users = state.user_repository.list(1000, 0).await?;

    let stats = DashboardStats {
        topics_count: topics.len() as u32,
        lessons_count: lessons.len() as u32,
        questions_count: questions.len() as u32,
        users_count: users.len() as u32,
    };

    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Rust Learning Admin Dashboard</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; }}
        .header {{ text-align: center; margin-bottom: 40px; }}
        .stats-grid {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 20px; margin-bottom: 40px; }}
        .stat-card {{ background-color: #f8f9fa; padding: 20px; border-radius: 8px; border: 1px solid #dee2e6; text-align: center; }}
        .stat-number {{ font-size: 32px; font-weight: bold; color: #007bff; margin-bottom: 10px; }}
        .stat-label {{ color: #6c757d; font-size: 14px; }}
        .actions-grid {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 15px; }}
        .action-card {{ background-color: #ffffff; padding: 20px; border-radius: 8px; border: 1px solid #dee2e6; text-align: center; }}
        .action-title {{ font-weight: bold; margin-bottom: 10px; color: #495057; }}
        .action-links {{ display: flex; flex-direction: column; gap: 8px; }}
        .btn {{ padding: 8px 16px; text-decoration: none; border-radius: 4px; font-size: 14px; }}
        .btn-primary {{ background-color: #007bff; color: white; }}
        .btn-success {{ background-color: #28a745; color: white; }}
        .btn-info {{ background-color: #17a2b8; color: white; }}
        .btn-warning {{ background-color: #ffc107; color: #212529; }}
        .btn-danger {{ background-color: #dc3545; color: white; }}
        .btn:hover {{ opacity: 0.8; }}
    </style>
</head>
<body>
    <div class="header">
        <h1>🚀 Rust Learning Admin Dashboard</h1>
        <p>Welcome to the Rust Learning Platform Administration Panel</p>
    </div>

    <div class="stats-grid">
        <div class="stat-card">
            <div class="stat-number">{}</div>
            <div class="stat-label">Total Topics</div>
        </div>
        <div class="stat-card">
            <div class="stat-number">{}</div>
            <div class="stat-label">Total Lessons</div>
        </div>
        <div class="stat-card">
            <div class="stat-number">{}</div>
            <div class="stat-label">Total Questions</div>
        </div>
        <div class="stat-card">
            <div class="stat-number">{}</div>
            <div class="stat-label">Total Users</div>
        </div>
    </div>

    <div class="actions-grid">
        <div class="action-card">
            <div class="action-title">📚 Topics Management</div>
            <div class="action-links">
                <a href="/admin/topics" class="btn btn-primary">View All Topics</a>
                <a href="/admin/search/topics" class="btn btn-info">Search Topics</a>
                <a href="/admin/topics/new" class="btn btn-success">Create New Topic</a>
            </div>
        </div>

        <div class="action-card">
            <div class="action-title">📖 Lessons Management</div>
            <div class="action-links">
                <a href="/admin/lessons" class="btn btn-primary">View All Lessons</a>
                <a href="/admin/search/lessons" class="btn btn-info">Search Lessons</a>
                <a href="/admin/lessons/new" class="btn btn-success">Create New Lesson</a>
            </div>
        </div>

        <div class="action-card">
            <div class="action-title">❓ Questions Management</div>
            <div class="action-links">
                <a href="/admin/questions" class="btn btn-primary">View All Questions</a>
                <a href="/admin/search/questions" class="btn btn-info">Search Questions</a>
                <a href="/admin/questions/new" class="btn btn-success">Create New Question</a>
            </div>
        </div>

        <div class="action-card">
            <div class="action-title">💻 Code Practices Management</div>
            <div class="action-links">
                <a href="/admin/code-practices" class="btn btn-primary">View All Code Practices</a>
                <a href="/admin/search/code-practices" class="btn btn-info">Search Code Practices</a>
                <a href="/admin/code-practices/new" class="btn btn-success">Create New Code Practice</a>
            </div>
        </div>

        <div class="action-card">
            <div class="action-title">👥 Users Management</div>
            <div class="action-links">
                <a href="/admin/users" class="btn btn-primary">View All Users</a>
                <a href="/admin/search/users" class="btn btn-info">Search Users</a>
                <a href="/admin/users/new" class="btn btn-success">Create New User</a>
            </div>
        </div>

        <div class="action-card">
            <div class="action-title">🤖 AI Features</div>
            <div class="action-links">
                <a href="/admin/ai" class="btn btn-warning">AI Dashboard</a>
                <a href="/admin/ai/generate-quiz" class="btn btn-info">Generate Quiz</a>
                <a href="/admin/ai/validate-code" class="btn btn-info">Validate Code</a>
            </div>
        </div>

        <div class="action-card">
            <div class="action-title">📊 Analytics & Reports</div>
            <div class="action-links">
                <a href="/admin/analytics" class="btn btn-warning">Analytics Dashboard</a>
                <a href="/admin/analytics" class="btn btn-info">View Statistics</a>
                <a href="/admin/analytics" class="btn btn-info">Performance Metrics</a>
            </div>
        </div>

        <div class="action-card">
            <div class="action-title">🔔 Notifications Management</div>
            <div class="action-links">
                <a href="/admin/notifications" class="btn btn-primary">View All Notifications</a>
                <a href="/admin/notifications/create" class="btn btn-success">Create Notification</a>
                <a href="/admin/notifications" class="btn btn-info">Manage Notifications</a>
            </div>
        </div>

        <div class="action-card">
            <div class="action-title">📤📥 Export/Import Management</div>
            <div class="action-links">
                <a href="/admin/export-import" class="btn btn-primary">Export/Import Data</a>
                <a href="/admin/export-import/backup" class="btn btn-success">Create Backup</a>
                <a href="/admin/export-import/stats" class="btn btn-info">View Statistics</a>
            </div>
        </div>

        <div class="action-card">
            <div class="action-title">📁 File Upload Center</div>
            <div class="action-links">
                <a href="/admin/file-upload" class="btn btn-primary">Upload Files</a>
                <a href="/admin/file-upload" class="btn btn-success">Manage Files</a>
                <a href="/admin/file-upload" class="btn btn-info">File Library</a>
            </div>
        </div>

        <div class="action-card">
            <div class="action-title">🔗 API Endpoints</div>
            <div class="action-links">
                <a href="/api/content/topics" class="btn btn-primary">Topics API</a>
                <a href="/api/content/lessons" class="btn btn-success">Lessons API</a>
                <a href="/api/content/questions" class="btn btn-info">Questions API</a>
            </div>
        </div>

        <div class="action-card">
            <div class="action-title">🔄 Bulk Operations</div>
            <div class="action-links">
                <a href="/admin/bulk-operations" class="btn btn-primary">Bulk Actions</a>
                <a href="/admin/bulk-operations" class="btn btn-success">Mass Delete</a>
                <a href="/admin/bulk-operations" class="btn btn-info">Bulk Update</a>
            </div>
        </div>

        <div class="action-card">
            <div class="action-title">👥 Advanced User Management</div>
            <div class="action-links">
                <a href="/admin/user-management" class="btn btn-primary">User Management</a>
                <a href="/admin/user-management" class="btn btn-success">Role Management</a>
                <a href="/admin/user-management" class="btn btn-info">Permissions</a>
            </div>
        </div>

        <div class="action-card">
            <div class="action-title">📝 Content Management</div>
            <div class="action-links">
                <a href="/admin/content-management" class="btn btn-primary">Content Management</a>
                <a href="/admin/content-management" class="btn btn-success">Version Control</a>
                <a href="/admin/content-management" class="btn btn-info">Approval Workflow</a>
            </div>
        </div>

        <div class="action-card">
            <div class="action-title">🔍 Data Validation</div>
            <div class="action-links">
                <a href="/admin/data-validation" class="btn btn-primary">Data Validation</a>
                <a href="/admin/data-validation" class="btn btn-success">Quality Check</a>
                <a href="/admin/data-validation" class="btn btn-info">Auto Fix</a>
            </div>
        </div>

        <div class="action-card">
            <div class="action-title">📋 Audit Logging</div>
            <div class="action-links">
                <a href="/admin/audit-logs" class="btn btn-primary">Audit Logs</a>
                <a href="/admin/audit-logs" class="btn btn-success">Activity Tracking</a>
                <a href="/admin/audit-logs" class="btn btn-info">Change History</a>
            </div>
        </div>
    </div>
</body>
</html>"#,
        stats.topics_count, stats.lessons_count, stats.questions_count, stats.users_count
    );

    Ok(Html(html))
}
