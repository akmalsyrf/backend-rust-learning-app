use crate::application::state::AppState;
use crate::domain::entities::user_progress::{CompletedCodePractice, QuestionResult, UserProgress};
use crate::domain::value_objects::{CodePracticeId, Points, QuestionId, UserId};
use crate::shared::errors::Result;
use axum::{extract::State, response::Html, Form};
use chrono::Utc;

#[derive(serde::Deserialize)]
pub struct SubmitQuestionForm {
    pub question_id: String,
    pub user_answer: String,
    pub time_spent_ms: u64,
}

#[derive(serde::Deserialize)]
pub struct SubmitCodePracticeForm {
    pub code_practice_id: String,
    pub user_code: String,
}

pub async fn progress_dashboard_handler(
    State(state): State<AppState>,
    axum::extract::Path(user_id): axum::extract::Path<String>,
) -> Result<Html<String>> {
    // Parse user_id
    let user_id =
        UserId::from_string(user_id).map_err(|e| anyhow::anyhow!("Invalid user ID: {}", e))?;

    // Fetch user progress
    let user_progress = state
        .user_progress_repository
        .find_by_user_id(&user_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("User progress not found"))?;

    // Fetch user details
    let user = state
        .user_repository
        .find_by_id(&user_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("User not found"))?;

    // Get leaderboard position
    let leaderboard = state
        .leaderboard_repository
        .get_all_time_leaderboard(100)
        .await?;
    let position = leaderboard
        .iter()
        .position(|entry| entry.user_id == user_id)
        .map(|p| p + 1)
        .unwrap_or(0);

    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Progress Dashboard - {}</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; }}
        .dashboard {{ display: grid; grid-template-columns: 1fr 1fr; gap: 20px; }}
        .card {{ background-color: #f8f9fa; padding: 20px; border-radius: 8px; border: 1px solid #dee2e6; }}
        .card h3 {{ margin-top: 0; color: #495057; }}
        .stat {{ display: flex; justify-content: space-between; margin: 10px 0; }}
        .stat-label {{ font-weight: bold; }}
        .stat-value {{ color: #007bff; }}
        .progress-bar {{ width: 100%; height: 20px; background-color: #e9ecef; border-radius: 10px; overflow: hidden; }}
        .progress-fill {{ height: 100%; background-color: #28a745; transition: width 0.3s ease; }}
        .achievement {{ background-color: #fff3cd; border: 1px solid #ffeaa7; padding: 10px; margin: 10px 0; border-radius: 5px; }}
        .achievement.unlocked {{ background-color: #d4edda; border-color: #c3e6cb; }}
        a {{ color: #007bff; text-decoration: none; }}
        a:hover {{ text-decoration: underline; }}
    </style>
</head>
<body>
    <h1>Progress Dashboard</h1>
    <p><strong>User:</strong> {} ({})</p>

    <div class="dashboard">
        <div class="card">
            <h3>📊 Overall Statistics</h3>
            <div class="stat">
                <span class="stat-label">Total XP:</span>
                <span class="stat-value">{}</span>
            </div>
            <div class="stat">
                <span class="stat-label">Current Streak:</span>
                <span class="stat-value">{} days</span>
            </div>
            <div class="stat">
                <span class="stat-label">Highest Streak:</span>
                <span class="stat-value">{} days</span>
            </div>
            <div class="stat">
                <span class="stat-label">Leaderboard Position:</span>
                <span class="stat-value">#{}</span>
            </div>
            <div class="stat">
                <span class="stat-label">Last Active:</span>
                <span class="stat-value">{}</span>
            </div>
        </div>

        <div class="card">
            <h3>🎯 Learning Progress</h3>
            <div class="stat">
                <span class="stat-label">Questions Completed:</span>
                <span class="stat-value">{}</span>
            </div>
            <div class="stat">
                <span class="stat-label">Code Practices Completed:</span>
                <span class="stat-value">{}</span>
            </div>
            <div class="stat">
                <span class="stat-label">Lessons with Stars:</span>
                <span class="stat-value">{}</span>
            </div>

            <h4>Weekly Progress</h4>
            <div class="stat">
                <span class="stat-label">Daily XP Cap:</span>
                <span class="stat-value">{}</span>
            </div>
            <div class="progress-bar">
                <div class="progress-fill" style="width: {}%"></div>
            </div>
        </div>
    </div>

    <div class="card">
        <h3>🏆 Recent Achievements</h3>
        <div class="achievement unlocked">
            <strong>First Question!</strong> - Completed your first question
        </div>
        <div class="achievement unlocked">
            <strong>Streak Master!</strong> - Maintained a 7-day streak
        </div>
        <div class="achievement">
            <strong>XP Collector!</strong> - Reach 1000 total XP
        </div>
        <div class="achievement">
            <strong>Code Warrior!</strong> - Complete 50 code practices
        </div>
    </div>

    <p><a href="/admin/users">← Back to Users</a></p>
    <p><a href="/admin/progress/{}/submit-question">Submit Question Result</a></p>
    <p><a href="/admin/progress/{}/submit-code-practice">Submit Code Practice</a></p>
</body>
</html>"#,
        user.display_name,
        user.display_name,
        user.email.clone().into_string(),
        user_progress.total_xp.value(),
        user_progress.current_streak_days,
        user_progress.highest_streak_days,
        position,
        user_progress.last_active_date.format("%Y-%m-%d %H:%M"),
        user_progress.completed_questions.len(),
        user_progress.completed_code_practices.len(),
        user_progress.lesson_stars.len(),
        user_progress.daily_xp_cap,
        (user_progress.total_xp.value() as f64 / 1000.0 * 100.0).min(100.0) as u32,
        user_id.to_string(),
        user_id.to_string()
    );

    Ok(Html(html))
}

pub async fn submit_question_form_handler(
    State(state): State<AppState>,
    axum::extract::Path(user_id): axum::extract::Path<String>,
) -> Result<Html<String>> {
    // Parse user_id
    let user_id =
        UserId::from_string(user_id).map_err(|e| anyhow::anyhow!("Invalid user ID: {}", e))?;

    // Fetch available questions
    let questions = state.question_repository.list(100, 0).await?;

    let mut question_options = String::new();
    for question in &questions {
        question_options.push_str(&format!(
            "<option value=\"{}\">{} - {}</option>",
            question.id.to_string(),
            question.prompt.get("en"),
            match question.difficulty {
                crate::domain::value_objects::Difficulty::Beginner => "Easy",
                crate::domain::value_objects::Difficulty::Intermediate => "Medium",
                crate::domain::value_objects::Difficulty::Advanced => "Hard",
            }
        ));
    }

    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Submit Question Result</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; }}
        .form-group {{ margin-bottom: 20px; }}
        label {{ display: block; margin-bottom: 5px; font-weight: bold; }}
        input, textarea, select {{ width: 100%; padding: 8px; border: 1px solid #ddd; border-radius: 4px; }}
        button {{ background-color: #28a745; color: white; padding: 10px 20px; border: none; border-radius: 4px; cursor: pointer; }}
        button:hover {{ background-color: #218838; }}
    </style>
</head>
<body>
    <h1>Submit Question Result</h1>
    <p><strong>User ID:</strong> {}</p>

    <form action="/admin/progress/{}/submit-question" method="POST">
        <div class="form-group">
            <label for="question_id">Question:</label>
            <select id="question_id" name="question_id" required>
                <option value="">Select a question...</option>
                {}
            </select>
        </div>
        <div class="form-group">
            <label for="user_answer">User Answer:</label>
            <textarea id="user_answer" name="user_answer" rows="3" required placeholder="Enter the user's answer..."></textarea>
        </div>
        <div class="form-group">
            <label for="time_spent_ms">Time Spent (milliseconds):</label>
            <input type="number" id="time_spent_ms" name="time_spent_ms" min="0" value="0" required>
        </div>
        <button type="submit">Submit Result</button>
    </form>

    <p><a href="/admin/progress/{}">← Back to Progress Dashboard</a></p>
</body>
</html>"#,
        user_id.to_string(),
        user_id.to_string(),
        question_options,
        user_id.to_string()
    );

    Ok(Html(html))
}

pub async fn submit_question_post_handler(
    State(state): State<AppState>,
    axum::extract::Path(user_id): axum::extract::Path<String>,
    Form(form): Form<SubmitQuestionForm>,
) -> Result<Html<String>> {
    // Parse user_id
    let user_id =
        UserId::from_string(user_id).map_err(|e| anyhow::anyhow!("Invalid user ID: {}", e))?;

    // Parse question_id
    let question_id = QuestionId::from_string(&form.question_id)
        .map_err(|e| anyhow::anyhow!("Invalid question ID: {}", e))?;

    // Fetch question to get points
    let question = state
        .question_repository
        .find_by_id(&question_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Question not found"))?;

    // Get or create user progress
    let mut user_progress = state
        .user_progress_repository
        .find_by_user_id(&user_id)
        .await?
        .unwrap_or_else(|| UserProgress::new(user_id.clone()));

    // Create question result
    let question_result = QuestionResult {
        question_id: question_id.clone(),
        correct: true, // Assume correct for now
        user_answer: form.user_answer.clone(),
        time_spent_ms: form.time_spent_ms,
        points: question.points,
        completed_at: Utc::now(),
    };

    // Add question result
    user_progress.add_question_result(question_result);

    // Save updated progress
    if user_progress.completed_questions.len() == 1
        && user_progress.completed_code_practices.is_empty()
    {
        state
            .user_progress_repository
            .create(&user_progress)
            .await?;
    } else {
        state
            .user_progress_repository
            .update(&user_progress)
            .await?;
    }

    // Update user total XP
    let mut user = state
        .user_repository
        .find_by_id(&user_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("User not found"))?;

    user.total_xp = Points::new(user.total_xp.value() + question.points.value());
    user.current_streak_days = user_progress.current_streak_days;
    user.highest_streak_days = user_progress.highest_streak_days;
    user.last_active_date = user_progress.last_active_date;

    state.user_repository.update(&user).await?;

    // Return success page
    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Question Result Submitted</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; text-align: center; }}
        .success {{ color: #28a745; font-size: 24px; margin-bottom: 20px; }}
        .info {{ background-color: #f8f9fa; padding: 20px; border-radius: 5px; margin: 20px 0; }}
        a {{ color: #007bff; text-decoration: none; }}
        a:hover {{ text-decoration: underline; }}
    </style>
</head>
<body>
    <div class="success">✅ Question Result Submitted Successfully!</div>
    <div class="info">
        <h3>Result Details:</h3>
        <p><strong>Question:</strong> {}</p>
        <p><strong>User Answer:</strong> {}</p>
        <p><strong>Time Spent:</strong> {} ms</p>
        <p><strong>Points Earned:</strong> {}</p>
        <p><strong>New Total XP:</strong> {}</p>
    </div>
    <p><a href="/admin/progress/{}">← Back to Progress Dashboard</a></p>
    <p><a href="/admin/progress/{}/submit-question">Submit Another Question</a></p>
</body>
</html>"#,
        question.prompt.get("en"),
        form.user_answer,
        form.time_spent_ms,
        question.points.value(),
        user.total_xp.value(),
        user_id.to_string(),
        user_id.to_string()
    );

    Ok(Html(html))
}

pub async fn submit_code_practice_form_handler(
    State(state): State<AppState>,
    axum::extract::Path(user_id): axum::extract::Path<String>,
) -> Result<Html<String>> {
    // Parse user_id
    let user_id =
        UserId::from_string(user_id).map_err(|e| anyhow::anyhow!("Invalid user ID: {}", e))?;

    // Fetch available code practices
    let code_practices = state.code_practice_repository.list(100, 0).await?;

    let mut practice_options = String::new();
    for practice in &code_practices {
        practice_options.push_str(&format!(
            "<option value=\"{}\">{} - {}</option>",
            practice.id.to_string(),
            practice.title.get("en"),
            match practice.difficulty {
                crate::domain::value_objects::Difficulty::Beginner => "Easy",
                crate::domain::value_objects::Difficulty::Intermediate => "Medium",
                crate::domain::value_objects::Difficulty::Advanced => "Hard",
            }
        ));
    }

    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Submit Code Practice</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; }}
        .form-group {{ margin-bottom: 20px; }}
        label {{ display: block; margin-bottom: 5px; font-weight: bold; }}
        input, textarea, select {{ width: 100%; padding: 8px; border: 1px solid #ddd; border-radius: 4px; }}
        button {{ background-color: #28a745; color: white; padding: 10px 20px; border: none; border-radius: 4px; cursor: pointer; }}
        button:hover {{ background-color: #218838; }}
    </style>
</head>
<body>
    <h1>Submit Code Practice</h1>
    <p><strong>User ID:</strong> {}</p>

    <form action="/admin/progress/{}/submit-code-practice" method="POST">
        <div class="form-group">
            <label for="code_practice_id">Code Practice:</label>
            <select id="code_practice_id" name="code_practice_id" required>
                <option value="">Select a code practice...</option>
                {}
            </select>
        </div>
        <div class="form-group">
            <label for="user_code">User Code:</label>
            <textarea id="user_code" name="user_code" rows="10" required placeholder="Enter the user's code solution..."></textarea>
        </div>
        <button type="submit">Submit Code Practice</button>
    </form>

    <p><a href="/admin/progress/{}">← Back to Progress Dashboard</a></p>
</body>
</html>"#,
        user_id.to_string(),
        user_id.to_string(),
        practice_options,
        user_id.to_string()
    );

    Ok(Html(html))
}

pub async fn submit_code_practice_post_handler(
    State(state): State<AppState>,
    axum::extract::Path(user_id): axum::extract::Path<String>,
    Form(form): Form<SubmitCodePracticeForm>,
) -> Result<Html<String>> {
    // Parse user_id
    let user_id =
        UserId::from_string(user_id).map_err(|e| anyhow::anyhow!("Invalid user ID: {}", e))?;

    // Parse code_practice_id
    let code_practice_id = CodePracticeId::from_string(&form.code_practice_id)
        .map_err(|e| anyhow::anyhow!("Invalid code practice ID: {}", e))?;

    // Fetch code practice to get points
    let code_practice = state
        .code_practice_repository
        .find_by_id(&code_practice_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Code practice not found"))?;

    // Get or create user progress
    let mut user_progress = state
        .user_progress_repository
        .find_by_user_id(&user_id)
        .await?
        .unwrap_or_else(|| UserProgress::new(user_id.clone()));

    // Create code practice completion
    let code_practice_completion = CompletedCodePractice {
        id: code_practice_id.clone(),
        completed_at: Utc::now(),
        user_code: form.user_code.clone(),
        is_correct: true, // Assume correct for now
        xp_earned: code_practice.points,
    };

    // Add code practice completion
    user_progress.add_code_practice_completion(code_practice_completion);

    // Save updated progress
    if user_progress.completed_questions.is_empty()
        && user_progress.completed_code_practices.len() == 1
    {
        state
            .user_progress_repository
            .create(&user_progress)
            .await?;
    } else {
        state
            .user_progress_repository
            .update(&user_progress)
            .await?;
    }

    // Update user total XP
    let mut user = state
        .user_repository
        .find_by_id(&user_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("User not found"))?;

    user.total_xp = Points::new(user.total_xp.value() + code_practice.points.value());
    user.current_streak_days = user_progress.current_streak_days;
    user.highest_streak_days = user_progress.highest_streak_days;
    user.last_active_date = user_progress.last_active_date;

    state.user_repository.update(&user).await?;

    // Return success page
    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Code Practice Submitted</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; text-align: center; }}
        .success {{ color: #28a745; font-size: 24px; margin-bottom: 20px; }}
        .info {{ background-color: #f8f9fa; padding: 20px; border-radius: 5px; margin: 20px 0; }}
        .code {{ background-color: #f8f9fa; padding: 10px; border-radius: 4px; font-family: monospace; white-space: pre-wrap; }}
        a {{ color: #007bff; text-decoration: none; }}
        a:hover {{ text-decoration: underline; }}
    </style>
</head>
<body>
    <div class="success">✅ Code Practice Submitted Successfully!</div>
    <div class="info">
        <h3>Submission Details:</h3>
        <p><strong>Code Practice:</strong> {}</p>
        <p><strong>Points Earned:</strong> {}</p>
        <p><strong>New Total XP:</strong> {}</p>
        <p><strong>User Code:</strong></p>
        <div class="code">{}</div>
    </div>
    <p><a href="/admin/progress/{}">← Back to Progress Dashboard</a></p>
    <p><a href="/admin/progress/{}/submit-code-practice">Submit Another Code Practice</a></p>
</body>
</html>"#,
        code_practice.title.get("en"),
        code_practice.points.value(),
        user.total_xp.value(),
        form.user_code,
        user_id.to_string(),
        user_id.to_string()
    );

    Ok(Html(html))
}
