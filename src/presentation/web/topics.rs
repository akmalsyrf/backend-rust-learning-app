use crate::application::state::AppState;
use crate::domain::entities::Topic;
use crate::domain::value_objects::{LocalizedText, TopicId};
use crate::presentation::web::pagination::{
    generate_pagination_controls, generate_pagination_html, generate_pagination_info,
    get_pagination_css, PaginationParams,
};
use crate::shared::errors::Result;
use axum::{
    extract::{Query, State},
    response::Html,
    Form,
};

#[derive(serde::Deserialize)]
pub struct CreateTopicForm {
    pub title_en: String,
    pub title_id: String,
    pub description_en: String,
    pub description_id: String,
    pub order: u32,
    pub required_skills_en: String,
    pub required_skills_id: String,
}

pub async fn topics_handler(
    State(state): State<AppState>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Html<String>> {
    let page = pagination.page();
    let limit = pagination.limit();
    let offset = pagination.offset();

    // Fetch topics with pagination
    let topics = state.topic_repository.list(limit, offset).await?;

    // Get total count for pagination (simplified - in real app, you'd have a count method)
    let all_topics = state.topic_repository.list(1000, 0).await?;
    let total_count = all_topics.len();

    let mut topics_html = String::new();
    for topic in &topics {
        topics_html.push_str(&format!(
            r#"<tr>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td>
                    <a href="/admin/topics/{}/edit" class="btn btn-sm btn-primary">Edit</a>
                    <a href="/admin/topics/{}/delete" class="btn btn-sm btn-danger" onclick="return confirm('Are you sure?')">Delete</a>
                </td>
            </tr>"#,
            topic.title.get("en"),
            topic.description.get("en"),
            topic.order,
            topic.required_skills.get("en"),
            topic.id.to_string(),
            topic.id.to_string()
        ));
    }

    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Topics Management</title>
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
        {}
    </style>
</head>
<body>
    <div class="header">
        <h1>📚 Topics Management</h1>
        <a href="/admin/topics/new" class="btn btn-success">+ Add New Topic</a>
    </div>

    <div class="stats">
        <div class="stat-card">
            <div class="stat-number">{}</div>
            <div class="stat-label">Total Topics</div>
        </div>
    </div>

    {}
    <div class="pagination-info">{}</div>

    <table>
        <thead>
            <tr>
                <th>Title</th>
                <th>Description</th>
                <th>Order</th>
                <th>Required Skills</th>
                <th>Actions</th>
            </tr>
        </thead>
        <tbody>
            {}
        </tbody>
    </table>

    <div class="pagination">
        {}
    </div>

    <div style="margin-top: 30px;">
        <a href="/admin" class="btn btn-primary">← Back to Dashboard</a>
    </div>
</body>
</html>"#,
        get_pagination_css(),
        total_count,
        generate_pagination_controls(page, limit, total_count),
        generate_pagination_info(page, limit, total_count),
        topics_html,
        generate_pagination_html(page, limit, total_count, "/admin/topics", "")
    );

    Ok(Html(html))
}

