use crate::application::state::AppState;
use crate::domain::entities::user::User;
use crate::domain::value_objects::{Email, Password, Points, UserId};
use crate::shared::errors::Result;
use axum::{extract::State, response::Html, Form};
use chrono::Utc;

#[derive(serde::Deserialize)]
pub struct CreateUserForm {
    pub email: String,
    pub password: String,
    pub display_name: String,
}

#[derive(serde::Deserialize)]
pub struct UpdateUserForm {
    pub email: String,
    pub display_name: String,
    pub total_xp: u32,
    pub current_streak_days: u32,
    pub highest_streak_days: u32,
}

pub async fn users_handler(State(state): State<AppState>) -> Result<Html<String>> {
    // Fetch real users from database
    let users = state.user_repository.list(100, 0).await?;

    let mut users_html = String::new();
    for user in &users {
        users_html.push_str(&format!(
            r#"<tr>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td>
                    <a href="/admin/users/{}/edit" class="btn btn-sm btn-primary">Edit</a>
                    <a href="/admin/users/{}/delete" class="btn btn-sm btn-danger" onclick="return confirm('Are you sure?')">Delete</a>
                </td>
            </tr>"#,
            user.display_name,
            user.email.clone().into_string(),
            user.total_xp.value(),
            user.current_streak_days,
            user.highest_streak_days,
            user.last_active_date,
            user.created_at.format("%Y-%m-%d %H:%M"),
            user.id.to_string(),
            user.id.to_string()
        ));
    }

    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Users Management</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; }}
        .header {{ display: flex; justify-content: space-between; align-items: center; margin-bottom: 30px; }}
        .btn {{ padding: 8px 16px; text-decoration: none; border-radius: 4px; font-size: 14px; }}
        .btn-primary {{ background-color: #007bff; color: white; }}
        .btn-danger {{ background-color: #dc3545; color: white; }}
        .btn-success {{ background-color: #28a745; color: white; }}
        .btn-sm {{ padding: 6px 12px; font-size: 12px; }}
        table {{ width: 100%; border-collapse: collapse; margin-top: 20px; }}
        th, td {{ border: 1px solid #ddd; padding: 12px; text-align: left; }}
        th {{ background-color: #f8f9fa; font-weight: bold; }}
        tr:nth-child(even) {{ background-color: #f9f9f9; }}
        .stats {{ display: flex; gap: 20px; margin-bottom: 20px; }}
        .stat-card {{ background-color: #f8f9fa; padding: 20px; border-radius: 8px; border: 1px solid #dee2e6; }}
        .stat-number {{ font-size: 24px; font-weight: bold; color: #007bff; }}
        .stat-label {{ color: #6c757d; }}
    </style>
</head>
<body>
    <div class="header">
        <h1>👥 Users Management</h1>
        <a href="/admin/users/new" class="btn btn-success">+ Add New User</a>
    </div>

    <div class="stats">
        <div class="stat-card">
            <div class="stat-number">{}</div>
            <div class="stat-label">Total Users</div>
        </div>
    </div>

    <table>
        <thead>
            <tr>
                <th>Display Name</th>
                <th>Email</th>
                <th>Total XP</th>
                <th>Current Streak</th>
                <th>Highest Streak</th>
                <th>Last Active</th>
                <th>Created At</th>
                <th>Actions</th>
            </tr>
        </thead>
        <tbody>
            {}
        </tbody>
    </table>

    <div style="margin-top: 30px;">
        <a href="/admin" class="btn btn-primary">← Back to Dashboard</a>
    </div>
</body>
</html>"#,
        users.len(),
        users_html
    );

    Ok(Html(html))
}

pub async fn create_user_handler(State(_state): State<AppState>) -> Result<Html<String>> {
    let html = r#"<!DOCTYPE html>
<html>
<head>
    <title>Create User</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; }
        .form-container { max-width: 600px; margin: 0 auto; }
        .form-group { margin-bottom: 20px; }
        label { display: block; margin-bottom: 5px; font-weight: bold; }
        input { width: 100%; padding: 8px; border: 1px solid #ddd; border-radius: 4px; }
        button { background-color: #28a745; color: white; padding: 10px 20px; border: none; border-radius: 4px; cursor: pointer; }
        button:hover { background-color: #218838; }
        .back-link { color: #007bff; text-decoration: none; }
        .back-link:hover { text-decoration: underline; }
    </style>
</head>
<body>
    <div class="form-container">
        <h1>👥 Create New User</h1>

        <form action="/admin/users" method="POST">
            <div class="form-group">
                <label for="email">Email:</label>
                <input type="email" id="email" name="email" required placeholder="user@example.com">
            </div>

            <div class="form-group">
                <label for="password">Password:</label>
                <input type="password" id="password" name="password" required placeholder="Enter password">
            </div>

            <div class="form-group">
                <label for="display_name">Display Name:</label>
                <input type="text" id="display_name" name="display_name" required placeholder="John Doe">
            </div>

            <button type="submit">Create User</button>
        </form>

        <p><a href="/admin/users" class="back-link">← Back to Users</a></p>
    </div>
</body>
</html>"#.to_string();

    Ok(Html(html))
}

pub async fn create_user_post_handler(
    State(state): State<AppState>,
    Form(form): Form<CreateUserForm>,
) -> Result<Html<String>> {
    // Parse form data
    let email = Email::new(form.email).map_err(|e| anyhow::anyhow!("Invalid email: {}", e))?;
    let password =
        Password::new(&form.password).map_err(|e| anyhow::anyhow!("Invalid password: {}", e))?;

    // Create user
    let user = User::new(email, password, form.display_name);

    // Save to database
    state.user_repository.create(&user).await?;

    // Return success page
    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>User Created</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; text-align: center; }}
        .success {{ color: #28a745; font-size: 24px; margin-bottom: 20px; }}
        .info {{ background-color: #f8f9fa; padding: 20px; border-radius: 5px; margin: 20px 0; }}
        a {{ color: #007bff; text-decoration: none; }}
        a:hover {{ text-decoration: underline; }}
    </style>
</head>
<body>
    <div class="success">✅ User Created Successfully!</div>
    <div class="info">
        <h3>User Details:</h3>
        <p><strong>Display Name:</strong> {}</p>
        <p><strong>Email:</strong> {}</p>
        <p><strong>User ID:</strong> {}</p>
        <p><strong>Total XP:</strong> {}</p>
        <p><strong>Current Streak:</strong> {} days</p>
        <p><strong>Highest Streak:</strong> {} days</p>
    </div>
    <p><a href="/admin/users">← Back to Users</a></p>
    <p><a href="/admin/users/new">Create Another User</a></p>
</body>
</html>"#,
        user.display_name,
        user.email.into_string(),
        user.id.to_string(),
        user.total_xp.value(),
        user.current_streak_days,
        user.highest_streak_days
    );

    Ok(Html(html))
}

pub async fn edit_user_handler(
    State(state): State<AppState>,
    axum::extract::Path(user_id): axum::extract::Path<String>,
) -> Result<Html<String>> {
    // Parse user ID
    let user_id =
        UserId::from_string(user_id).map_err(|e| anyhow::anyhow!("Invalid user ID: {}", e))?;

    // Fetch user
    let user = state
        .user_repository
        .find_by_id(&user_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("User not found"))?;

    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Edit User</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; }}
        .form-container {{ max-width: 600px; margin: 0 auto; }}
        .form-group {{ margin-bottom: 20px; }}
        label {{ display: block; margin-bottom: 5px; font-weight: bold; }}
        input {{ width: 100%; padding: 8px; border: 1px solid #ddd; border-radius: 4px; }}
        button {{ background-color: #007bff; color: white; padding: 10px 20px; border: none; border-radius: 4px; cursor: pointer; }}
        button:hover {{ background-color: #0056b3; }}
        .btn-danger {{ background-color: #dc3545; }}
        .btn-danger:hover {{ background-color: #c82333; }}
        .back-link {{ color: #007bff; text-decoration: none; }}
        .back-link:hover {{ text-decoration: underline; }}
        .delete-section {{ margin-top: 30px; padding-top: 20px; border-top: 1px solid #ddd; }}
    </style>
</head>
<body>
    <div class="form-container">
        <h1>👥 Edit User</h1>

        <form action="/admin/users/{}/update" method="POST">
            <div class="form-group">
                <label for="email">Email:</label>
                <input type="email" id="email" name="email" value="{}" required>
            </div>

            <div class="form-group">
                <label for="display_name">Display Name:</label>
                <input type="text" id="display_name" name="display_name" value="{}" required>
            </div>

            <div class="form-group">
                <label for="total_xp">Total XP:</label>
                <input type="number" id="total_xp" name="total_xp" min="0" max="100000" value="{}" required>
            </div>

            <div class="form-group">
                <label for="current_streak_days">Current Streak (days):</label>
                <input type="number" id="current_streak_days" name="current_streak_days" min="0" max="365" value="{}" required>
            </div>

            <div class="form-group">
                <label for="highest_streak_days">Highest Streak (days):</label>
                <input type="number" id="highest_streak_days" name="highest_streak_days" min="0" max="365" value="{}" required>
            </div>

            <button type="submit">Update User</button>
        </form>

        <div class="delete-section">
            <h3>⚠️ Danger Zone</h3>
            <p>Once you delete a user, there is no going back. Please be certain.</p>
            <a href="/admin/users/{}/delete" class="btn btn-danger" onclick="return confirm('Are you sure you want to delete this user? This action cannot be undone.')">Delete User</a>
        </div>

        <p><a href="/admin/users" class="back-link">← Back to Users</a></p>
    </div>
</body>
</html>"#,
        user.id.to_string(),
        user.email.into_string(),
        user.display_name,
        user.total_xp.value(),
        user.current_streak_days,
        user.highest_streak_days,
        user.id.to_string()
    );

    Ok(Html(html))
}

pub async fn update_user_post_handler(
    State(state): State<AppState>,
    axum::extract::Path(user_id): axum::extract::Path<String>,
    Form(form): Form<UpdateUserForm>,
) -> Result<Html<String>> {
    // Parse user ID
    let user_id =
        UserId::from_string(user_id).map_err(|e| anyhow::anyhow!("Invalid user ID: {}", e))?;

    // Parse form data
    let email = Email::new(form.email).map_err(|e| anyhow::anyhow!("Invalid email: {}", e))?;
    let total_xp = Points::new(form.total_xp);

    // Fetch existing user
    let mut user = state
        .user_repository
        .find_by_id(&user_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("User not found"))?;

    // Update fields
    user.email = email;
    user.display_name = form.display_name;
    user.total_xp = total_xp;
    user.current_streak_days = form.current_streak_days;
    user.highest_streak_days = form.highest_streak_days;
    user.updated_at = Utc::now();

    // Save to database
    state.user_repository.update(&user).await?;

    // Return success page
    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>User Updated</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; text-align: center; }}
        .success {{ color: #28a745; font-size: 24px; margin-bottom: 20px; }}
        .info {{ background-color: #f8f9fa; padding: 20px; border-radius: 5px; margin: 20px 0; }}
        a {{ color: #007bff; text-decoration: none; }}
        a:hover {{ text-decoration: underline; }}
    </style>
</head>
<body>
    <div class="success">✅ User Updated Successfully!</div>
    <div class="info">
        <h3>Updated User Details:</h3>
        <p><strong>Display Name:</strong> {}</p>
        <p><strong>Email:</strong> {}</p>
        <p><strong>User ID:</strong> {}</p>
        <p><strong>Total XP:</strong> {}</p>
        <p><strong>Current Streak:</strong> {} days</p>
        <p><strong>Highest Streak:</strong> {} days</p>
    </div>
    <p><a href="/admin/users">← Back to Users</a></p>
    <p><a href="/admin/users/{}/edit">Edit Again</a></p>
</body>
</html>"#,
        user.display_name,
        user.email.into_string(),
        user.id.to_string(),
        user.total_xp.value(),
        user.current_streak_days,
        user.highest_streak_days,
        user.id.to_string()
    );

    Ok(Html(html))
}

pub async fn delete_user_handler(
    State(state): State<AppState>,
    axum::extract::Path(user_id): axum::extract::Path<String>,
) -> Result<Html<String>> {
    // Parse user ID
    let user_id =
        UserId::from_string(user_id).map_err(|e| anyhow::anyhow!("Invalid user ID: {}", e))?;

    // Delete from database
    state.user_repository.delete(&user_id).await?;

    // Return success page
    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>User Deleted</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; text-align: center; }}
        .success {{ color: #28a745; font-size: 24px; margin-bottom: 20px; }}
        .info {{ background-color: #f8f9fa; padding: 20px; border-radius: 5px; margin: 20px 0; }}
        a {{ color: #007bff; text-decoration: none; }}
        a:hover {{ text-decoration: underline; }}
    </style>
</head>
<body>
    <div class="success">✅ User Deleted Successfully!</div>
    <div class="info">
        <h3>Deleted User:</h3>
        <p><strong>User ID:</strong> {}</p>
        <p>The user has been permanently removed from the database.</p>
    </div>
    <p><a href="/admin/users">← Back to Users</a></p>
    <p><a href="/admin/users/new">Create New User</a></p>
</body>
</html>"#,
        user_id.to_string()
    );

    Ok(Html(html))
}
