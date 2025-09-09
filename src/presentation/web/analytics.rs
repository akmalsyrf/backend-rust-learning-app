use crate::application::state::AppState;
use crate::shared::errors::Result;
use axum::{extract::State, response::Html};
use chrono::{Duration, Utc};

#[derive(Debug, Clone)]
pub struct AnalyticsData {
    pub total_users: u32,
    pub total_topics: u32,
    pub total_lessons: u32,
    pub total_questions: u32,
    pub total_code_practices: u32,
    pub active_users_today: u32,
    pub active_users_this_week: u32,
    pub active_users_this_month: u32,
    pub questions_answered_today: u32,
    pub code_practices_completed_today: u32,
    pub average_xp_per_user: f64,
    pub top_users: Vec<TopUser>,
    pub difficulty_distribution: DifficultyDistribution,
    pub category_distribution: CategoryDistribution,
    pub daily_activity: Vec<DailyActivity>,
}

#[derive(Debug, Clone)]
pub struct TopUser {
    pub display_name: String,
    pub email: String,
    pub total_xp: u32,
    pub current_streak: u32,
    pub highest_streak: u32,
}

#[derive(Debug, Clone)]
pub struct DifficultyDistribution {
    pub easy: u32,
    pub medium: u32,
    pub hard: u32,
}

#[derive(Debug, Clone)]
pub struct CategoryDistribution {
    pub variables: u32,
    pub functions: u32,
    pub loops: u32,
    pub structs: u32,
    pub ownership: u32,
    pub other: u32,
}

#[derive(Debug, Clone)]
pub struct DailyActivity {
    pub date: String,
    pub users_active: u32,
    pub questions_answered: u32,
    pub code_practices_completed: u32,
}

