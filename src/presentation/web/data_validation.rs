use crate::application::state::AppState;
use crate::shared::errors::Result;
use axum::{
    extract::{Query, State},
    response::Html,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ValidationParams {
    pub entity_type: Option<String>,
    pub validation_type: Option<String>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

pub async fn data_validation_handler(
    State(_state): State<AppState>,
    Query(params): Query<ValidationParams>,
) -> Result<Html<String>> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(20);
    let _offset = (page - 1) * limit;

    // In a real implementation, you'd run actual validation checks
    let validation_results = vec![
        ValidationResult {
            entity_type: "Topic".to_string(),
            entity_id: "topic_123".to_string(),
            entity_name: "Rust Basics".to_string(),
            validation_type: "Data Integrity".to_string(),
            status: "Pass".to_string(),
            message: "All required fields are present and valid".to_string(),
            severity: "Info".to_string(),
            timestamp: chrono::Utc::now(),
        },
        ValidationResult {
            entity_type: "Lesson".to_string(),
            entity_id: "lesson_456".to_string(),
            entity_name: "Variables and Types".to_string(),
            validation_type: "Content Quality".to_string(),
            status: "Warning".to_string(),
            message: "Lesson content is too short (less than 100 words)".to_string(),
            severity: "Warning".to_string(),
            timestamp: chrono::Utc::now(),
        },
        ValidationResult {
            entity_type: "Question".to_string(),
            entity_id: "question_789".to_string(),
            entity_name: "What is ownership in Rust?".to_string(),
            validation_type: "Answer Validation".to_string(),
            status: "Fail".to_string(),
            message: "Question has no correct answer specified".to_string(),
            severity: "Error".to_string(),
            timestamp: chrono::Utc::now(),
        },
        ValidationResult {
            entity_type: "User".to_string(),
            entity_id: "user_101".to_string(),
            entity_name: "john.doe@example.com".to_string(),
            validation_type: "Email Validation".to_string(),
            status: "Pass".to_string(),
            message: "Email format is valid".to_string(),
            severity: "Info".to_string(),
            timestamp: chrono::Utc::now(),
        },
        ValidationResult {
            entity_type: "Code Practice".to_string(),
            entity_id: "practice_202".to_string(),
            entity_name: "Implement a simple calculator".to_string(),
            validation_type: "Code Syntax".to_string(),
            status: "Warning".to_string(),
            message: "Code example has some syntax issues".to_string(),
            severity: "Warning".to_string(),
            timestamp: chrono::Utc::now(),
        },
    ];

    let mut results_html = String::new();
    for result in &validation_results {
        let status_class = match result.status.as_str() {
            "Pass" => "status-pass",
            "Warning" => "status-warning",
            "Fail" => "status-fail",
            _ => "status-unknown",
        };

        let severity_class = match result.severity.as_str() {
            "Info" => "severity-info",
            "Warning" => "severity-warning",
            "Error" => "severity-error",
            _ => "severity-unknown",
        };

        results_html.push_str(&format!(
            r#"<tr>
                <td>
                    <div class="entity-info">
                        <div class="entity-icon">{}</div>
                        <div class="entity-details">
                            <div class="entity-name">{}</div>
                            <div class="entity-id">ID: {}</div>
                        </div>
                    </div>
                </td>
                <td>{}</td>
                <td><span class="status {}">{}</span></td>
                <td><span class="severity {}">{}</span></td>
                <td>{}</td>
                <td>{}</td>
                <td>
                    <div class="validation-actions">
                        <a href="/admin/validation/{}/fix" class="btn btn-sm btn-primary">Fix</a>
                        <a href="/admin/validation/{}/ignore" class="btn btn-sm btn-secondary">Ignore</a>
                        <a href="/admin/validation/{}/details" class="btn btn-sm btn-info">Details</a>
                    </div>
                </td>
            </tr>"#,
            get_entity_icon(&result.entity_type),
            result.entity_name,
            result.entity_id,
            result.validation_type,
            status_class,
            result.status,
            severity_class,
            result.severity,
            result.message,
            result.timestamp.format("%Y-%m-%d %H:%M"),
            result.entity_id,
            result.entity_id,
            result.entity_id
        ));
    }

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Data Validation - Rust Learning Platform</title>
    <style>
        body {{
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            margin: 0;
            padding: 20px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
        }}
        .container {{
            max-width: 1400px;
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
        .entity-info {{
            display: flex;
            align-items: center;
            gap: 15px;
        }}
        .entity-icon {{
            width: 40px;
            height: 40px;
            border-radius: 8px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            display: flex;
            align-items: center;
            justify-content: center;
            color: white;
            font-size: 1.2em;
        }}
        .entity-details {{
            flex: 1;
        }}
        .entity-name {{
            font-weight: 600;
            color: #333;
            margin-bottom: 2px;
        }}
        .entity-id {{
            color: #666;
            font-size: 0.9em;
        }}
        .status {{
            padding: 4px 8px;
            border-radius: 12px;
            font-size: 0.8em;
            font-weight: 600;
        }}
        .status-pass {{ background: #d4edda; color: #155724; }}
        .status-warning {{ background: #fff3cd; color: #856404; }}
        .status-fail {{ background: #f8d7da; color: #721c24; }}
        .severity {{
            padding: 4px 8px;
            border-radius: 12px;
            font-size: 0.8em;
            font-weight: 600;
        }}
        .severity-info {{ background: #d1ecf1; color: #0c5460; }}
        .severity-warning {{ background: #fff3cd; color: #856404; }}
        .severity-error {{ background: #f8d7da; color: #721c24; }}
        .validation-actions {{
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
            <h1>🔍 Data Validation</h1>
            <p>Validate data integrity and quality across the platform</p>
        </div>

        <div class="content">
            <a href="/admin/dashboard" class="back-btn">← Back to Dashboard</a>

            <div class="stats-grid">
                <div class="stat-card">
                    <div class="stat-number">{}</div>
                    <div class="stat-label">Total Validations</div>
                </div>
                <div class="stat-card">
                    <div class="stat-number">{}</div>
                    <div class="stat-label">Passed</div>
                </div>
                <div class="stat-card">
                    <div class="stat-number">{}</div>
                    <div class="stat-label">Warnings</div>
                </div>
                <div class="stat-card">
                    <div class="stat-number">{}</div>
                    <div class="stat-label">Failed</div>
                </div>
            </div>

            <div class="filters">
                <div class="filter-row">
                    <div class="filter-group">
                        <label for="entity_type">Entity Type:</label>
                        <select id="entity_type" name="entity_type">
                            <option value="">All Types</option>
                            <option value="topics">Topics</option>
                            <option value="lessons">Lessons</option>
                            <option value="questions">Questions</option>
                            <option value="users">Users</option>
                            <option value="code_practices">Code Practices</option>
                        </select>
                    </div>
                    <div class="filter-group">
                        <label for="validation_type">Validation Type:</label>
                        <select id="validation_type" name="validation_type">
                            <option value="">All Types</option>
                            <option value="data_integrity">Data Integrity</option>
                            <option value="content_quality">Content Quality</option>
                            <option value="email_validation">Email Validation</option>
                            <option value="code_syntax">Code Syntax</option>
                        </select>
                    </div>
                    <div class="filter-group">
                        <label for="status">Status:</label>
                        <select id="status" name="status">
                            <option value="">All Status</option>
                            <option value="pass">Pass</option>
                            <option value="warning">Warning</option>
                            <option value="fail">Fail</option>
                        </select>
                    </div>
                </div>
                <div class="filter-actions">
                    <button type="button" class="btn btn-primary" onclick="applyFilters()">Apply Filters</button>
                    <button type="button" class="btn btn-secondary" onclick="clearFilters()">Clear</button>
                    <button type="button" class="btn btn-success" onclick="runValidation()">Run Validation</button>
                    <button type="button" class="btn btn-warning" onclick="fixAll()">Fix All</button>
                </div>
            </div>

            <div class="table-container">
                <table>
                    <thead>
                        <tr>
                            <th>Entity</th>
                            <th>Validation Type</th>
                            <th>Status</th>
                            <th>Severity</th>
                            <th>Message</th>
                            <th>Timestamp</th>
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
            const entityType = document.getElementById('entity_type').value;
            const validationType = document.getElementById('validation_type').value;
            const status = document.getElementById('status').value;

            const params = new URLSearchParams();
            if (entityType) params.append('entity_type', entityType);
            if (validationType) params.append('validation_type', validationType);
            if (status) params.append('status', status);

            window.location.href = '?' + params.toString();
        }}

        function clearFilters() {{
            document.getElementById('entity_type').value = '';
            document.getElementById('validation_type').value = '';
            document.getElementById('status').value = '';
            window.location.href = '/admin/validation';
        }}

        function runValidation() {{
            if (confirm('This will run validation on all entities. Continue?')) {{
                // In a real implementation, this would trigger validation
                alert('Validation started. Please refresh the page in a few moments.');
            }}
        }}

        function fixAll() {{
            if (confirm('This will attempt to fix all fixable validation issues. Continue?')) {{
                // In a real implementation, this would trigger auto-fix
                alert('Auto-fix started. Please refresh the page in a few moments.');
            }}
        }}
    </script>
</body>
</html>"#,
        validation_results.len(),
        validation_results
            .iter()
            .filter(|r| r.status == "Pass")
            .count(),
        validation_results
            .iter()
            .filter(|r| r.status == "Warning")
            .count(),
        validation_results
            .iter()
            .filter(|r| r.status == "Fail")
            .count(),
        results_html,
        if page > 1 { page - 1 } else { 1 },
        page,
        page + 1
    );

    Ok(Html(html))
}

fn get_entity_icon(entity_type: &str) -> &'static str {
    match entity_type {
        "Topic" => "📚",
        "Lesson" => "📖",
        "Question" => "❓",
        "User" => "👤",
        "Code Practice" => "💻",
        _ => "📄",
    }
}

#[derive(Debug)]
struct ValidationResult {
    entity_type: String,
    entity_id: String,
    entity_name: String,
    validation_type: String,
    status: String,
    message: String,
    severity: String,
    timestamp: chrono::DateTime<chrono::Utc>,
}
