use crate::application::state::AppState;
use crate::shared::errors::Result;
use axum::{
    extract::{Path, Query, State},
    response::Html,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UserFilterParams {
    pub role: Option<String>,
    pub status: Option<String>,
    pub search: Option<String>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

pub async fn user_management_handler(
    State(state): State<AppState>,
    Query(params): Query<UserFilterParams>,
) -> Result<Html<String>> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(20);
    let offset = (page - 1) * limit;

    // Get all users (in a real implementation, you'd filter by role/status)
    let users = state.user_repository.list(limit, offset).await?;
    let total_users = state.user_repository.list(1000, 0).await?.len();

    let mut users_html = String::new();
    for user in &users {
        let role_badge = match user.display_name.as_str() {
            name if name.contains("admin") || name.contains("Admin") => {
                r#"<span class="badge badge-admin">Admin</span>"#
            }
            name if name.contains("moderator") || name.contains("Moderator") => {
                r#"<span class="badge badge-moderator">Moderator</span>"#
            }
            _ => r#"<span class="badge badge-user">User</span>"#,
        };

        let status_badge = if user.created_at < chrono::Utc::now() - chrono::Duration::days(30) {
            r#"<span class="badge badge-active">Active</span>"#
        } else {
            r#"<span class="badge badge-new">New</span>"#
        };

        users_html.push_str(&format!(
            r#"<tr>
                <td>
                    <div class="user-info">
                        <div class="user-avatar">{}</div>
                        <div class="user-details">
                            <div class="user-name">{}</div>
                            <div class="user-email">{}</div>
                        </div>
                    </div>
                </td>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td>
                    <div class="user-actions">
                        <a href="/admin/users/{}/profile" class="btn btn-sm btn-primary">View</a>
                        <a href="/admin/users/{}/edit" class="btn btn-sm btn-secondary">Edit</a>
                        <a href="/admin/users/{}/permissions" class="btn btn-sm btn-warning">Permissions</a>
                        <a href="/admin/users/{}/delete" class="btn btn-sm btn-danger" onclick="return confirm('Are you sure?')">Delete</a>
                    </div>
                </td>
            </tr>"#,
            "👤",
            user.display_name,
            user.email,
            role_badge,
            status_badge,
            user.created_at.format("%Y-%m-%d"),
            user.id.to_string(),
            user.id.to_string(),
            user.id.to_string(),
            user.id.to_string()
        ));
    }

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>User Management - Rust Learning Platform</title>
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
        .user-info {{
            display: flex;
            align-items: center;
            gap: 15px;
        }}
        .user-avatar {{
            width: 40px;
            height: 40px;
            border-radius: 50%;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            display: flex;
            align-items: center;
            justify-content: center;
            color: white;
            font-size: 1.2em;
        }}
        .user-details {{
            flex: 1;
        }}
        .user-name {{
            font-weight: 600;
            color: #333;
            margin-bottom: 2px;
        }}
        .user-email {{
            color: #666;
            font-size: 0.9em;
        }}
        .badge {{
            padding: 4px 8px;
            border-radius: 12px;
            font-size: 0.8em;
            font-weight: 600;
        }}
        .badge-admin {{ background: #dc3545; color: white; }}
        .badge-moderator {{ background: #ffc107; color: #212529; }}
        .badge-user {{ background: #6c757d; color: white; }}
        .badge-active {{ background: #28a745; color: white; }}
        .badge-new {{ background: #17a2b8; color: white; }}
        .user-actions {{
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
            <h1>👥 User Management</h1>
            <p>Manage users, roles, and permissions</p>
        </div>

        <div class="content">
            <a href="/admin/dashboard" class="back-btn">← Back to Dashboard</a>

            <div class="stats-grid">
                <div class="stat-card">
                    <div class="stat-number">{}</div>
                    <div class="stat-label">Total Users</div>
                </div>
                <div class="stat-card">
                    <div class="stat-number">{}</div>
                    <div class="stat-label">Active Users</div>
                </div>
                <div class="stat-card">
                    <div class="stat-number">{}</div>
                    <div class="stat-label">New Users (30 days)</div>
                </div>
                <div class="stat-card">
                    <div class="stat-number">{}</div>
                    <div class="stat-label">Admin Users</div>
                </div>
            </div>

            <div class="filters">
                <div class="filter-row">
                    <div class="filter-group">
                        <label for="role">Role:</label>
                        <select id="role" name="role">
                            <option value="">All Roles</option>
                            <option value="admin">Admin</option>
                            <option value="moderator">Moderator</option>
                            <option value="user">User</option>
                        </select>
                    </div>
                    <div class="filter-group">
                        <label for="status">Status:</label>
                        <select id="status" name="status">
                            <option value="">All Status</option>
                            <option value="active">Active</option>
                            <option value="new">New</option>
                            <option value="inactive">Inactive</option>
                        </select>
                    </div>
                    <div class="filter-group">
                        <label for="search">Search:</label>
                        <input type="text" id="search" name="search" placeholder="Search by name or email...">
                    </div>
                </div>
                <div class="filter-actions">
                    <button type="button" class="btn btn-primary" onclick="applyFilters()">Apply Filters</button>
                    <button type="button" class="btn btn-secondary" onclick="clearFilters()">Clear</button>
                    <a href="/admin/users/new" class="btn btn-success">Add New User</a>
                    <a href="/admin/bulk-operations" class="btn btn-warning">Bulk Operations</a>
                </div>
            </div>

            <div class="table-container">
                <table>
                    <thead>
                        <tr>
                            <th>User</th>
                            <th>Role</th>
                            <th>Status</th>
                            <th>Joined</th>
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
            const role = document.getElementById('role').value;
            const status = document.getElementById('status').value;
            const search = document.getElementById('search').value;

            const params = new URLSearchParams();
            if (role) params.append('role', role);
            if (status) params.append('status', status);
            if (search) params.append('search', search);

            window.location.href = '?' + params.toString();
        }}

        function clearFilters() {{
            document.getElementById('role').value = '';
            document.getElementById('status').value = '';
            document.getElementById('search').value = '';
            window.location.href = '/admin/users';
        }}
    </script>
</body>
</html>"#,
        total_users,
        total_users, // In a real implementation, you'd count active users
        total_users, // In a real implementation, you'd count new users
        1,           // In a real implementation, you'd count admin users
        users_html,
        if page > 1 { page - 1 } else { 1 },
        page,
        page + 1
    );

    Ok(Html(html))
}

pub async fn user_permissions_handler(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> Result<Html<String>> {
    let user_id = crate::domain::value_objects::UserId::from_string(user_id)
        .map_err(crate::shared::errors::AppError::Validation)?;

    let user = state
        .user_repository
        .find_by_id(&user_id)
        .await
        .map_err(|e| crate::shared::errors::AppError::Internal(e.to_string()))?
        .ok_or_else(|| crate::shared::errors::AppError::NotFound("User not found".to_string()))?;

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>User Permissions - Rust Learning Platform</title>
    <style>
        body {{
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            margin: 0;
            padding: 20px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
        }}
        .container {{
            max-width: 800px;
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
        .permission-section {{
            margin-bottom: 30px;
        }}
        .permission-section h3 {{
            color: #333;
            margin-bottom: 15px;
            font-size: 1.3em;
        }}
        .permission-grid {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
            gap: 15px;
        }}
        .permission-item {{
            display: flex;
            align-items: center;
            padding: 15px;
            background: #f8f9fa;
            border-radius: 8px;
            border: 1px solid #e9ecef;
        }}
        .permission-item input[type="checkbox"] {{
            margin-right: 10px;
            transform: scale(1.2);
        }}
        .permission-item label {{
            flex: 1;
            font-weight: 500;
            color: #333;
            cursor: pointer;
        }}
        .permission-description {{
            font-size: 0.9em;
            color: #666;
            margin-top: 5px;
        }}
        .btn {{
            padding: 12px 24px;
            border: none;
            border-radius: 25px;
            font-size: 1em;
            cursor: pointer;
            transition: all 0.3s ease;
            text-decoration: none;
            display: inline-block;
            text-align: center;
        }}
        .btn-primary {{
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
        }}
        .btn-secondary {{
            background: #6c757d;
            color: white;
        }}
        .btn:hover {{
            transform: translateY(-2px);
            box-shadow: 0 10px 20px rgba(0,0,0,0.2);
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
        .user-info {{
            background: #f8f9fa;
            padding: 20px;
            border-radius: 10px;
            margin-bottom: 30px;
            text-align: center;
        }}
        .user-avatar {{
            width: 80px;
            height: 80px;
            border-radius: 50%;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            display: flex;
            align-items: center;
            justify-content: center;
            color: white;
            font-size: 2em;
            margin: 0 auto 15px;
        }}
        .user-name {{
            font-size: 1.5em;
            font-weight: 600;
            color: #333;
            margin-bottom: 5px;
        }}
        .user-email {{
            color: #666;
        }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🔐 User Permissions</h1>
        </div>

        <div class="content">
            <a href="/admin/users" class="back-btn">← Back to Users</a>

            <div class="user-info">
                <div class="user-avatar">👤</div>
                <div class="user-name">{}</div>
                <div class="user-email">{}</div>
            </div>

            <form method="POST" action="/admin/users/{}/permissions">
                <div class="permission-section">
                    <h3>📚 Content Management</h3>
                    <div class="permission-grid">
                        <div class="permission-item">
                            <input type="checkbox" id="create_topics" name="permissions[]" value="create_topics">
                            <label for="create_topics">
                                Create Topics
                                <div class="permission-description">Allow user to create new topics</div>
                            </label>
                        </div>
                        <div class="permission-item">
                            <input type="checkbox" id="edit_topics" name="permissions[]" value="edit_topics">
                            <label for="edit_topics">
                                Edit Topics
                                <div class="permission-description">Allow user to edit existing topics</div>
                            </label>
                        </div>
                        <div class="permission-item">
                            <input type="checkbox" id="delete_topics" name="permissions[]" value="delete_topics">
                            <label for="delete_topics">
                                Delete Topics
                                <div class="permission-description">Allow user to delete topics</div>
                            </label>
                        </div>
                        <div class="permission-item">
                            <input type="checkbox" id="create_lessons" name="permissions[]" value="create_lessons">
                            <label for="create_lessons">
                                Create Lessons
                                <div class="permission-description">Allow user to create new lessons</div>
                            </label>
                        </div>
                        <div class="permission-item">
                            <input type="checkbox" id="edit_lessons" name="permissions[]" value="edit_lessons">
                            <label for="edit_lessons">
                                Edit Lessons
                                <div class="permission-description">Allow user to edit existing lessons</div>
                            </label>
                        </div>
                        <div class="permission-item">
                            <input type="checkbox" id="delete_lessons" name="permissions[]" value="delete_lessons">
                            <label for="delete_lessons">
                                Delete Lessons
                                <div class="permission-description">Allow user to delete lessons</div>
                            </label>
                        </div>
                    </div>
                </div>

                <div class="permission-section">
                    <h3>👥 User Management</h3>
                    <div class="permission-grid">
                        <div class="permission-item">
                            <input type="checkbox" id="view_users" name="permissions[]" value="view_users">
                            <label for="view_users">
                                View Users
                                <div class="permission-description">Allow user to view user list</div>
                            </label>
                        </div>
                        <div class="permission-item">
                            <input type="checkbox" id="edit_users" name="permissions[]" value="edit_users">
                            <label for="edit_users">
                                Edit Users
                                <div class="permission-description">Allow user to edit user profiles</div>
                            </label>
                        </div>
                        <div class="permission-item">
                            <input type="checkbox" id="delete_users" name="permissions[]" value="delete_users">
                            <label for="delete_users">
                                Delete Users
                                <div class="permission-description">Allow user to delete users</div>
                            </label>
                        </div>
                        <div class="permission-item">
                            <input type="checkbox" id="manage_permissions" name="permissions[]" value="manage_permissions">
                            <label for="manage_permissions">
                                Manage Permissions
                                <div class="permission-description">Allow user to manage user permissions</div>
                            </label>
                        </div>
                    </div>
                </div>

                <div class="permission-section">
                    <h3>📊 System Administration</h3>
                    <div class="permission-grid">
                        <div class="permission-item">
                            <input type="checkbox" id="view_analytics" name="permissions[]" value="view_analytics">
                            <label for="view_analytics">
                                View Analytics
                                <div class="permission-description">Allow user to view system analytics</div>
                            </label>
                        </div>
                        <div class="permission-item">
                            <input type="checkbox" id="manage_system" name="permissions[]" value="manage_system">
                            <label for="manage_system">
                                Manage System
                                <div class="permission-description">Allow user to manage system settings</div>
                            </label>
                        </div>
                        <div class="permission-item">
                            <input type="checkbox" id="export_data" name="permissions[]" value="export_data">
                            <label for="export_data">
                                Export Data
                                <div class="permission-description">Allow user to export system data</div>
                            </label>
                        </div>
                        <div class="permission-item">
                            <input type="checkbox" id="import_data" name="permissions[]" value="import_data">
                            <label for="import_data">
                                Import Data
                                <div class="permission-description">Allow user to import system data</div>
                            </label>
                        </div>
                    </div>
                </div>

                <div style="text-align: center; margin-top: 30px;">
                    <button type="submit" class="btn btn-primary">Save Permissions</button>
                    <a href="/admin/users" class="btn btn-secondary">Cancel</a>
                </div>
            </form>
        </div>
    </div>
</body>
</html>"#,
        user.display_name,
        user.email,
        user.id.to_string()
    );

    Ok(Html(html))
}