pub async fn analytics_dashboard_handler(State(state): State<AppState>) -> Result<Html<String>> {
    // Fetch all data for analytics
    let users = state.user_repository.list(1000, 0).await?;
    let topics = state.topic_repository.list(1000, 0).await?;
    let lessons = state.lesson_repository.list(1000, 0).await?;
    let questions = state.question_repository.list(1000, 0).await?;
    let code_practices = state.code_practice_repository.list(1000, 0).await?;

    // Calculate analytics data
    let total_users = users.len() as u32;
    let total_topics = topics.len() as u32;
    let total_lessons = lessons.len() as u32;
    let total_questions = questions.len() as u32;
    let total_code_practices = code_practices.len() as u32;

    // Calculate active users (simplified - using last_active_date)
    let today = Utc::now().date_naive();
    let week_ago = today - Duration::days(7);
    let month_ago = today - Duration::days(30);

    let active_users_today = users
        .iter()
        .filter(|user| user.last_active_date == today)
        .count() as u32;

    let active_users_this_week = users
        .iter()
        .filter(|user| user.last_active_date >= week_ago)
        .count() as u32;

    let active_users_this_month = users
        .iter()
        .filter(|user| user.last_active_date >= month_ago)
        .count() as u32;

    // Calculate average XP
    let total_xp: u32 = users.iter().map(|user| user.total_xp.value()).sum();
    let average_xp_per_user = if total_users > 0 {
        total_xp as f64 / total_users as f64
    } else {
        0.0
    };

    // Get top users (sorted by XP)
    let mut top_users: Vec<TopUser> = users
        .iter()
        .map(|user| TopUser {
            display_name: user.display_name.clone(),
            email: user.email.clone().into_string(),
            total_xp: user.total_xp.value(),
            current_streak: user.current_streak_days,
            highest_streak: user.highest_streak_days,
        })
        .collect();
    top_users.sort_by(|a, b| b.total_xp.cmp(&a.total_xp));
    top_users.truncate(10); // Top 10 users

    // Calculate difficulty distribution
    let difficulty_distribution = DifficultyDistribution {
        easy: questions
            .iter()
            .filter(|q| {
                matches!(
                    q.difficulty,
                    crate::domain::value_objects::Difficulty::Beginner
                )
            })
            .count() as u32,
        medium: questions
            .iter()
            .filter(|q| {
                matches!(
                    q.difficulty,
                    crate::domain::value_objects::Difficulty::Intermediate
                )
            })
            .count() as u32,
        hard: questions
            .iter()
            .filter(|q| {
                matches!(
                    q.difficulty,
                    crate::domain::value_objects::Difficulty::Advanced
                )
            })
            .count() as u32,
    };

    // Calculate category distribution for code practices
    let category_distribution = CategoryDistribution {
        variables: code_practices
            .iter()
            .filter(|cp| cp.category.to_lowercase().contains("variable"))
            .count() as u32,
        functions: code_practices
            .iter()
            .filter(|cp| cp.category.to_lowercase().contains("function"))
            .count() as u32,
        loops: code_practices
            .iter()
            .filter(|cp| cp.category.to_lowercase().contains("loop"))
            .count() as u32,
        structs: code_practices
            .iter()
            .filter(|cp| cp.category.to_lowercase().contains("struct"))
            .count() as u32,
        ownership: code_practices
            .iter()
            .filter(|cp| cp.category.to_lowercase().contains("ownership"))
            .count() as u32,
        other: code_practices
            .iter()
            .filter(|cp| {
                let cat = cp.category.to_lowercase();
                !cat.contains("variable")
                    && !cat.contains("function")
                    && !cat.contains("loop")
                    && !cat.contains("struct")
                    && !cat.contains("ownership")
            })
            .count() as u32,
    };

    // Generate mock daily activity data (in real app, this would come from database)
    let daily_activity = generate_mock_daily_activity();

    let analytics_data = AnalyticsData {
        total_users,
        total_topics,
        total_lessons,
        total_questions,
        total_code_practices,
        active_users_today,
        active_users_this_week,
        active_users_this_month,
        questions_answered_today: 0, // Would need progress tracking data
        code_practices_completed_today: 0, // Would need progress tracking data
        average_xp_per_user,
        top_users,
        difficulty_distribution,
        category_distribution,
        daily_activity,
    };

    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Analytics Dashboard</title>
    <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; background-color: #f8f9fa; }}
        .header {{ text-align: center; margin-bottom: 40px; }}
        .stats-grid {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 20px; margin-bottom: 40px; }}
        .stat-card {{ background-color: white; padding: 20px; border-radius: 8px; border: 1px solid #dee2e6; text-align: center; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }}
        .stat-number {{ font-size: 28px; font-weight: bold; color: #007bff; margin-bottom: 5px; }}
        .stat-label {{ color: #6c757d; font-size: 14px; }}
        .charts-grid {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(400px, 1fr)); gap: 20px; margin-bottom: 40px; }}
        .chart-container {{ background-color: white; padding: 20px; border-radius: 8px; border: 1px solid #dee2e6; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }}
        .chart-title {{ font-weight: bold; margin-bottom: 15px; color: #495057; text-align: center; }}
        .top-users {{ background-color: white; padding: 20px; border-radius: 8px; border: 1px solid #dee2e6; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }}
        .user-item {{ display: flex; justify-content: space-between; align-items: center; padding: 10px 0; border-bottom: 1px solid #f8f9fa; }}
        .user-item:last-child {{ border-bottom: none; }}
        .user-info {{ flex: 1; }}
        .user-name {{ font-weight: bold; color: #495057; }}
        .user-email {{ color: #6c757d; font-size: 12px; }}
        .user-stats {{ text-align: right; }}
        .user-xp {{ font-weight: bold; color: #28a745; }}
        .user-streak {{ color: #6c757d; font-size: 12px; }}
        .btn {{ padding: 8px 16px; text-decoration: none; border-radius: 4px; font-size: 14px; }}
        .btn-primary {{ background-color: #007bff; color: white; }}
        .btn:hover {{ opacity: 0.8; }}
    </style>
</head>
<body>
    <div class="header">
        <h1>📊 Analytics Dashboard</h1>
        <p>Comprehensive insights into the Rust Learning Platform</p>
    </div>

    <div class="stats-grid">
        <div class="stat-card">
            <div class="stat-number">{}</div>
            <div class="stat-label">Total Users</div>
        </div>
        <div class="stat-card">
            <div class="stat-number">{}</div>
            <div class="stat-label">Active Today</div>
        </div>
        <div class="stat-card">
            <div class="stat-number">{}</div>
            <div class="stat-label">Active This Week</div>
        </div>
        <div class="stat-card">
            <div class="stat-number">{}</div>
            <div class="stat-label">Active This Month</div>
        </div>
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
            <div class="stat-label">Code Practices</div>
        </div>
        <div class="stat-card">
            <div class="stat-number">{:.1}</div>
            <div class="stat-label">Avg XP per User</div>
        </div>
    </div>

    <div class="charts-grid">
        <div class="chart-container">
            <div class="chart-title">Question Difficulty Distribution</div>
            <canvas id="difficultyChart" width="400" height="200"></canvas>
        </div>

        <div class="chart-container">
            <div class="chart-title">Code Practice Categories</div>
            <canvas id="categoryChart" width="400" height="200"></canvas>
        </div>

        <div class="chart-container">
            <div class="chart-title">Daily Activity (Last 7 Days)</div>
            <canvas id="activityChart" width="400" height="200"></canvas>
        </div>
    </div>

    <div class="top-users">
        <div class="chart-title">🏆 Top 10 Users by XP</div>
        {}
    </div>

    <div style="margin-top: 30px; text-align: center;">
        <a href="/admin" class="btn btn-primary">← Back to Dashboard</a>
    </div>

    <script>
        // Difficulty Distribution Chart
        const difficultyCtx = document.getElementById('difficultyChart').getContext('2d');
        new Chart(difficultyCtx, {{
            type: 'doughnut',
            data: {{
                labels: ['Easy', 'Medium', 'Hard'],
                datasets: [{{
                    data: [{}, {}, {}],
                    backgroundColor: ['#28a745', '#ffc107', '#dc3545'],
                    borderWidth: 2,
                    borderColor: '#fff'
                }}]
            }},
            options: {{
                responsive: true,
                plugins: {{
                    legend: {{
                        position: 'bottom'
                    }}
                }}
            }}
        }});

        // Category Distribution Chart
        const categoryCtx = document.getElementById('categoryChart').getContext('2d');
        new Chart(categoryCtx, {{
            type: 'bar',
            data: {{
                labels: ['Variables', 'Functions', 'Loops', 'Structs', 'Ownership', 'Other'],
                datasets: [{{
                    label: 'Code Practices',
                    data: [{}, {}, {}, {}, {}, {}],
                    backgroundColor: ['#007bff', '#28a745', '#ffc107', '#dc3545', '#6f42c1', '#17a2b8'],
                    borderWidth: 1
                }}]
            }},
            options: {{
                responsive: true,
                scales: {{
                    y: {{
                        beginAtZero: true
                    }}
                }}
            }}
        }});

        // Daily Activity Chart
        const activityCtx = document.getElementById('activityChart').getContext('2d');
        new Chart(activityCtx, {{
            type: 'line',
            data: {{
                labels: {},
                datasets: [{{
                    label: 'Active Users',
                    data: {},
                    borderColor: '#007bff',
                    backgroundColor: 'rgba(0, 123, 255, 0.1)',
                    tension: 0.4
                }}, {{
                    label: 'Questions Answered',
                    data: {},
                    borderColor: '#28a745',
                    backgroundColor: 'rgba(40, 167, 69, 0.1)',
                    tension: 0.4
                }}, {{
                    label: 'Code Practices Completed',
                    data: {},
                    borderColor: '#ffc107',
                    backgroundColor: 'rgba(255, 193, 7, 0.1)',
                    tension: 0.4
                }}]
            }},
            options: {{
                responsive: true,
                scales: {{
                    y: {{
                        beginAtZero: true
                    }}
                }}
            }}
        }});
    </script>
</body>
</html>"#,
        analytics_data.total_users,
        analytics_data.active_users_today,
        analytics_data.active_users_this_week,
        analytics_data.active_users_this_month,
        analytics_data.total_topics,
        analytics_data.total_lessons,
        analytics_data.total_questions,
        analytics_data.total_code_practices,
        analytics_data.average_xp_per_user,
        generate_top_users_html(&analytics_data.top_users),
        analytics_data.difficulty_distribution.easy,
        analytics_data.difficulty_distribution.medium,
        analytics_data.difficulty_distribution.hard,
        analytics_data.category_distribution.variables,
        analytics_data.category_distribution.functions,
        analytics_data.category_distribution.loops,
        analytics_data.category_distribution.structs,
        analytics_data.category_distribution.ownership,
        analytics_data.category_distribution.other,
        generate_daily_activity_labels(&analytics_data.daily_activity),
        generate_daily_activity_users_data(&analytics_data.daily_activity),
        generate_daily_activity_questions_data(&analytics_data.daily_activity),
        generate_daily_activity_practices_data(&analytics_data.daily_activity)
    );

    Ok(Html(html))
}

fn generate_top_users_html(top_users: &[TopUser]) -> String {
    let mut html = String::new();

    for (index, user) in top_users.iter().enumerate() {
        let rank_emoji = match index {
            0 => "🥇",
            1 => "🥈",
            2 => "🥉",
            _ => "🏅",
        };

        html.push_str(&format!(
            r#"<div class="user-item">
                <div class="user-info">
                    <div class="user-name">{} {}</div>
                    <div class="user-email">{}</div>
                </div>
                <div class="user-stats">
                    <div class="user-xp">{} XP</div>
                    <div class="user-streak">Streak: {} days</div>
                </div>
            </div>"#,
            rank_emoji, user.display_name, user.email, user.total_xp, user.current_streak
        ));
    }

    html
}

fn generate_mock_daily_activity() -> Vec<DailyActivity> {
    let mut activity = Vec::new();
    let today = Utc::now().date_naive();

    for i in 0..7 {
        let date = today - chrono::Duration::days(6 - i);
        activity.push(DailyActivity {
            date: date.format("%m/%d").to_string(),
            users_active: (20 + (i * 3) + (i % 2) * 5) as u32,
            questions_answered: (15 + (i * 2) + (i % 3) * 3) as u32,
            code_practices_completed: (8 + i + (i % 2) * 2) as u32,
        });
    }

    activity
}

fn generate_daily_activity_labels(activity: &[DailyActivity]) -> String {
    let labels: Vec<String> = activity.iter().map(|a| format!("\"{}\"", a.date)).collect();
    format!("[{}]", labels.join(", "))
}

fn generate_daily_activity_users_data(activity: &[DailyActivity]) -> String {
    let data: Vec<String> = activity
        .iter()
        .map(|a| a.users_active.to_string())
        .collect();
    format!("[{}]", data.join(", "))
}

fn generate_daily_activity_questions_data(activity: &[DailyActivity]) -> String {
    let data: Vec<String> = activity
        .iter()
        .map(|a| a.questions_answered.to_string())
        .collect();
    format!("[{}]", data.join(", "))
}

fn generate_daily_activity_practices_data(activity: &[DailyActivity]) -> String {
    let data: Vec<String> = activity
        .iter()
        .map(|a| a.code_practices_completed.to_string())
        .collect();
    format!("[{}]", data.join(", "))
}
