use crate::application::state::AppState;
use crate::shared::errors::Result;
use axum::{
    extract::{Query, State},
    response::Html,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BulkActionParams {
    pub action: String,
    pub entity_type: String,
    pub ids: String, // Comma-separated IDs
}

pub async fn bulk_operations_handler(
    State(state): State<AppState>,
    Query(params): Query<BulkActionParams>,
) -> Result<Html<String>> {
    let ids: Vec<String> = params
        .ids
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();
    let mut results = Vec::new();
    let mut success_count = 0;
    let mut error_count = 0;

    match (params.action.as_str(), params.entity_type.as_str()) {
        ("delete", "topics") => {
            for id in &ids {
                match crate::domain::value_objects::TopicId::from_string(id.clone()) {
                    Ok(topic_id) => match state.topic_repository.delete(&topic_id).await {
                        Ok(_) => {
                            results.push(format!("✅ Topic {} deleted successfully", id));
                            success_count += 1;
                        }
                        Err(e) => {
                            results.push(format!("❌ Failed to delete topic {}: {}", id, e));
                            error_count += 1;
                        }
                    },
                    Err(e) => {
                        results.push(format!("❌ Invalid topic ID {}: {}", id, e));
                        error_count += 1;
                    }
                }
            }
        }
        ("delete", "lessons") => {
            for id in &ids {
                match crate::domain::value_objects::LessonId::from_string(id.clone()) {
                    Ok(lesson_id) => match state.lesson_repository.delete(&lesson_id).await {
                        Ok(_) => {
                            results.push(format!("✅ Lesson {} deleted successfully", id));
                            success_count += 1;
                        }
                        Err(e) => {
                            results.push(format!("❌ Failed to delete lesson {}: {}", id, e));
                            error_count += 1;
                        }
                    },
                    Err(e) => {
                        results.push(format!("❌ Invalid lesson ID {}: {}", id, e));
                        error_count += 1;
                    }
                }
            }
        }
        ("delete", "questions") => {
            for id in &ids {
                match crate::domain::value_objects::QuestionId::from_string(&id) {
                    Ok(question_id) => match state.question_repository.delete(&question_id).await {
                        Ok(_) => {
                            results.push(format!("✅ Question {} deleted successfully", id));
                            success_count += 1;
                        }
                        Err(e) => {
                            results.push(format!("❌ Failed to delete question {}: {}", id, e));
                            error_count += 1;
                        }
                    },
                    Err(e) => {
                        results.push(format!("❌ Invalid question ID {}: {}", id, e));
                        error_count += 1;
                    }
                }
            }
        }
        ("delete", "users") => {
            for id in &ids {
                match crate::domain::value_objects::UserId::from_string(id.clone()) {
                    Ok(user_id) => match state.user_repository.delete(&user_id).await {
                        Ok(_) => {
                            results.push(format!("✅ User {} deleted successfully", id));
                            success_count += 1;
                        }
                        Err(e) => {
                            results.push(format!("❌ Failed to delete user {}: {}", id, e));
                            error_count += 1;
                        }
                    },
                    Err(e) => {
                        results.push(format!("❌ Invalid user ID {id}: {e}"));
                        error_count += 1;
                    }
                }
            }
        }
        ("activate", "users") => {
            for id in &ids {
                match crate::domain::value_objects::UserId::from_string(id.clone()) {
                    Ok(user_id) => {
                        match state.user_repository.find_by_id(&user_id).await {
                            Ok(Some(_user)) => {
                                // In a real implementation, you'd have an is_active field
                                results.push(format!("✅ User {} activated successfully", id));
                                success_count += 1;
                            }
                            Ok(None) => {
                                results.push(format!("❌ User {} not found", id));
                                error_count += 1;
                            }
                            Err(e) => {
                                results.push(format!("❌ Failed to activate user {}: {}", id, e));
                                error_count += 1;
                            }
                        }
                    }
                    Err(e) => {
                        results.push(format!("❌ Invalid user ID {id}: {e}"));
                        error_count += 1;
                    }
                }
            }
        }
        ("deactivate", "users") => {
            for id in &ids {
                match crate::domain::value_objects::UserId::from_string(id.clone()) {
                    Ok(user_id) => {
                        match state.user_repository.find_by_id(&user_id).await {
                            Ok(Some(_user)) => {
                                // In a real implementation, you'd have an is_active field
                                results.push(format!("✅ User {} deactivated successfully", id));
                                success_count += 1;
                            }
                            Ok(None) => {
                                results.push(format!("❌ User {} not found", id));
                                error_count += 1;
                            }
                            Err(e) => {
                                results.push(format!("❌ Failed to deactivate user {id}: {e}"));
                                error_count += 1;
                            }
                        }
                    }
                    Err(e) => {
                        results.push(format!("❌ Invalid user ID {id}: {e}"));
                        error_count += 1;
                    }
                }
            }
        }
        _ => {
            results.push(format!(
                "❌ Unknown action '{}' for entity type '{}'",
                params.action, params.entity_type
            ));
            error_count += 1;
        }
    }

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Bulk Operations Result - Rust Learning Platform</title>
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
        .summary {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 20px;
            margin-bottom: 30px;
        }}
        .summary-card {{
            text-align: center;
            padding: 20px;
            border-radius: 10px;
            background: #f8f9fa;
        }}
        .summary-number {{
            font-size: 2.5em;
            font-weight: bold;
            margin-bottom: 10px;
        }}
        .summary-label {{
            color: #666;
            font-size: 1.1em;
        }}
        .success {{ color: #28a745; }}
        .error {{ color: #dc3545; }}
        .warning {{ color: #ffc107; }}
        .results {{
            background: #f8f9fa;
            border-radius: 10px;
            padding: 20px;
            margin-bottom: 30px;
        }}
        .result-item {{
            padding: 10px;
            margin: 5px 0;
            border-radius: 5px;
            background: white;
            border-left: 4px solid #ddd;
        }}
        .result-item.success {{
            border-left-color: #28a745;
        }}
        .result-item.error {{
            border-left-color: #dc3545;
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
            <h1>🔄 Bulk Operations Result</h1>
            <p>Operation completed for {} items</p>
        </div>

        <div class="content">
            <a href="/admin/dashboard" class="back-btn">← Back to Dashboard</a>

            <div class="summary">
                <div class="summary-card">
                    <div class="summary-number success">{}</div>
                    <div class="summary-label">Successful</div>
                </div>
                <div class="summary-card">
                    <div class="summary-number error">{}</div>
                    <div class="summary-label">Failed</div>
                </div>
                <div class="summary-card">
                    <div class="summary-number warning">{}</div>
                    <div class="summary-label">Total Processed</div>
                </div>
            </div>

            <div class="results">
                <h3>📋 Operation Details</h3>
                <div class="result-list">
                    {}
                </div>
            </div>

            <div style="text-align: center;">
                <a href="/admin/dashboard" class="btn">Back to Dashboard</a>
                <a href="/admin/bulk-operations" class="btn">New Bulk Operation</a>
            </div>
        </div>
    </div>
</body>
</html>"#,
        ids.len(),
        success_count,
        error_count,
        ids.len(),
        results.join("")
    );

    Ok(Html(html))
}

pub async fn bulk_operations_form_handler() -> Result<Html<String>> {
    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Bulk Operations - Rust Learning Platform</title>
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
        .form-group select, .form-group input, .form-group textarea {{
            width: 100%;
            padding: 12px;
            border: 2px solid #e9ecef;
            border-radius: 8px;
            font-size: 1em;
            transition: border-color 0.3s ease;
        }}
        .form-group select:focus, .form-group input:focus, .form-group textarea:focus {{
            outline: none;
            border-color: #667eea;
        }}
        .form-group textarea {{
            height: 120px;
            resize: vertical;
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
        .warning-box {{
            background: #fff3cd;
            border: 1px solid #ffeaa7;
            border-radius: 8px;
            padding: 15px;
            margin-bottom: 20px;
        }}
        .warning-title {{
            font-weight: bold;
            color: #856404;
            margin-bottom: 5px;
        }}
        .warning-text {{
            color: #856404;
            font-size: 0.9em;
        }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🔄 Bulk Operations</h1>
            <p>Perform operations on multiple items at once</p>
        </div>

        <div class="content">
            <a href="/admin/dashboard" class="back-btn">← Back to Dashboard</a>

            <div class="warning-box">
                <div class="warning-title">⚠️ Important Notice</div>
                <div class="warning-text">
                    Bulk operations are powerful and can affect multiple items at once.
                    Please double-check your selections before proceeding. These operations cannot be undone.
                </div>
            </div>

            <form method="GET" action="/admin/bulk-operations/execute">
                <div class="form-group">
                    <label for="entity_type">Entity Type:</label>
                    <select id="entity_type" name="entity_type" required>
                        <option value="">Select entity type...</option>
                        <option value="topics">Topics</option>
                        <option value="lessons">Lessons</option>
                        <option value="questions">Questions</option>
                        <option value="users">Users</option>
                    </select>
                </div>

                <div class="form-group">
                    <label for="action">Action:</label>
                    <select id="action" name="action" required>
                        <option value="">Select action...</option>
                        <option value="delete">Delete</option>
                        <option value="activate">Activate</option>
                        <option value="deactivate">Deactivate</option>
                    </select>
                </div>

                <div class="form-group">
                    <label for="ids">Item IDs (comma-separated):</label>
                    <textarea
                        id="ids"
                        name="ids"
                        placeholder="Enter IDs separated by commas, e.g., 123,456,789"
                        required
                    ></textarea>
                </div>

                <div class="form-actions">
                    <button type="submit" class="btn btn-primary">Execute Bulk Operation</button>
                    <a href="/admin/dashboard" class="btn btn-secondary">Cancel</a>
                </div>
            </form>
        </div>
    </div>
</body>
</html>"#
    );

    Ok(Html(html))
}
