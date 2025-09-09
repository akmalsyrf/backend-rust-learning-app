use crate::application::state::AppState;
use crate::shared::errors::Result;
use axum::{
    extract::{Path, State},
    response::Html,
};

pub async fn user_profile_handler(
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

    let progress = state
        .user_progress_repository
        .find_by_user_id(&user_id)
        .await
        .map_err(|e| crate::shared::errors::AppError::Internal(e.to_string()))?
        .unwrap_or_else(|| crate::domain::entities::UserProgress::new(user_id.clone()));

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>User Profile - Rust Learning Platform</title>
    <style>
        body {{
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            margin: 0;
            padding: 20px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
        }}
        .container {{
            max-width: 1200px;
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
        .profile-grid {{
            display: grid;
            grid-template-columns: 1fr 2fr;
            gap: 40px;
            margin-bottom: 40px;
        }}
        .profile-card {{
            background: #f8f9fa;
            border-radius: 10px;
            padding: 30px;
            text-align: center;
        }}
        .profile-avatar {{
            width: 120px;
            height: 120px;
            border-radius: 50%;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            display: flex;
            align-items: center;
            justify-content: center;
            margin: 0 auto 20px;
            font-size: 3em;
            color: white;
        }}
        .profile-name {{
            font-size: 1.5em;
            font-weight: 600;
            color: #333;
            margin-bottom: 10px;
        }}
        .profile-email {{
            color: #666;
            margin-bottom: 20px;
        }}
        .stats-grid {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 20px;
            margin-bottom: 40px;
        }}
        .stat-card {{
            background: white;
            border: 1px solid #eee;
            border-radius: 10px;
            padding: 20px;
            text-align: center;
            transition: all 0.3s ease;
        }}
        .stat-card:hover {{
            transform: translateY(-5px);
            box-shadow: 0 10px 20px rgba(0,0,0,0.1);
        }}
        .stat-icon {{
            font-size: 2.5em;
            margin-bottom: 10px;
        }}
        .stat-value {{
            font-size: 2em;
            font-weight: 600;
            color: #333;
            margin-bottom: 5px;
        }}
        .stat-label {{
            color: #666;
            font-size: 0.9em;
        }}
        .progress-section {{
            background: #f8f9fa;
            border-radius: 10px;
            padding: 30px;
            margin-bottom: 40px;
        }}
        .progress-section h3 {{
            color: #333;
            margin-bottom: 20px;
            font-size: 1.3em;
        }}
        .progress-bar {{
            background: #e9ecef;
            border-radius: 10px;
            height: 20px;
            margin-bottom: 10px;
            overflow: hidden;
        }}
        .progress-fill {{
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            height: 100%;
            border-radius: 10px;
            transition: width 0.3s ease;
        }}
        .progress-text {{
            display: flex;
            justify-content: space-between;
            color: #666;
            font-size: 0.9em;
        }}
        .achievements {{
            background: white;
            border: 1px solid #eee;
            border-radius: 10px;
            padding: 30px;
        }}
        .achievements h3 {{
            color: #333;
            margin-bottom: 20px;
            font-size: 1.3em;
        }}
        .achievement-list {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
            gap: 15px;
        }}
        .achievement-item {{
            display: flex;
            align-items: center;
            padding: 15px;
            background: #f8f9fa;
            border-radius: 8px;
            border-left: 4px solid #667eea;
        }}
        .achievement-icon {{
            font-size: 2em;
            margin-right: 15px;
        }}
        .achievement-info h4 {{
            margin: 0 0 5px 0;
            color: #333;
            font-size: 1em;
        }}
        .achievement-info p {{
            margin: 0;
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
        .btn {{
            display: inline-block;
            padding: 12px 24px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            text-decoration: none;
            border-radius: 25px;
            margin: 10px 5px;
            transition: all 0.3s ease;
        }}
        .btn:hover {{
            transform: translateY(-2px);
            box-shadow: 0 10px 20px rgba(102, 126, 234, 0.3);
        }}
        .btn-secondary {{
            background: #6c757d;
        }}
        .btn-danger {{
            background: #dc3545;
        }}
        @media (max-width: 768px) {{
            .profile-grid {{
                grid-template-columns: 1fr;
            }}
            .stats-grid {{
                grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
            }}
        }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>👤 User Profile</h1>
            <p>Detailed user information and progress tracking</p>
        </div>

        <div class="content">
            <a href="/admin/users" class="back-btn">← Back to Users</a>

            <div class="profile-grid">
                <div class="profile-card">
                    <div class="profile-avatar">👤</div>
                    <div class="profile-name">{}</div>
                    <div class="profile-email">{}</div>
                    <div style="margin-top: 20px;">
                        <a href="/admin/users/{}/edit" class="btn">Edit Profile</a>
                        <a href="/admin/users/{}/delete" class="btn btn-danger" onclick="return confirm('Are you sure?')">Delete User</a>
                    </div>
                </div>

                <div class="stats-grid">
                    <div class="stat-card">
                        <div class="stat-icon">🏆</div>
                        <div class="stat-value">{}</div>
                        <div class="stat-label">Total XP</div>
                    </div>
                    <div class="stat-card">
                        <div class="stat-icon">🔥</div>
                        <div class="stat-value">{}</div>
                        <div class="stat-label">Current Streak</div>
                    </div>
                    <div class="stat-card">
                        <div class="stat-icon">⭐</div>
                        <div class="stat-value">{}</div>
                        <div class="stat-label">Highest Streak</div>
                    </div>
                    <div class="stat-card">
                        <div class="stat-icon">❓</div>
                        <div class="stat-value">{}</div>
                        <div class="stat-label">Questions Answered</div>
                    </div>
                    <div class="stat-card">
                        <div class="stat-icon">💻</div>
                        <div class="stat-value">{}</div>
                        <div class="stat-label">Code Practices</div>
                    </div>
                    <div class="stat-card">
                        <div class="stat-icon">📅</div>
                        <div class="stat-value">{}</div>
                        <div class="stat-label">Last Active</div>
                    </div>
                </div>
            </div>

            <div class="progress-section">
                <h3>📊 Learning Progress</h3>
                <div class="progress-bar">
                    <div class="progress-fill" style="width: {}%"></div>
                </div>
                <div class="progress-text">
                    <span>Progress: {}%</span>
                    <span>Level: {}</span>
                </div>
            </div>

            <div class="achievements">
                <h3>🏅 Achievements</h3>
                <div class="achievement-list">
                    <div class="achievement-item">
                        <div class="achievement-icon">🎯</div>
                        <div class="achievement-info">
                            <h4>First Steps</h4>
                            <p>Complete your first lesson</p>
                        </div>
                    </div>
                    <div class="achievement-item">
                        <div class="achievement-icon">🔥</div>
                        <div class="achievement-info">
                            <h4>Streak Master</h4>
                            <p>Maintain a 7-day streak</p>
                        </div>
                    </div>
                    <div class="achievement-item">
                        <div class="achievement-icon">💯</div>
                        <div class="achievement-info">
                            <h4>Perfect Score</h4>
                            <p>Get 100% on a quiz</p>
                        </div>
                    </div>
                    <div class="achievement-item">
                        <div class="achievement-icon">🚀</div>
                        <div class="achievement-info">
                            <h4>Speed Learner</h4>
                            <p>Complete 10 lessons in one day</p>
                        </div>
                    </div>
                    <div class="achievement-item">
                        <div class="achievement-icon">🎓</div>
                        <div class="achievement-info">
                            <h4>Graduate</h4>
                            <p>Complete all beginner topics</p>
                        </div>
                    </div>
                    <div class="achievement-item">
                        <div class="achievement-icon">👑</div>
                        <div class="achievement-info">
                            <h4>Champion</h4>
                            <p>Reach 1000 XP</p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</body>
</html>"#,
        user.display_name,
        user.email,
        user.id.to_string(),
        user.id.to_string(),
        progress.total_xp.value(),
        progress.current_streak_days,
        progress.highest_streak_days,
        progress.completed_questions.len(),
        progress.completed_code_practices.len(),
        progress.last_active_date.format("%Y-%m-%d"),
        (progress.total_xp.value() as f64 / 1000.0 * 100.0).min(100.0) as u32,
        (progress.total_xp.value() as f64 / 1000.0 * 100.0).min(100.0) as u32,
        match progress.total_xp.value() {
            0..=99 => "Beginner",
            100..=499 => "Intermediate",
            500..=999 => "Advanced",
            _ => "Expert",
        }
    );

    Ok(Html(html))
}

pub async fn edit_user_profile_handler(
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
    <title>Edit User Profile - Rust Learning Platform</title>
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
        .form-group {{
            margin-bottom: 25px;
        }}
        .form-group label {{
            display: block;
            margin-bottom: 8px;
            font-weight: 600;
            color: #333;
        }}
        .form-group input {{
            width: 100%;
            padding: 12px;
            border: 2px solid #e9ecef;
            border-radius: 8px;
            font-size: 1em;
            transition: border-color 0.3s ease;
        }}
        .form-group input:focus {{
            outline: none;
            border-color: #667eea;
        }}
        .form-actions {{
            display: flex;
            gap: 15px;
            margin-top: 30px;
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
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>✏️ Edit User Profile</h1>
        </div>

        <div class="content">
            <a href="/admin/users/{}/profile" class="back-btn">← Back to Profile</a>

            <form method="POST" action="/admin/users/{}/update">
                <div class="form-group">
                    <label for="display_name">Display Name:</label>
                    <input type="text" id="display_name" name="display_name" value="{}" required>
                </div>

                <div class="form-group">
                    <label for="email">Email:</label>
                    <input type="email" id="email" name="email" value="{}" required>
                </div>

                <div class="form-group">
                    <label for="password">New Password (leave blank to keep current):</label>
                    <input type="password" id="password" name="password">
                </div>

                <div class="form-actions">
                    <button type="submit" class="btn btn-primary">Update Profile</button>
                    <a href="/admin/users/{}/profile" class="btn btn-secondary">Cancel</a>
                </div>
            </form>
        </div>
    </div>
</body>
</html>"#,
        user.id.to_string(),
        user.id.to_string(),
        user.display_name,
        user.email,
        user.id.to_string()
    );

    Ok(Html(html))
}
