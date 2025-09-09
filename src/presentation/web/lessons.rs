use crate::application::state::AppState;
use crate::domain::entities::Lesson;
use crate::domain::value_objects::{LessonId, LocalizedText, TopicId};
use crate::shared::errors::Result;
use axum::{extract::State, response::Html, Form};

#[derive(serde::Deserialize)]
pub struct CreateLessonForm {
    pub title_en: String,
    pub title_id: String,
    pub topic_id: String,
    pub summary_en: String,
    pub summary_id: String,
    pub order: u32,
    pub attribution_url: String,
}

pub async fn lessons_handler(State(state): State<AppState>) -> Result<Html<String>> {
    // Fetch real lessons from database
    let lessons = state.lesson_repository.list(100, 0).await?;

    let mut lessons_html = String::new();
    for lesson in &lessons {
        lessons_html.push_str(&format!(
            "<div><h3>{}</h3><p>{}</p><p>Topic ID: {}</p><p>Order: {}</p></div>",
            lesson.title.get("en"),
            lesson.summary.get("en"),
            lesson.topic_id.to_string(),
            lesson.order
        ));
    }

    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head><title>Lessons Management</title></head>
<body>
    <h1>Lessons Management</h1>
    <p>Total Lessons: {}</p>
    {}
</body>
</html>"#,
        lessons.len(),
        lessons_html
    );

    Ok(Html(html))
}

