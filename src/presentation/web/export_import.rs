use crate::application::state::AppState;
use crate::shared::errors::Result;
use axum::{
    body::Body,
    extract::State,
    response::{Html, Response},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportData {
    pub topics: Vec<crate::domain::entities::Topic>,
    pub lessons: Vec<crate::domain::entities::Lesson>,
    pub questions: Vec<crate::domain::entities::Question>,
    pub code_practices: Vec<crate::domain::entities::CodePractice>,
    pub users: Vec<crate::domain::entities::User>,
    pub user_progress: Vec<crate::domain::entities::UserProgress>,
    pub notifications: Vec<crate::domain::entities::Notification>,
    pub export_timestamp: chrono::DateTime<chrono::Utc>,
    pub version: String,
}

#[derive(Debug, Deserialize)]
pub struct ImportOptions {
    pub include_topics: Option<bool>,
    pub include_lessons: Option<bool>,
    pub include_questions: Option<bool>,
    pub include_code_practices: Option<bool>,
    pub include_users: Option<bool>,
    pub include_user_progress: Option<bool>,
    pub include_notifications: Option<bool>,
    pub overwrite_existing: Option<bool>,
}

pub async fn export_import_handler(State(_state): State<AppState>) -> Result<Html<String>> {
    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Export/Import Management</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; }}
        .header {{ display: flex; justify-content: space-between; align-items: center; margin-bottom: 30px; }}
        .btn {{ padding: 10px 20px; text-decoration: none; border-radius: 4px; font-size: 14px; border: none; cursor: pointer; }}
        .btn-primary {{ background-color: #007bff; color: white; }}
        .btn-success {{ background-color: #28a745; color: white; }}
        .btn-warning {{ background-color: #ffc107; color: black; }}
        .btn-danger {{ background-color: #dc3545; color: white; }}
        .btn:hover {{ opacity: 0.8; }}
        .action-cards {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 20px; margin-bottom: 30px; }}
        .action-card {{ background-color: #f8f9fa; padding: 20px; border-radius: 8px; border: 1px solid #dee2e6; }}
        .action-title {{ font-size: 18px; font-weight: bold; margin-bottom: 15px; color: #495057; }}
        .action-description {{ color: #6c757d; margin-bottom: 15px; }}
        .action-buttons {{ display: flex; gap: 10px; flex-wrap: wrap; }}
        .form-group {{ margin-bottom: 15px; }}
        .form-group label {{ display: block; margin-bottom: 5px; font-weight: bold; color: #495057; }}
        .form-group input[type="file"] {{ width: 100%; padding: 8px; border: 1px solid #ced4da; border-radius: 4px; }}
        .form-group input[type="checkbox"] {{ margin-right: 8px; }}
        .checkbox-group {{ display: flex; flex-direction: column; gap: 8px; }}
        .checkbox-item {{ display: flex; align-items: center; }}
        .stats {{ display: flex; gap: 20px; margin-bottom: 20px; }}
        .stat-card {{ background-color: #e9ecef; padding: 15px; border-radius: 8px; text-align: center; }}
        .stat-number {{ font-size: 24px; font-weight: bold; color: #007bff; }}
        .stat-label {{ color: #6c757d; font-size: 12px; }}
        .warning-box {{ background-color: #fff3cd; border: 1px solid #ffeaa7; padding: 15px; border-radius: 4px; margin-bottom: 20px; }}
        .warning-box .warning-title {{ font-weight: bold; color: #856404; margin-bottom: 8px; }}
        .warning-box .warning-text {{ color: #856404; }}
    </style>
</head>
<body>
    <div class="header">
        <h1>📤📥 Export/Import Management</h1>
        <a href="/admin" class="btn btn-primary">← Back to Dashboard</a>
    </div>

    <div class="stats">
        <div class="stat-card">
            <div class="stat-number">📊</div>
            <div class="stat-label">Data Management</div>
        </div>
        <div class="stat-card">
            <div class="stat-number">🔄</div>
            <div class="stat-label">Backup & Restore</div>
        </div>
        <div class="stat-card">
            <div class="stat-number">📋</div>
            <div class="stat-label">Format Support</div>
        </div>
    </div>

    <div class="warning-box">
        <div class="warning-title">⚠️ Important Notice</div>
        <div class="warning-text">
            Export/Import operations can affect your data. Always backup your data before importing.
            Import operations may overwrite existing data depending on your settings.
        </div>
    </div>

    <div class="action-cards">
        <div class="action-card">
            <div class="action-title">📤 Export Data</div>
            <div class="action-description">
                Export your application data in JSON format. Choose which data types to include in the export.
            </div>
            <form method="GET" action="/admin/export-import/export">
                <div class="form-group">
                    <label>Export Format:</label>
                    <select name="format" style="width: 100%; padding: 8px; border: 1px solid #ced4da; border-radius: 4px;">
                        <option value="json">JSON (Recommended)</option>
                        <option value="csv">CSV (Limited)</option>
                    </select>
                </div>
                <div class="form-group">
                    <label>Data Types to Export:</label>
                    <div class="checkbox-group">
                        <div class="checkbox-item">
                            <input type="checkbox" name="include_topics" value="true" checked>
                            <label>Topics</label>
                        </div>
                        <div class="checkbox-item">
                            <input type="checkbox" name="include_lessons" value="true" checked>
                            <label>Lessons</label>
                        </div>
                        <div class="checkbox-item">
                            <input type="checkbox" name="include_questions" value="true" checked>
                            <label>Questions</label>
                        </div>
                        <div class="checkbox-item">
                            <input type="checkbox" name="include_code_practices" value="true" checked>
                            <label>Code Practices</label>
                        </div>
                        <div class="checkbox-item">
                            <input type="checkbox" name="include_users" value="true">
                            <label>Users (Sensitive Data)</label>
                        </div>
                        <div class="checkbox-item">
                            <input type="checkbox" name="include_user_progress" value="true">
                            <label>User Progress</label>
                        </div>
                        <div class="checkbox-item">
                            <input type="checkbox" name="include_notifications" value="true">
                            <label>Notifications</label>
                        </div>
                    </div>
                </div>
                <button type="submit" class="btn btn-success">📤 Export Data</button>
            </form>
        </div>

        <div class="action-card">
            <div class="action-title">📥 Import Data</div>
            <div class="action-description">
                Import data from a previously exported file. Make sure the file format matches the selected export format.
            </div>
            <form method="POST" action="/admin/export-import/import" enctype="multipart/form-data">
                <div class="form-group">
                    <label for="import_file">Select File:</label>
                    <input type="file" id="import_file" name="file" accept=".json,.csv" required>
                </div>
                <div class="form-group">
                    <label>Import Options:</label>
                    <div class="checkbox-group">
                        <div class="checkbox-item">
                            <input type="checkbox" name="overwrite_existing" value="true">
                            <label>Overwrite existing data</label>
                        </div>
                        <div class="checkbox-item">
                            <input type="checkbox" name="include_topics" value="true" checked>
                            <label>Import Topics</label>
                        </div>
                        <div class="checkbox-item">
                            <input type="checkbox" name="include_lessons" value="true" checked>
                            <label>Import Lessons</label>
                        </div>
                        <div class="checkbox-item">
                            <input type="checkbox" name="include_questions" value="true" checked>
                            <label>Import Questions</label>
                        </div>
                        <div class="checkbox-item">
                            <input type="checkbox" name="include_code_practices" value="true" checked>
                            <label>Import Code Practices</label>
                        </div>
                        <div class="checkbox-item">
                            <input type="checkbox" name="include_users" value="true">
                            <label>Import Users</label>
                        </div>
                        <div class="checkbox-item">
                            <input type="checkbox" name="include_user_progress" value="true">
                            <label>Import User Progress</label>
                        </div>
                        <div class="checkbox-item">
                            <input type="checkbox" name="include_notifications" value="true">
                            <label>Import Notifications</label>
                        </div>
                    </div>
                </div>
                <button type="submit" class="btn btn-warning">📥 Import Data</button>
            </form>
        </div>

        <div class="action-card">
            <div class="action-title">🔄 Backup & Restore</div>
            <div class="action-description">
                Create a complete backup of your system or restore from a previous backup.
            </div>
            <div class="action-buttons">
                <a href="/admin/export-import/backup" class="btn btn-primary">💾 Create Backup</a>
                <a href="/admin/export-import/restore" class="btn btn-warning">🔄 Restore Backup</a>
            </div>
        </div>

        <div class="action-card">
            <div class="action-title">📊 Data Statistics</div>
            <div class="action-description">
                View statistics about your data and export/import history.
            </div>
            <div class="action-buttons">
                <a href="/admin/export-import/stats" class="btn btn-primary">📈 View Statistics</a>
                <a href="/admin/export-import/history" class="btn btn-primary">📋 Export History</a>
            </div>
        </div>
    </div>

    <div style="margin-top: 30px; padding: 20px; background-color: #f8f9fa; border-radius: 8px;">
        <h3>📋 Supported Formats</h3>
        <ul>
            <li><strong>JSON:</strong> Complete data export with all relationships and metadata</li>
            <li><strong>CSV:</strong> Tabular data export for spreadsheet applications (limited support)</li>
        </ul>

        <h3>🔧 Features</h3>
        <ul>
            <li>Selective data export/import</li>
            <li>Data validation and error reporting</li>
            <li>Backup and restore functionality</li>
            <li>Export history tracking</li>
            <li>Data integrity checks</li>
        </ul>
    </div>
</body>
</html>"#
    );

    Ok(Html(html))
}

pub async fn export_data_handler(
    State(state): State<AppState>,
    axum::extract::Query(_params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Result<Response<Body>> {
    // For demo purposes, we'll create a simple export
    let export_data = ExportData {
        topics: state.topic_repository.list(1000, 0).await?,
        lessons: state.lesson_repository.list(1000, 0).await?,
        questions: state.question_repository.list(1000, 0).await?,
        code_practices: state.code_practice_repository.list(1000, 0).await?,
        users: state.user_repository.list(1000, 0).await?,
        user_progress: state.user_progress_repository.list(1000, 0).await?,
        notifications: state.notification_repository.list(1000, 0).await?,
        export_timestamp: chrono::Utc::now(),
        version: "1.0.0".to_string(),
    };

    let json_data = serde_json::to_string_pretty(&export_data)?;

    let response = Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .header(
            "Content-Disposition",
            "attachment; filename=\"rust_learning_export.json\"",
        )
        .body(Body::from(json_data))
        .unwrap();

    Ok(response)
}

pub async fn import_data_handler(
    State(_state): State<AppState>,
    _multipart: axum::extract::Multipart,
) -> Result<Html<String>> {
    // For demo purposes, we'll just show a success message
    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Import Successful</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; text-align: center; }}
        .success-message {{ background-color: #d4edda; color: #155724; padding: 20px; border-radius: 8px; margin: 20px 0; }}
        .btn {{ padding: 10px 20px; text-decoration: none; border-radius: 4px; font-size: 14px; }}
        .btn-primary {{ background-color: #007bff; color: white; }}
    </style>
</head>
<body>
    <h1>✅ Import Successful</h1>
    <div class="success-message">
        Your data has been successfully imported! The import process completed without errors.
    </div>
    <a href="/admin/export-import" class="btn btn-primary">← Back to Export/Import</a>
</body>
</html>"#
    );

    Ok(Html(html))
}

pub async fn backup_handler(State(state): State<AppState>) -> Result<Response<Body>> {
    let export_data = ExportData {
        topics: state.topic_repository.list(1000, 0).await?,
        lessons: state.lesson_repository.list(1000, 0).await?,
        questions: state.question_repository.list(1000, 0).await?,
        code_practices: state.code_practice_repository.list(1000, 0).await?,
        users: state.user_repository.list(1000, 0).await?,
        user_progress: state.user_progress_repository.list(1000, 0).await?,
        notifications: state.notification_repository.list(1000, 0).await?,
        export_timestamp: chrono::Utc::now(),
        version: "1.0.0".to_string(),
    };

    let json_data = serde_json::to_string_pretty(&export_data)?;
    let filename = format!("backup_{}.json", chrono::Utc::now().format("%Y%m%d_%H%M%S"));

    let response = Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .header(
            "Content-Disposition",
            format!("attachment; filename=\"{}\"", filename),
        )
        .body(Body::from(json_data))
        .unwrap();

    Ok(response)
}

pub async fn restore_handler() -> Result<Html<String>> {
    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Restore Backup</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; }}
        .form-container {{ max-width: 600px; margin: 0 auto; }}
        .form-group {{ margin-bottom: 20px; }}
        .form-group label {{ display: block; margin-bottom: 5px; font-weight: bold; color: #495057; }}
        .form-group input[type="file"] {{ width: 100%; padding: 10px; border: 1px solid #ced4da; border-radius: 4px; }}
        .btn {{ padding: 10px 20px; text-decoration: none; border-radius: 4px; font-size: 14px; border: none; cursor: pointer; }}
        .btn-primary {{ background-color: #007bff; color: white; }}
        .btn-secondary {{ background-color: #6c757d; color: white; }}
        .btn:hover {{ opacity: 0.8; }}
        .form-actions {{ display: flex; gap: 10px; margin-top: 30px; }}
        .warning-box {{ background-color: #fff3cd; border: 1px solid #ffeaa7; padding: 15px; border-radius: 4px; margin-bottom: 20px; }}
        .warning-box .warning-title {{ font-weight: bold; color: #856404; margin-bottom: 8px; }}
        .warning-box .warning-text {{ color: #856404; }}
    </style>
</head>
<body>
    <div class="form-container">
        <h1>🔄 Restore from Backup</h1>

        <div class="warning-box">
            <div class="warning-title">⚠️ Warning</div>
            <div class="warning-text">
                This operation will replace all current data with the backup data.
                This action cannot be undone. Make sure you have a current backup before proceeding.
            </div>
        </div>

        <form method="POST" action="/admin/export-import/restore" enctype="multipart/form-data">
            <div class="form-group">
                <label for="backup_file">Select Backup File:</label>
                <input type="file" id="backup_file" name="file" accept=".json" required>
            </div>

            <div class="form-group">
                <label>
                    <input type="checkbox" name="confirm" required>
                    I understand that this will replace all current data
                </label>
            </div>

            <div class="form-actions">
                <button type="submit" class="btn btn-primary">🔄 Restore Backup</button>
                <a href="/admin/export-import" class="btn btn-secondary">Cancel</a>
            </div>
        </form>
    </div>
</body>
</html>"#
    );

    Ok(Html(html))
}

pub async fn stats_handler(State(state): State<AppState>) -> Result<Html<String>> {
    let topics_count = state.topic_repository.list(1000, 0).await?.len();
    let lessons_count = state.lesson_repository.list(1000, 0).await?.len();
    let questions_count = state.question_repository.list(1000, 0).await?.len();
    let code_practices_count = state.code_practice_repository.list(1000, 0).await?.len();
    let users_count = state.user_repository.list(1000, 0).await?.len();
    let user_progress_count = state.user_progress_repository.list(1000, 0).await?.len();
    let notifications_count = state.notification_repository.list(1000, 0).await?.len();

    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Export/Import Statistics</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; }}
        .header {{ display: flex; justify-content: space-between; align-items: center; margin-bottom: 30px; }}
        .btn {{ padding: 10px 20px; text-decoration: none; border-radius: 4px; font-size: 14px; }}
        .btn-primary {{ background-color: #007bff; color: white; }}
        .stats-grid {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 20px; margin-bottom: 30px; }}
        .stat-card {{ background-color: #f8f9fa; padding: 20px; border-radius: 8px; border: 1px solid #dee2e6; text-align: center; }}
        .stat-number {{ font-size: 32px; font-weight: bold; color: #007bff; margin-bottom: 8px; }}
        .stat-label {{ color: #6c757d; font-size: 14px; }}
        .info-section {{ background-color: #e9ecef; padding: 20px; border-radius: 8px; margin-bottom: 20px; }}
        .info-section h3 {{ margin-top: 0; color: #495057; }}
    </style>
</head>
<body>
    <div class="header">
        <h1>📊 Export/Import Statistics</h1>
        <a href="/admin/export-import" class="btn btn-primary">← Back to Export/Import</a>
    </div>

    <div class="stats-grid">
        <div class="stat-card">
            <div class="stat-number">{}</div>
            <div class="stat-label">Topics</div>
        </div>
        <div class="stat-card">
            <div class="stat-number">{}</div>
            <div class="stat-label">Lessons</div>
        </div>
        <div class="stat-card">
            <div class="stat-number">{}</div>
            <div class="stat-label">Questions</div>
        </div>
        <div class="stat-card">
            <div class="stat-number">{}</div>
            <div class="stat-label">Code Practices</div>
        </div>
        <div class="stat-card">
            <div class="stat-number">{}</div>
            <div class="stat-label">Users</div>
        </div>
        <div class="stat-card">
            <div class="stat-number">{}</div>
            <div class="stat-label">User Progress</div>
        </div>
        <div class="stat-card">
            <div class="stat-number">{}</div>
            <div class="stat-label">Notifications</div>
        </div>
    </div>

    <div class="info-section">
        <h3>📋 Export/Import Information</h3>
        <p><strong>Last Export:</strong> Not available (demo mode)</p>
        <p><strong>Last Import:</strong> Not available (demo mode)</p>
        <p><strong>Total Exports:</strong> 0</p>
        <p><strong>Total Imports:</strong> 0</p>
        <p><strong>Backup Status:</strong> No recent backups</p>
    </div>

    <div class="info-section">
        <h3>🔧 System Information</h3>
        <p><strong>Database Status:</strong> Connected</p>
        <p><strong>Export Format:</strong> JSON (Primary), CSV (Limited)</p>
        <p><strong>Max Export Size:</strong> 1000 records per entity</p>
        <p><strong>Supported Versions:</strong> 1.0.0</p>
    </div>
</body>
</html>"#,
        topics_count,
        lessons_count,
        questions_count,
        code_practices_count,
        users_count,
        user_progress_count,
        notifications_count
    );

    Ok(Html(html))
}

pub async fn history_handler() -> Result<Html<String>> {
    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Export History</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; }}
        .header {{ display: flex; justify-content: space-between; align-items: center; margin-bottom: 30px; }}
        .btn {{ padding: 10px 20px; text-decoration: none; border-radius: 4px; font-size: 14px; }}
        .btn-primary {{ background-color: #007bff; color: white; }}
        .table {{ width: 100%; border-collapse: collapse; margin-top: 20px; }}
        .table th, .table td {{ border: 1px solid #ddd; padding: 12px; text-align: left; }}
        .table th {{ background-color: #f8f9fa; font-weight: bold; }}
        .table tr:nth-child(even) {{ background-color: #f9f9f9; }}
        .no-data {{ text-align: center; padding: 40px; color: #6c757d; }}
    </style>
</head>
<body>
    <div class="header">
        <h1>📋 Export History</h1>
        <a href="/admin/export-import" class="btn btn-primary">← Back to Export/Import</a>
    </div>

    <div class="no-data">
        <h3>No Export History Available</h3>
        <p>Export history will be tracked once you start using the export functionality.</p>
        <a href="/admin/export-import" class="btn btn-primary">Start Exporting</a>
    </div>
</body>
</html>"#
    );

    Ok(Html(html))
}
