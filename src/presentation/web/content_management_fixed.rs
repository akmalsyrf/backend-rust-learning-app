use crate::application::state::AppState;
use crate::shared::errors::Result;
use axum::{
    extract::{Path, Query, State},
    response::Html,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ContentFilterParams {
    pub status: Option<String>,
    pub content_type: Option<String>,
    pub author: Option<String>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

pub async fn content_management_handler(
    State(state): State<AppState>,
    Query(params): Query<ContentFilterParams>,
) -> Result<Html<String>> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(20);
    let offset = (page - 1) * limit;

    // Get content items (in a real implementation, you'd have a content management system)
    let topics = state.topic_repository.list(limit, offset).await?;
    let lessons = state.lesson_repository.list(limit, offset).await?;
    let questions = state.question_repository.list(limit, offset).await?;

    let mut content_html = String::new();

    // Topics
    for topic in &topics {
        let status_badge = match topic.created_at {
            date if date > chrono::Utc::now() - chrono::Duration::days(7) => {
                r#"<span class="badge badge-new">New</span>"#
            }
            _ => r#"<span class="badge badge-published">Published</span>"#
        };

        content_html.push_str(&format!(
            r#"<tr>
                <td>
                    <div class="content-info">
                        <div class="content-icon">📚</div>
                        <div class="content-details">
                            <div class="content-title">{}</div>
                            <div class="content-meta">Topic • Created by Admin</div>
                        </div>
                    </div>
                </td>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td>
                    <div class="content-actions">
                        <a href="/admin/topics/{}/edit" class="btn btn-sm btn-primary">Edit</a>
                        <a href="/admin/content/{}/versions" class="btn btn-sm btn-secondary">Versions</a>
                        <a href="/admin/content/{}/approve" class="btn btn-sm btn-success">Approve</a>
                        <a href="/admin/content/{}/reject" class="btn btn-sm btn-warning">Reject</a>
                    </div>
                </td>
            </tr>"#,
            topic.title.en,
            status_badge,
            topic.created_at.format("%Y-%m-%d"),
            topic.updated_at.format("%Y-%m-%d"),
            topic.id.to_string(),
            topic.id.to_string(),
            topic.id.to_string(),
            topic.id.to_string()
        ));
    }

    // Lessons
    for lesson in &lessons {
        let status_badge = match lesson.created_at {
            date if date > chrono::Utc::now() - chrono::Duration::days(7) => {
                r#"<span class="badge badge-new">New</span>"#
            }
            _ => r#"<span class="badge badge-published">Published</span>"#
        };

        content_html.push_str(&format!(
            r#"<tr>
                <td>
                    <div class="content-info">
                        <div class="content-icon">📖</div>
                        <div class="content-details">
                            <div class="content-title">{}</div>
                            <div class="content-meta">Lesson • Created by Admin</div>
                        </div>
                    </div>
                </td>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td>
                    <div class="content-actions">
                        <a href="/admin/lessons/{}/edit" class="btn btn-sm btn-primary">Edit</a>
                        <a href="/admin/content/{}/versions" class="btn btn-sm btn-secondary">Versions</a>
                        <a href="/admin/content/{}/approve" class="btn btn-sm btn-success">Approve</a>
                        <a href="/admin/content/{}/reject" class="btn btn-sm btn-warning">Reject</a>
                    </div>
                </td>
            </tr>"#,
            lesson.title.en,
            status_badge,
            lesson.created_at.format("%Y-%m-%d"),
            lesson.updated_at.format("%Y-%m-%d"),
            lesson.id.to_string(),
            lesson.id.to_string(),
            lesson.id.to_string(),
            lesson.id.to_string()
        ));
    }

    // Questions
    for question in &questions {
        let status_badge = match question.created_at {
            date if date > chrono::Utc::now() - chrono::Duration::days(7) => {
                r#"<span class="badge badge-new">New</span>"#
            }
            _ => r#"<span class="badge badge-published">Published</span>"#
        };

        content_html.push_str(&format!(
            r#"<tr>
                <td>
                    <div class="content-info">
                        <div class="content-icon">❓</div>
                        <div class="content-details">
                            <div class="content-title">{}</div>
                            <div class="content-meta">Question • Created by Admin</div>
                        </div>
                    </div>
                </td>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td>
                    <div class="content-actions">
                        <a href="/admin/questions/{}/edit" class="btn btn-sm btn-primary">Edit</a>
                        <a href="/admin/content/{}/versions" class="btn btn-sm btn-secondary">Versions</a>
                        <a href="/admin/content/{}/approve" class="btn btn-sm btn-success">Approve</a>
                        <a href="/admin/content/{}/reject" class="btn btn-sm btn-warning">Reject</a>
                    </div>
                </td>
            </tr>"#,
            question.prompt.en,
            status_badge,
            question.created_at.format("%Y-%m-%d"),
            question.updated_at.format("%Y-%m-%d"),
            question.id.to_string(),
            question.id.to_string(),
            question.id.to_string(),
            question.id.to_string()
        ));
    }

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Content Management - Rust Learning Platform</title>
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
        .content-info {{
            display: flex;
            align-items: center;
            gap: 15px;
        }}
        .content-icon {{
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
        .content-details {{
            flex: 1;
        }}
        .content-title {{
            font-weight: 600;
            color: #333;
            margin-bottom: 2px;
        }}
        .content-meta {{
            color: #666;
            font-size: 0.9em;
        }}
        .badge {{
            padding: 4px 8px;
            border-radius: 12px;
            font-size: 0.8em;
            font-weight: 600;
        }}
        .badge-new {{ background: #17a2b8; color: white; }}
        .badge-published {{ background: #28a745; color: white; }}
        .badge-pending {{ background: #ffc107; color: #212529; }}
        .badge-rejected {{ background: #dc3545; color: white; }}
        .content-actions {{
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
            <h1>📝 Content Management</h1>
            <p>Manage content, versions, and approval workflow</p>
        </div>

        <div class="content">
            <a href="/admin/dashboard" class="back-btn">← Back to Dashboard</a>

            <div class="stats-grid">
                <div class="stat-card">
                    <div class="stat-number">{}</div>
                    <div class="stat-label">Total Content</div>
                </div>
                <div class="stat-card">
                    <div class="stat-number">{}</div>
                    <div class="stat-label">Published</div>
                </div>
                <div class="stat-card">
                    <div class="stat-number">{}</div>
                    <div class="stat-label">Pending Review</div>
                </div>
                <div class="stat-card">
                    <div class="stat-number">{}</div>
                    <div class="stat-label">New This Week</div>
                </div>
            </div>

            <div class="filters">
                <div class="filter-row">
                    <div class="filter-group">
                        <label for="status">Status:</label>
                        <select id="status" name="status">
                            <option value="">All Status</option>
                            <option value="published">Published</option>
                            <option value="pending">Pending Review</option>
                            <option value="rejected">Rejected</option>
                            <option value="draft">Draft</option>
                        </select>
                    </div>
                    <div class="filter-group">
                        <label for="content_type">Content Type:</label>
                        <select id="content_type" name="content_type">
                            <option value="">All Types</option>
                            <option value="topics">Topics</option>
                            <option value="lessons">Lessons</option>
                            <option value="questions">Questions</option>
                        </select>
                    </div>
                    <div class="filter-group">
                        <label for="author">Author:</label>
                        <input type="text" id="author" name="author" placeholder="Search by author...">
                    </div>
                </div>
                <div class="filter-actions">
                    <button type="button" class="btn btn-primary" onclick="applyFilters()">Apply Filters</button>
                    <button type="button" class="btn btn-secondary" onclick="clearFilters()">Clear</button>
                    <a href="/admin/content/bulk-approve" class="btn btn-success">Bulk Approve</a>
                    <a href="/admin/content/export" class="btn btn-warning">Export Content</a>
                </div>
            </div>

            <div class="table-container">
                <table>
                    <thead>
                        <tr>
                            <th>Content</th>
                            <th>Status</th>
                            <th>Created</th>
                            <th>Updated</th>
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
            const status = document.getElementById('status').value;
            const contentType = document.getElementById('content_type').value;
            const author = document.getElementById('author').value;

            const params = new URLSearchParams();
            if (status) params.append('status', status);
            if (contentType) params.append('content_type', contentType);
            if (author) params.append('author', author);

            window.location.href = '?' + params.toString();
        }}

        function clearFilters() {{
            document.getElementById('status').value = '';
            document.getElementById('content_type').value = '';
            document.getElementById('author').value = '';
            window.location.href = '/admin/content';
        }}
    </script>
</body>
</html>"#,
        topics.len() + lessons.len() + questions.len(),
        topics.len() + lessons.len() + questions.len(), // In a real implementation, you'd count published
        0, // In a real implementation, you'd count pending
        topics.len() + lessons.len() + questions.len(), // In a real implementation, you'd count new this week
        content_html,
        if page > 1 { page - 1 } else { 1 },
        page,
        page + 1
    );

    Ok(Html(html))
}

pub async fn content_versions_handler(
    State(_state): State<AppState>,
    Path(_content_id): Path<String>,
) -> Result<Html<String>> {
    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Content Versions - Rust Learning Platform</title>
    <style>
        body {{
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            margin: 0;
            padding: 20px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
        }}
        .container {{
            max-width: 1000px;
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
        .version-item {{
            background: #f8f9fa;
            border-radius: 10px;
            padding: 20px;
            margin-bottom: 20px;
            border: 1px solid #e9ecef;
        }}
        .version-header {{
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 15px;
        }}
        .version-number {{
            font-weight: bold;
            color: #333;
            font-size: 1.1em;
        }}
        .version-status {{
            padding: 4px 8px;
            border-radius: 12px;
            font-size: 0.8em;
            font-weight: 600;
        }}
        .status-current {{ background: #28a745; color: white; }}
        .status-previous {{ background: #6c757d; color: white; }}
        .status-draft {{ background: #ffc107; color: #212529; }}
        .version-meta {{
            color: #666;
            font-size: 0.9em;
            margin-bottom: 10px;
        }}
        .version-actions {{
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
        .btn:hover {{ opacity: 0.8; }}
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
        .version-changes {{
            background: white;
            border-radius: 5px;
            padding: 15px;
            margin-top: 10px;
            border-left: 4px solid #007bff;
        }}
        .change-item {{
            margin: 5px 0;
            color: #333;
        }}
        .change-added {{ color: #28a745; }}
        .change-removed {{ color: #dc3545; }}
        .change-modified {{ color: #ffc107; }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>📚 Content Versions</h1>
            <p>Version history and change tracking</p>
        </div>

        <div class="content">
            <a href="/admin/content" class="back-btn">← Back to Content Management</a>

            <div class="version-item">
                <div class="version-header">
                    <div class="version-number">Version 3.0</div>
                    <div class="version-status status-current">Current</div>
                </div>
                <div class="version-meta">
                    Updated by Admin • 2024-01-15 14:30:00
                </div>
                <div class="version-changes">
                    <div class="change-item change-added">+ Added new section about advanced concepts</div>
                    <div class="change-item change-modified">~ Updated examples with better explanations</div>
                    <div class="change-item change-added">+ Added interactive code snippets</div>
                </div>
                <div class="version-actions">
                    <a href="#" class="btn btn-primary">View</a>
                    <a href="#" class="btn btn-secondary">Compare</a>
                    <a href="#" class="btn btn-success">Restore</a>
                </div>
            </div>

            <div class="version-item">
                <div class="version-header">
                    <div class="version-number">Version 2.1</div>
                    <div class="version-status status-previous">Previous</div>
                </div>
                <div class="version-meta">
                    Updated by Admin • 2024-01-10 09:15:00
                </div>
                <div class="version-changes">
                    <div class="change-item change-modified">~ Fixed typos in explanations</div>
                    <div class="change-item change-added">+ Added more practice exercises</div>
                </div>
                <div class="version-actions">
                    <a href="#" class="btn btn-primary">View</a>
                    <a href="#" class="btn btn-secondary">Compare</a>
                    <a href="#" class="btn btn-success">Restore</a>
                </div>
            </div>

            <div class="version-item">
                <div class="version-header">
                    <div class="version-number">Version 2.0</div>
                    <div class="version-status status-previous">Previous</div>
                </div>
                <div class="version-meta">
                    Updated by Admin • 2024-01-05 16:45:00
                </div>
                <div class="version-changes">
                    <div class="change-item change-added">+ Complete rewrite of content structure</div>
                    <div class="change-item change-added">+ Added multimedia content</div>
                    <div class="change-item change-removed">- Removed outdated examples</div>
                </div>
                <div class="version-actions">
                    <a href="#" class="btn btn-primary">View</a>
                    <a href="#" class="btn btn-secondary">Compare</a>
                    <a href="#" class="btn btn-success">Restore</a>
                </div>
            </div>

            <div class="version-item">
                <div class="version-header">
                    <div class="version-number">Version 1.0</div>
                    <div class="version-status status-previous">Previous</div>
                </div>
                <div class="version-meta">
                    Created by Admin • 2024-01-01 10:00:00
                </div>
                <div class="version-changes">
                    <div class="change-item change-added">+ Initial version created</div>
                    <div class="change-item change-added">+ Basic content structure</div>
                </div>
                <div class="version-actions">
                    <a href="#" class="btn btn-primary">View</a>
                    <a href="#" class="btn btn-secondary">Compare</a>
                    <a href="#" class="btn btn-success">Restore</a>
                </div>
            </div>
        </div>
    </div>
</body>
</html>"#
    );

    Ok(Html(html))
}