pub async fn create_lesson_handler(State(state): State<AppState>) -> Result<Html<String>> {
    // Fetch topics for dropdown
    let topics = state.topic_repository.list(100, 0).await?;

    let mut topic_options = String::new();
    for topic in &topics {
        topic_options.push_str(&format!(
            "<option value=\"{}\">{}</option>",
            topic.id.to_string(),
            topic.title.get("en")
        ));
    }

    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Create New Lesson</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; }}
        .form-group {{ margin-bottom: 20px; }}
        label {{ display: block; margin-bottom: 5px; font-weight: bold; }}
        input, textarea, select {{ width: 100%; padding: 8px; border: 1px solid #ddd; border-radius: 4px; }}
        button {{ background-color: #007bff; color: white; padding: 10px 20px; border: none; border-radius: 4px; cursor: pointer; }}
        button:hover {{ background-color: #0056b3; }}
    </style>
</head>
<body>
    <h1>Create New Lesson</h1>
    <form action="/admin/lessons" method="POST">
        <div class="form-group">
            <label for="title_en">Title (English):</label>
            <input type="text" id="title_en" name="title_en" required>
        </div>
        <div class="form-group">
            <label for="title_id">Title (Indonesian):</label>
            <input type="text" id="title_id" name="title_id" required>
        </div>
        <div class="form-group">
            <label for="topic_id">Topic:</label>
            <select id="topic_id" name="topic_id" required>
                <option value="">Select a topic...</option>
                {}
            </select>
        </div>
        <div class="form-group">
            <label for="summary_en">Summary (English):</label>
            <textarea id="summary_en" name="summary_en" rows="4" required></textarea>
        </div>
        <div class="form-group">
            <label for="summary_id">Summary (Indonesian):</label>
            <textarea id="summary_id" name="summary_id" rows="4" required></textarea>
        </div>
        <div class="form-group">
            <label for="order">Order:</label>
            <input type="number" id="order" name="order" min="1" value="1" required>
        </div>
        <div class="form-group">
            <label for="attribution_url">Attribution URL:</label>
            <input type="url" id="attribution_url" name="attribution_url">
        </div>
        <button type="submit">Create Lesson</button>
    </form>
    <p><a href="/admin/lessons">← Back to Lessons</a></p>
</body>
</html>"#,
        topic_options
    );

    Ok(Html(html))
}

pub async fn create_lesson_post_handler(
    State(state): State<AppState>,
    Form(form): Form<CreateLessonForm>,
) -> Result<Html<String>> {
    // Parse topic_id
    let topic_id = TopicId::from_string(form.topic_id.clone())
        .map_err(|e| anyhow::anyhow!("Invalid topic ID: {}", e))?;

    // Create LocalizedText objects
    let title = LocalizedText::new(form.title_en, form.title_id);
    let summary = LocalizedText::new(form.summary_en, form.summary_id);

    // Create new lesson
    let lesson = Lesson::new(title, topic_id, summary, form.attribution_url, form.order);

    // Save to database
    state.lesson_repository.create(&lesson).await?;

    // Return success page
    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Lesson Created Successfully</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; text-align: center; }}
        .success {{ color: #28a745; font-size: 24px; margin-bottom: 20px; }}
        .info {{ background-color: #f8f9fa; padding: 20px; border-radius: 5px; margin: 20px 0; }}
        a {{ color: #007bff; text-decoration: none; }}
        a:hover {{ text-decoration: underline; }}
    </style>
</head>
<body>
    <div class="success">✅ Lesson Created Successfully!</div>
    <div class="info">
        <h3>Lesson Details:</h3>
        <p><strong>Title:</strong> {}</p>
        <p><strong>Topic ID:</strong> {}</p>
        <p><strong>Order:</strong> {}</p>
        <p><strong>Lesson ID:</strong> {}</p>
    </div>
    <p><a href="/admin/lessons">← Back to Lessons</a></p>
    <p><a href="/admin/lessons/new">Create Another Lesson</a></p>
</body>
</html>"#,
        lesson.title.get("en"),
        lesson.topic_id.to_string(),
        lesson.order,
        lesson.id.to_string()
    );

    Ok(Html(html))
}

pub async fn edit_lesson_handler(
    State(state): State<AppState>,
    axum::extract::Path(lesson_id): axum::extract::Path<String>,
) -> Result<Html<String>> {
    // Parse lesson_id
    let lesson_id = LessonId::from_string(lesson_id.clone())
        .map_err(|e| anyhow::anyhow!("Invalid lesson ID: {}", e))?;

    // Fetch lesson from database
    let lesson = state
        .lesson_repository
        .find_by_id(&lesson_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Lesson not found"))?;

    // Fetch topics for dropdown
    let topics = state.topic_repository.list(100, 0).await?;

    let mut topic_options = String::new();
    for topic in &topics {
        let selected = if topic.id == lesson.topic_id {
            "selected"
        } else {
            ""
        };
        topic_options.push_str(&format!(
            "<option value=\"{}\" {}>{}</option>",
            topic.id.to_string(),
            selected,
            topic.title.get("en")
        ));
    }

    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Edit Lesson</title>
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
    <h1>Edit Lesson</h1>
    <form action="/admin/lessons/{}/update" method="POST">
        <div class="form-group">
            <label for="title_en">Title (English):</label>
            <input type="text" id="title_en" name="title_en" value="{}" required>
        </div>
        <div class="form-group">
            <label for="title_id">Title (Indonesian):</label>
            <input type="text" id="title_id" name="title_id" value="{}" required>
        </div>
        <div class="form-group">
            <label for="topic_id">Topic:</label>
            <select id="topic_id" name="topic_id" required>
                <option value="">Select a topic...</option>
                {}
            </select>
        </div>
        <div class="form-group">
            <label for="summary_en">Summary (English):</label>
            <textarea id="summary_en" name="summary_en" rows="4" required>{}</textarea>
        </div>
        <div class="form-group">
            <label for="summary_id">Summary (Indonesian):</label>
            <textarea id="summary_id" name="summary_id" rows="4" required>{}</textarea>
        </div>
        <div class="form-group">
            <label for="order">Order:</label>
            <input type="number" id="order" name="order" min="1" value="{}" required>
        </div>
        <div class="form-group">
            <label for="attribution_url">Attribution URL:</label>
            <input type="url" id="attribution_url" name="attribution_url" value="{}">
        </div>
        <button type="submit">Update Lesson</button>
        <button type="button" class="delete-btn" onclick="confirmDelete()">Delete Lesson</button>
    </form>
    <p><a href="/admin/lessons">← Back to Lessons</a></p>

    <script>
        function confirmDelete() {{
            if (confirm('Are you sure you want to delete this lesson? This action cannot be undone.')) {{
                window.location.href = '/admin/lessons/{}/delete';
            }}
        }}
    </script>
</body>
</html>"#,
        lesson_id.to_string(),
        lesson.title.get("en"),
        lesson.title.get("id"),
        topic_options,
        lesson.summary.get("en"),
        lesson.summary.get("id"),
        lesson.order,
        lesson.attribution_url,
        lesson_id.to_string()
    );

    Ok(Html(html))
}

pub async fn update_lesson_post_handler(
    State(state): State<AppState>,
    axum::extract::Path(lesson_id): axum::extract::Path<String>,
    Form(form): Form<CreateLessonForm>,
) -> Result<Html<String>> {
    // Parse lesson_id
    let lesson_id = LessonId::from_string(lesson_id.clone())
        .map_err(|e| anyhow::anyhow!("Invalid lesson ID: {}", e))?;

    // Parse topic_id
    let topic_id = TopicId::from_string(form.topic_id.clone())
        .map_err(|e| anyhow::anyhow!("Invalid topic ID: {}", e))?;

    // Create LocalizedText objects
    let title = LocalizedText::new(form.title_en, form.title_id);
    let summary = LocalizedText::new(form.summary_en, form.summary_id);

    // Create updated lesson
    let mut lesson = state
        .lesson_repository
        .find_by_id(&lesson_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Lesson not found"))?;

    // Update fields
    lesson.title = title;
    lesson.topic_id = topic_id;
    lesson.summary = summary;
    lesson.attribution_url = form.attribution_url;
    lesson.order = form.order;

    // Save to database
    state.lesson_repository.update(&lesson).await?;

    // Return success page
    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Lesson Updated Successfully</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; text-align: center; }}
        .success {{ color: #28a745; font-size: 24px; margin-bottom: 20px; }}
        .info {{ background-color: #f8f9fa; padding: 20px; border-radius: 5px; margin: 20px 0; }}
        a {{ color: #007bff; text-decoration: none; }}
        a:hover {{ text-decoration: underline; }}
    </style>
</head>
<body>
    <div class="success">✅ Lesson Updated Successfully!</div>
    <div class="info">
        <h3>Updated Lesson Details:</h3>
        <p><strong>Title:</strong> {}</p>
        <p><strong>Topic ID:</strong> {}</p>
        <p><strong>Order:</strong> {}</p>
        <p><strong>Lesson ID:</strong> {}</p>
    </div>
    <p><a href="/admin/lessons">← Back to Lessons</a></p>
    <p><a href="/admin/lessons/{}/edit">Edit Again</a></p>
</body>
</html>"#,
        lesson.title.get("en"),
        lesson.topic_id.to_string(),
        lesson.order,
        lesson.id.to_string(),
        lesson.id.to_string()
    );

    Ok(Html(html))
}

pub async fn delete_lesson_handler(
    State(state): State<AppState>,
    axum::extract::Path(lesson_id): axum::extract::Path<String>,
) -> Result<Html<String>> {
    // Parse lesson_id
    let lesson_id = LessonId::from_string(lesson_id.clone())
        .map_err(|e| anyhow::anyhow!("Invalid lesson ID: {}", e))?;

    // Delete from database
    state.lesson_repository.delete(&lesson_id).await?;

    // Return success page
    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Lesson Deleted Successfully</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; text-align: center; }}
        .success {{ color: #dc3545; font-size: 24px; margin-bottom: 20px; }}
        .info {{ background-color: #f8f9fa; padding: 20px; border-radius: 5px; margin: 20px 0; }}
        a {{ color: #007bff; text-decoration: none; }}
        a:hover {{ text-decoration: underline; }}
    </style>
</head>
<body>
    <div class="success">🗑️ Lesson Deleted Successfully!</div>
    <div class="info">
        <h3>Deleted Lesson ID:</h3>
        <p><strong>Lesson ID:</strong> {}</p>
    </div>
    <p><a href="/admin/lessons">← Back to Lessons</a></p>
    <p><a href="/admin/lessons/new">Create New Lesson</a></p>
</body>
</html>"#,
        lesson_id.to_string()
    );

    Ok(Html(html))
}
