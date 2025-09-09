use crate::application::state::AppState;
use crate::shared::errors::Result;
use axum::{
    extract::{Query, State},
    response::Html,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AuditFilterParams {
    pub action: Option<String>,
    pub entity_type: Option<String>,
    pub user: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

pub async fn audit_logging_handler(
    State(_state): State<AppState>,
    Query(params): Query<AuditFilterParams>,
) -> Result<Html<String>> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(20);
    let _offset = (page - 1) * limit;

    // In a real implementation, you'd fetch audit logs from a database
    let mut audit_logs = Vec::new();

    // Simulate audit log entries
    audit_logs.push(AuditLogEntry {
        id: "audit_001".to_string(),
        action: "CREATE".to_string(),
        entity_type: "Topic".to_string(),
        entity_id: "topic_123".to_string(),
        entity_name: "Rust Basics".to_string(),
        user_id: "user_456".to_string(),
        user_name: "Admin User".to_string(),
        changes: Some("Created new topic with title 'Rust Basics' and description 'Introduction to Rust programming language'".to_string()),
        ip_address: "192.168.1.100".to_string(),
        user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36".to_string(),
        timestamp: chrono::Utc::now() - chrono::Duration::hours(2),
    });

    audit_logs.push(AuditLogEntry {
        id: "audit_002".to_string(),
        action: "UPDATE".to_string(),
        entity_type: "Lesson".to_string(),
        entity_id: "lesson_789".to_string(),
        entity_name: "Variables and Types".to_string(),
        user_id: "user_456".to_string(),
        user_name: "Admin User".to_string(),
        changes: Some(
            "Updated lesson content: Added new examples and improved explanations".to_string(),
        ),
        ip_address: "192.168.1.100".to_string(),
        user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36".to_string(),
        timestamp: chrono::Utc::now() - chrono::Duration::hours(4),
    });

    audit_logs.push(AuditLogEntry {
        id: "audit_003".to_string(),
        action: "DELETE".to_string(),
        entity_type: "Question".to_string(),
        entity_id: "question_101".to_string(),
        entity_name: "What is ownership in Rust?".to_string(),
        user_id: "user_789".to_string(),
        user_name: "Moderator User".to_string(),
        changes: Some("Deleted question due to incorrect answer options".to_string()),
        ip_address: "192.168.1.101".to_string(),
        user_agent: "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36"
            .to_string(),
        timestamp: chrono::Utc::now() - chrono::Duration::hours(6),
    });

    audit_logs.push(AuditLogEntry {
        id: "audit_004".to_string(),
        action: "LOGIN".to_string(),
        entity_type: "User".to_string(),
        entity_id: "user_456".to_string(),
        entity_name: "Admin User".to_string(),
        user_id: "user_456".to_string(),
        user_name: "Admin User".to_string(),
        changes: None,
        ip_address: "192.168.1.100".to_string(),
        user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36".to_string(),
        timestamp: chrono::Utc::now() - chrono::Duration::hours(8),
    });

    audit_logs.push(AuditLogEntry {
        id: "audit_005".to_string(),
        action: "PERMISSION_CHANGE".to_string(),
        entity_type: "User".to_string(),
        entity_id: "user_789".to_string(),
        entity_name: "Moderator User".to_string(),
        user_id: "user_456".to_string(),
        user_name: "Admin User".to_string(),
        changes: Some("Granted moderator permissions to user".to_string()),
        ip_address: "192.168.1.100".to_string(),
        user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36".to_string(),
        timestamp: chrono::Utc::now() - chrono::Duration::hours(12),
    });

    let mut logs_html = String::new();
    for log in &audit_logs {
        let action_class = match log.action.as_str() {
            "CREATE" => "action-create",
            "UPDATE" => "action-update",
            "DELETE" => "action-delete",
            "LOGIN" => "action-login",
            "LOGOUT" => "action-logout",
            "PERMISSION_CHANGE" => "action-permission",
            _ => "action-other",
        };

        let action_icon = match log.action.as_str() {
            "CREATE" => "➕",
            "UPDATE" => "✏️",
            "DELETE" => "🗑️",
            "LOGIN" => "🔐",
            "LOGOUT" => "🚪",
            "PERMISSION_CHANGE" => "🔑",
            _ => "📝",
        };

        logs_html.push_str(&format!(
            r#"<tr>
                <td>
                    <div class="log-info">
                        <div class="log-icon {}">{}</div>
                        <div class="log-details">
                            <div class="log-action">{}</div>
                            <div class="log-entity">{}</div>
                        </div>
                    </div>
                </td>
                <td>
                    <div class="entity-info">
                        <div class="entity-name">{}</div>
                        <div class="entity-id">ID: {}</div>
                    </div>
                </td>
                <td>
                    <div class="user-info">
                        <div class="user-name">{}</div>
                        <div class="user-id">ID: {}</div>
                    </div>
                </td>
                <td>{}</td>
                <td>
                    <div class="log-meta">
                        <div class="ip-address">IP: {}</div>
                        <div class="user-agent">{}</div>
                    </div>
                </td>
                <td>
                    <div class="log-actions">
                        <a href="/admin/audit/{}/details" class="btn btn-sm btn-primary">Details</a>
                        <a href="/admin/audit/{}/revert" class="btn btn-sm btn-warning">Revert</a>
                    </div>
                </td>
            </tr>"#,
            action_class,
            action_icon,
            log.action,
            log.entity_type,
            log.entity_name,
            log.entity_id,
            log.user_name,
            log.user_id,
            log.timestamp.format("%Y-%m-%d %H:%M:%S"),
            log.ip_address,
            log.user_agent,
            log.id,
            log.id
        ));
    }

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Audit Logging - Rust Learning Platform</title>
    <style>
        body {{
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            margin: 0;
            padding: 20px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
        }}
        .container {{
            max-width: 1600px;
            margin: 0 auto;
            background: white;
            border-radius: 15px;
            box-shadow: 0 20px 40px rgba(0,0,0,0.1);
            overflow: hidden;
        }}
        .header {{
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            padding: 30px;
            text-align: center;
        }}
        .header h1 {{
            margin: 0;
            font-size: 2.5em;
            font-weight: 300;
        }}
        .content {{
            padding: 40px;
        }}
        .filters {{
            background: #f8f9fa;
            border-radius: 10px;
            padding: 20px;
            margin-bottom: 30px;
        }}
        .filter-row {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 15px;
            margin-bottom: 15px;
        }}
        .filter-group {{
            display: flex;
            flex-direction: column;
        }}
        .filter-group label {{
            font-weight: 600;
            margin-bottom: 5px;
            color: #333;
        }}
        .filter-group select, .filter-group input {{
            padding: 8px 12px;
            border: 1px solid #ddd;
            border-radius: 5px;
            font-size: 0.9em;
        }}
        .filter-actions {{
            display: flex;
            gap: 10px;
            margin-top: 15px;
        }}
        .btn {{
            padding: 8px 16px;
            border: none;
            border-radius: 5px;
            cursor: pointer;
            font-size: 0.9em;
            text-decoration: none;
            display: inline-block;
            transition: all 0.3s ease;
        }}
        .btn-primary {{ background: #007bff; color: white; }}
        .btn-secondary {{ background: #6c757d; color: white; }}
        .btn-success {{ background: #28a745; color: white; }}
        .btn-warning {{ background: #ffc107; color: #212529; }}
        .btn-danger {{ background: #dc3545; color: white; }}
        .btn-info {{ background: #17a2b8; color: white; }}
        .btn-sm {{ padding: 6px 12px; font-size: 0.8em; }}
        .btn:hover {{ opacity: 0.8; }}
        .table-container {{
            overflow-x: auto;
            border-radius: 10px;
            border: 1px solid #ddd;
        }}
        table {{
            width: 100%;
            border-collapse: collapse;
            background: white;
        }}
        th, td {{
            padding: 15px;
            text-align: left;
            border-bottom: 1px solid #eee;
        }}
        th {{
            background: #f8f9fa;
            font-weight: 600;
            color: #333;
        }}
        .log-info {{
            display: flex;
            align-items: center;
            gap: 15px;
        }}
        .log-icon {{
            width: 40px;
            height: 40px;
            border-radius: 8px;
            display: flex;
            align-items: center;
            justify-content: center;
            color: white;
            font-size: 1.2em;
        }}
        .action-create {{ background: #28a745; }}
        .action-update {{ background: #ffc107; color: #212529; }}
        .action-delete {{ background: #dc3545; }}
        .action-login {{ background: #17a2b8; }}
        .action-logout {{ background: #6c757d; }}
        .action-permission {{ background: #6f42c1; }}
        .action-other {{ background: #6c757d; }}
        .log-details {{
            flex: 1;
        }}
        .log-action {{
            font-weight: 600;
            color: #333;
            margin-bottom: 2px;
        }}
        .log-entity {{
            color: #666;
            font-size: 0.9em;
        }}
        .entity-info, .user-info {{
            display: flex;
            flex-direction: column;
        }}
        .entity-name, .user-name {{
            font-weight: 600;
            color: #333;
            margin-bottom: 2px;
        }}
        .entity-id, .user-id {{
            color: #666;
            font-size: 0.9em;
        }}
        .log-meta {{
            display: flex;
            flex-direction: column;
        }}
        .ip-address {{
            font-weight: 600;
            color: #333;
            margin-bottom: 2px;
        }}
        .user-agent {{
            color: #666;
            font-size: 0.8em;
            max-width: 200px;
            overflow: hidden;
            text-overflow: ellipsis;
            white-space: nowrap;
        }}
        .log-actions {{
            display: flex;
            gap: 5px;
            flex-wrap: wrap;
        }}
        .stats-grid {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 20px;
            margin-bottom: 30px;
        }}
        .stat-card {{
            background: #f8f9fa;
            padding: 20px;
            border-radius: 10px;
            text-align: center;
        }}
        .stat-number {{
            font-size: 2em;
            font-weight: bold;
            color: #333;
            margin-bottom: 5px;
        }}
        .stat-label {{
            color: #666;
            font-size: 0.9em;
        }}
        .back-btn {{
            display: inline-block;
            margin-bottom: 20px;
            color: #667eea;
            text-decoration: none;
            font-weight: 600;
        }}
        .back-btn:hover {{
            text-decoration: underline;
        }}
        .pagination {{
            display: flex;
            justify-content: center;
            margin-top: 30px;
            gap: 10px;
        }}
        .pagination a, .pagination span {{
            padding: 8px 12px;
            border: 1px solid #ddd;
            border-radius: 5px;
            text-decoration: none;
            color: #333;
        }}
        .pagination .current {{
            background: #007bff;
            color: white;
            border-color: #007bff;
        }}
        .pagination a:hover {{
            background: #f8f9fa;
        }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>📋 Audit Logging</h1>
            <p>Track all system changes and user activities</p>
        </div>

        <div class="content">
            <a href="/admin/dashboard" class="back-btn">← Back to Dashboard</a>

            <div class="stats-grid">
                <div class="stat-card">
                    <div class="stat-number">{}</div>
                    <div class="stat-label">Total Logs</div>
                </div>
                <div class="stat-card">
                    <div class="stat-number">{}</div>
                    <div class="stat-label">Today</div>
                </div>
                <div class="stat-card">
                    <div class="stat-number">{}</div>
                    <div class="stat-label">This Week</div>
                </div>
                <div class="stat-card">
                    <div class="stat-number">{}</div>
                    <div class="stat-label">Active Users</div>
                </div>
            </div>

            <div class="filters">
                <div class="filter-row">
                    <div class="filter-group">
                        <label for="action">Action:</label>
                        <select id="action" name="action">
                            <option value="">All Actions</option>
                            <option value="CREATE">Create</option>
                            <option value="UPDATE">Update</option>
                            <option value="DELETE">Delete</option>
                            <option value="LOGIN">Login</option>
                            <option value="LOGOUT">Logout</option>
                            <option value="PERMISSION_CHANGE">Permission Change</option>
                        </select>
                    </div>
                    <div class="filter-group">
                        <label for="entity_type">Entity Type:</label>
                        <select id="entity_type" name="entity_type">
                            <option value="">All Types</option>
                            <option value="Topic">Topic</option>
                            <option value="Lesson">Lesson</option>
                            <option value="Question">Question</option>
                            <option value="User">User</option>
                            <option value="Code Practice">Code Practice</option>
                        </select>
                    </div>
                    <div class="filter-group">
                        <label for="user">User:</label>
                        <input type="text" id="user" name="user" placeholder="Search by user name...">
                    </div>
                    <div class="filter-group">
                        <label for="date_from">From Date:</label>
                        <input type="date" id="date_from" name="date_from">
                    </div>
                    <div class="filter-group">
                        <label for="date_to">To Date:</label>
                        <input type="date" id="date_to" name="date_to">
                    </div>
                </div>
                <div class="filter-actions">
                    <button type="button" class="btn btn-primary" onclick="applyFilters()">Apply Filters</button>
                    <button type="button" class="btn btn-secondary" onclick="clearFilters()">Clear</button>
                    <button type="button" class="btn btn-success" onclick="exportLogs()">Export Logs</button>
                    <button type="button" class="btn btn-warning" onclick="clearOldLogs()">Clear Old Logs</button>
                </div>
            </div>

            <div class="table-container">
                <table>
                    <thead>
                        <tr>
                            <th>Action</th>
                            <th>Entity</th>
                            <th>User</th>
                            <th>Timestamp</th>
                            <th>Details</th>
                            <th>Actions</th>
                        </tr>
                    </thead>
                    <tbody>
                        {}
                    </tbody>
                </table>
            </div>

            <div class="pagination">
                <a href="?page={}">Previous</a>
                <span class="current">{}</span>
                <a href="?page={}">Next</a>
            </div>
        </div>
    </div>

    <script>
        function applyFilters() {{
            const action = document.getElementById('action').value;
            const entityType = document.getElementById('entity_type').value;
            const user = document.getElementById('user').value;
            const dateFrom = document.getElementById('date_from').value;
            const dateTo = document.getElementById('date_to').value;

            const params = new URLSearchParams();
            if (action) params.append('action', action);
            if (entityType) params.append('entity_type', entityType);
            if (user) params.append('user', user);
            if (dateFrom) params.append('date_from', dateFrom);
            if (dateTo) params.append('date_to', dateTo);

            window.location.href = '?' + params.toString();
        }}

        function clearFilters() {{
            document.getElementById('action').value = '';
            document.getElementById('entity_type').value = '';
            document.getElementById('user').value = '';
            document.getElementById('date_from').value = '';
            document.getElementById('date_to').value = '';
            window.location.href = '/admin/audit';
        }}

        function exportLogs() {{
            if (confirm('Export audit logs to CSV file?')) {{
                // In a real implementation, this would trigger CSV export
                alert('Export started. File will be downloaded shortly.');
            }}
        }}

        function clearOldLogs() {{
            if (confirm('This will delete audit logs older than 90 days. Continue?')) {{
                // In a real implementation, this would clear old logs
                alert('Old logs cleared successfully.');
            }}
        }}
    </script>
</body>
</html>"#,
        audit_logs.len(),
        audit_logs
            .iter()
            .filter(|l| l.timestamp.date_naive() == chrono::Utc::now().date_naive())
            .count(),
        audit_logs
            .iter()
            .filter(|l| l.timestamp > chrono::Utc::now() - chrono::Duration::days(7))
            .count(),
        audit_logs
            .iter()
            .map(|l| &l.user_id)
            .collect::<std::collections::HashSet<_>>()
            .len(),
        logs_html,
        if page > 1 { page - 1 } else { 1 },
        page,
        page + 1
    );

    Ok(Html(html))
}

#[derive(Debug)]
struct AuditLogEntry {
    id: String,
    action: String,
    entity_type: String,
    entity_id: String,
    entity_name: String,
    user_id: String,
    user_name: String,
    changes: Option<String>,
    ip_address: String,
    user_agent: String,
    timestamp: chrono::DateTime<chrono::Utc>,
}