pub async fn create_topic_handler() -> Result<Html<String>> {
    let html = r#"<!DOCTYPE html>
<html>
<head>
    <title>Create New Topic</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; }
        .form-group { margin-bottom: 20px; }
        label { display: block; margin-bottom: 5px; font-weight: bold; }
        input, textarea, select { width: 100%; padding: 8px; border: 1px solid #ddd; border-radius: 4px; }
        button { background-color: #007bff; color: white; padding: 10px 20px; border: none; border-radius: 4px; cursor: pointer; }
        button:hover { background-color: #0056b3; }
    </style>
</head>
<body>
    <h1>Create New Topic</h1>
    <form action="/admin/topics" method="POST">
        <div class="form-group">
            <label for="title_en">Title (English):</label>
            <input type="text" id="title_en" name="title_en" required>
        </div>
        <div class="form-group">
            <label for="title_id">Title (Indonesian):</label>
            <input type="text" id="title_id" name="title_id" required>
        </div>
        <div class="form-group">
            <label for="description_en">Description (English):</label>
            <textarea id="description_en" name="description_en" rows="4" required></textarea>
        </div>
        <div class="form-group">
            <label for="description_id">Description (Indonesian):</label>
            <textarea id="description_id" name="description_id" rows="4" required></textarea>
        </div>
        <div class="form-group">
            <label for="order">Order:</label>
            <input type="number" id="order" name="order" min="1" value="1" required>
        </div>
        <div class="form-group">
            <label for="required_skills_en">Required Skills (English):</label>
            <textarea id="required_skills_en" name="required_skills_en" rows="3"></textarea>
        </div>
        <div class="form-group">
            <label for="required_skills_id">Required Skills (Indonesian):</label>
            <textarea id="required_skills_id" name="required_skills_id" rows="3"></textarea>
        </div>
        <button type="submit">Create Topic</button>
    </form>
    <p><a href="/admin/topics">← Back to Topics</a></p>
</body>
</html>"#.to_string();

    Ok(Html(html))
}

pub async fn create_topic_post_handler(
    State(state): State<AppState>,
    Form(form): Form<CreateTopicForm>,
) -> Result<Html<String>> {
    // Create LocalizedText objects
    let title = LocalizedText::new(form.title_en, form.title_id);
    let description = LocalizedText::new(form.description_en, form.description_id);
    let required_skills = LocalizedText::new(form.required_skills_en, form.required_skills_id);

    // Create new topic
    let topic = Topic::new(title, description, form.order, required_skills);

    // Save to database
    state.topic_repository.create(&topic).await?;

    // Return success page
    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Topic Created Successfully</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; text-align: center; }}
        .success {{ color: #28a745; font-size: 24px; margin-bottom: 20px; }}
        .info {{ background-color: #f8f9fa; padding: 20px; border-radius: 5px; margin: 20px 0; }}
        a {{ color: #007bff; text-decoration: none; }}
        a:hover {{ text-decoration: underline; }}
    </style>
</head>
<body>
    <div class="success">✅ Topic Created Successfully!</div>
    <div class="info">
        <h3>Topic Details:</h3>
        <p><strong>Title:</strong> {}</p>
        <p><strong>Order:</strong> {}</p>
        <p><strong>Topic ID:</strong> {}</p>
    </div>
    <p><a href="/admin/topics">← Back to Topics</a></p>
    <p><a href="/admin/topics/new">Create Another Topic</a></p>
</body>
</html>"#,
        topic.title.get("en"),
        topic.order,
        topic.id.to_string()
    );

    Ok(Html(html))
}

pub async fn edit_topic_handler(
    State(state): State<AppState>,
    axum::extract::Path(topic_id): axum::extract::Path<String>,
) -> Result<Html<String>> {
    // Parse topic_id
    let topic_id = TopicId::from_string(topic_id.clone())
        .map_err(|e| anyhow::anyhow!("Invalid topic ID: {}", e))?;

    // Fetch topic from database
    let topic = state
        .topic_repository
        .find_by_id(&topic_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Topic not found"))?;

    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Edit Topic</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; }}
        .form-group {{ margin-bottom: 20px; }}
        label {{ display: block; margin-bottom: 5px; font-weight: bold; }}
        input, textarea, select {{ width: 100%; padding: 8px; border: 1px solid #ddd; border-radius: 4px; }}
        button {{ background-color: #28a745; color: white; padding: 10px 20px; border: none; border-radius: 4px; cursor: pointer; }}
        button:hover {{ background-color: #218838; }}
        .delete-btn {{ background-color: #dc3545; margin-left: 10px; }}
        .delete-btn:hover {{ background-color: #c82333; }}
    </style>
</head>
<body>
    <h1>Edit Topic</h1>
    <form action="/admin/topics/{}/update" method="POST">
        <div class="form-group">
            <label for="title_en">Title (English):</label>
            <input type="text" id="title_en" name="title_en" value="{}" required>
        </div>
        <div class="form-group">
            <label for="title_id">Title (Indonesian):</label>
            <input type="text" id="title_id" name="title_id" value="{}" required>
        </div>
        <div class="form-group">
            <label for="description_en">Description (English):</label>
            <textarea id="description_en" name="description_en" rows="4" required>{}</textarea>
        </div>
        <div class="form-group">
            <label for="description_id">Description (Indonesian):</label>
            <textarea id="description_id" name="description_id" rows="4" required>{}</textarea>
        </div>
        <div class="form-group">
            <label for="order">Order:</label>
            <input type="number" id="order" name="order" min="1" value="{}" required>
        </div>
        <div class="form-group">
            <label for="required_skills_en">Required Skills (English):</label>
            <textarea id="required_skills_en" name="required_skills_en" rows="3">{}</textarea>
        </div>
        <div class="form-group">
            <label for="required_skills_id">Required Skills (Indonesian):</label>
            <textarea id="required_skills_id" name="required_skills_id" rows="3">{}</textarea>
        </div>
        <button type="submit">Update Topic</button>
        <button type="button" class="delete-btn" onclick="confirmDelete()">Delete Topic</button>
    </form>
    <p><a href="/admin/topics">← Back to Topics</a></p>

    <script>
        function confirmDelete() {{
            if (confirm('Are you sure you want to delete this topic? This action cannot be undone.')) {{
                window.location.href = '/admin/topics/{}/delete';
            }}
        }}
    </script>
</body>
</html>"#,
        topic_id.to_string(),
        topic.title.get("en"),
        topic.title.get("id"),
        topic.description.get("en"),
        topic.description.get("id"),
        topic.order,
        topic.required_skills.get("en"),
        topic.required_skills.get("id"),
        topic_id.to_string()
    );

    Ok(Html(html))
}

pub async fn update_topic_post_handler(
    State(state): State<AppState>,
    axum::extract::Path(topic_id): axum::extract::Path<String>,
    Form(form): Form<CreateTopicForm>,
) -> Result<Html<String>> {
    // Parse topic_id
    let topic_id = TopicId::from_string(topic_id.clone())
        .map_err(|e| anyhow::anyhow!("Invalid topic ID: {}", e))?;

    // Create LocalizedText objects
    let title = LocalizedText::new(form.title_en, form.title_id);
    let description = LocalizedText::new(form.description_en, form.description_id);
    let required_skills = LocalizedText::new(form.required_skills_en, form.required_skills_id);

    // Create updated topic
    let mut topic = state
        .topic_repository
        .find_by_id(&topic_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Topic not found"))?;

    // Update fields
    topic.title = title;
    topic.description = description;
    topic.order = form.order;
    topic.required_skills = required_skills;

    // Save to database
    state.topic_repository.update(&topic).await?;

    // Return success page
    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Topic Updated Successfully</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; text-align: center; }}
        .success {{ color: #28a745; font-size: 24px; margin-bottom: 20px; }}
        .info {{ background-color: #f8f9fa; padding: 20px; border-radius: 5px; margin: 20px 0; }}
        a {{ color: #007bff; text-decoration: none; }}
        a:hover {{ text-decoration: underline; }}
    </style>
</head>
<body>
    <div class="success">✅ Topic Updated Successfully!</div>
    <div class="info">
        <h3>Updated Topic Details:</h3>
        <p><strong>Title:</strong> {}</p>
        <p><strong>Order:</strong> {}</p>
        <p><strong>Topic ID:</strong> {}</p>
    </div>
    <p><a href="/admin/topics">← Back to Topics</a></p>
    <p><a href="/admin/topics/{}/edit">Edit Again</a></p>
</body>
</html>"#,
        topic.title.get("en"),
        topic.order,
        topic.id.to_string(),
        topic.id.to_string()
    );

    Ok(Html(html))
}

pub async fn delete_topic_handler(
    State(state): State<AppState>,
    axum::extract::Path(topic_id): axum::extract::Path<String>,
) -> Result<Html<String>> {
    // Parse topic_id
    let topic_id = TopicId::from_string(topic_id.clone())
        .map_err(|e| anyhow::anyhow!("Invalid topic ID: {}", e))?;

    // Delete from database
    state.topic_repository.delete(&topic_id).await?;

    // Return success page
    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Topic Deleted Successfully</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; text-align: center; }}
        .success {{ color: #dc3545; font-size: 24px; margin-bottom: 20px; }}
        .info {{ background-color: #f8f9fa; padding: 20px; border-radius: 5px; margin: 20px 0; }}
        a {{ color: #007bff; text-decoration: none; }}
        a:hover {{ text-decoration: underline; }}
    </style>
</head>
<body>
    <div class="success">🗑️ Topic Deleted Successfully!</div>
    <div class="info">
        <h3>Deleted Topic ID:</h3>
        <p><strong>Topic ID:</strong> {}</p>
    </div>
    <p><a href="/admin/topics">← Back to Topics</a></p>
    <p><a href="/admin/topics/new">Create New Topic</a></p>
</body>
</html>"#,
        topic_id.to_string()
    );

    Ok(Html(html))
}
