use crate::application::state::AppState;
use crate::domain::entities::code_practice::CodePractice;
use crate::domain::value_objects::{
    CodePracticeId, Difficulty, LessonId, LocalizedText, Points, TopicId,
};
use crate::shared::errors::Result;
use axum::{extract::State, response::Html, Form};
use chrono::Utc;

#[derive(serde::Deserialize)]
pub struct CreateCodePracticeForm {
    pub title_en: String,
    pub title_id: String,
    pub description_en: String,
    pub description_id: String,
    pub initial_code: String,
    pub expected_output: String,
    pub solution: String,
    pub hints_en: String,
    pub hints_id: String,
    pub difficulty: String,
    pub category: String,
    pub lesson_id: String,
    pub topic_id: String,
    pub points: u32,
}

pub async fn code_practices_handler(State(state): State<AppState>) -> Result<Html<String>> {
    // Fetch code practices from database
    let code_practices = state.code_practice_repository.list(100, 0).await?;

    let mut practices_html = String::new();
    for practice in &code_practices {
        practices_html.push_str(&format!(
            r#"<tr>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td>
                    <a href="/admin/code-practices/{}/edit" class="btn btn-sm btn-primary">Edit</a>
                    <a href="/admin/code-practices/{}/delete" class="btn btn-sm btn-danger" onclick="return confirm('Are you sure?')">Delete</a>
                </td>
            </tr>"#,
            practice.title.get("en"),
            practice.description.get("en"),
            practice.category,
            match practice.difficulty {
                Difficulty::Beginner => "Easy",
                Difficulty::Intermediate => "Medium",
                Difficulty::Advanced => "Hard",
            },
            practice.points.value(),
            practice.lesson_id.to_string(),
            practice.topic_id.to_string(),
            practice.id.to_string(),
            practice.id.to_string()
        ));
    }

    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Code Practices Management</title>
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
        <h1>💻 Code Practices Management</h1>
        <a href="/admin/code-practices/new" class="btn btn-success">+ Add New Code Practice</a>
    </div>

    <div class="stats">
        <div class="stat-card">
            <div class="stat-number">{}</div>
            <div class="stat-label">Total Code Practices</div>
        </div>
    </div>

    <table>
        <thead>
            <tr>
                <th>Title</th>
                <th>Description</th>
                <th>Category</th>
                <th>Difficulty</th>
                <th>Points</th>
                <th>Lesson ID</th>
                <th>Topic ID</th>
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
        code_practices.len(),
        practices_html
    );

    Ok(Html(html))
}

pub async fn create_code_practice_handler(State(state): State<AppState>) -> Result<Html<String>> {
    // Fetch topics and lessons for dropdowns
    let topics = state.topic_repository.list(100, 0).await?;
    let lessons = state.lesson_repository.list(100, 0).await?;

    let mut topic_options = String::new();
    for topic in &topics {
        topic_options.push_str(&format!(
            "<option value=\"{}\">{}</option>",
            topic.id.to_string(),
            topic.title.get("en")
        ));
    }

    let mut lesson_options = String::new();
    for lesson in &lessons {
        lesson_options.push_str(&format!(
            "<option value=\"{}\">{}</option>",
            lesson.id.to_string(),
            lesson.title.get("en")
        ));
    }

    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Create Code Practice</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; }}
        .form-container {{ max-width: 800px; margin: 0 auto; }}
        .form-group {{ margin-bottom: 20px; }}
        label {{ display: block; margin-bottom: 5px; font-weight: bold; }}
        input, textarea, select {{ width: 100%; padding: 8px; border: 1px solid #ddd; border-radius: 4px; }}
        textarea {{ height: 100px; font-family: monospace; }}
        .code-textarea {{ height: 200px; }}
        button {{ background-color: #28a745; color: white; padding: 10px 20px; border: none; border-radius: 4px; cursor: pointer; }}
        button:hover {{ background-color: #218838; }}
        .back-link {{ color: #007bff; text-decoration: none; }}
        .back-link:hover {{ text-decoration: underline; }}
        .form-row {{ display: flex; gap: 20px; }}
        .form-row .form-group {{ flex: 1; }}
    </style>
</head>
<body>
    <div class="form-container">
        <h1>💻 Create New Code Practice</h1>

        <form action="/admin/code-practices" method="POST">
            <div class="form-row">
                <div class="form-group">
                    <label for="title_en">Title (English):</label>
                    <input type="text" id="title_en" name="title_en" required>
                </div>
                <div class="form-group">
                    <label for="title_id">Title (Indonesian):</label>
                    <input type="text" id="title_id" name="title_id" required>
                </div>
            </div>

            <div class="form-row">
                <div class="form-group">
                    <label for="description_en">Description (English):</label>
                    <textarea id="description_en" name="description_en" required></textarea>
                </div>
                <div class="form-group">
                    <label for="description_id">Description (Indonesian):</label>
                    <textarea id="description_id" name="description_id" required></textarea>
                </div>
            </div>

            <div class="form-group">
                <label for="initial_code">Initial Code:</label>
                <textarea id="initial_code" name="initial_code" class="code-textarea" required placeholder="fn main() {{&#10;    // Your initial code here&#10;}}"></textarea>
            </div>

            <div class="form-group">
                <label for="expected_output">Expected Output (optional):</label>
                <input type="text" id="expected_output" name="expected_output" placeholder="e.g., Hello, world!">
            </div>

            <div class="form-group">
                <label for="solution">Solution Code:</label>
                <textarea id="solution" name="solution" class="code-textarea" required placeholder="fn main() {{&#10;    // Complete solution here&#10;}}"></textarea>
            </div>

            <div class="form-row">
                <div class="form-group">
                    <label for="hints_en">Hints (English):</label>
                    <textarea id="hints_en" name="hints_en" placeholder="Hint 1&#10;Hint 2&#10;Hint 3"></textarea>
                </div>
                <div class="form-group">
                    <label for="hints_id">Hints (Indonesian):</label>
                    <textarea id="hints_id" name="hints_id" placeholder="Petunjuk 1&#10;Petunjuk 2&#10;Petunjuk 3"></textarea>
                </div>
            </div>

            <div class="form-row">
                <div class="form-group">
                    <label for="difficulty">Difficulty:</label>
                    <select id="difficulty" name="difficulty" required>
                        <option value="">Select difficulty...</option>
                        <option value="Easy">Easy</option>
                        <option value="Medium">Medium</option>
                        <option value="Hard">Hard</option>
                    </select>
                </div>
                <div class="form-group">
                    <label for="category">Category:</label>
                    <input type="text" id="category" name="category" required placeholder="e.g., Variables, Functions, Loops">
                </div>
            </div>

            <div class="form-row">
                <div class="form-group">
                    <label for="topic_id">Topic:</label>
                    <select id="topic_id" name="topic_id" required>
                        <option value="">Select topic...</option>
                        {topic_options}
                    </select>
                </div>
                <div class="form-group">
                    <label for="lesson_id">Lesson:</label>
                    <select id="lesson_id" name="lesson_id" required>
                        <option value="">Select lesson...</option>
                        {lesson_options}
                    </select>
                </div>
            </div>

            <div class="form-group">
                <label for="points">Points:</label>
                <input type="number" id="points" name="points" min="1" max="1000" value="10" required>
            </div>

            <button type="submit">Create Code Practice</button>
        </form>

        <p><a href="/admin/code-practices" class="back-link">← Back to Code Practices</a></p>
    </div>
</body>
</html>"#
    );

    Ok(Html(html))
}

pub async fn create_code_practice_post_handler(
    State(state): State<AppState>,
    Form(form): Form<CreateCodePracticeForm>,
) -> Result<Html<String>> {
    // Parse form data
    let title = LocalizedText::new(form.title_en, form.title_id);
    let description = LocalizedText::new(form.description_en, form.description_id);
    let topic_id = TopicId::from_string(form.topic_id.clone())
        .map_err(|e| anyhow::anyhow!("Invalid topic ID: {}", e))?;
    let lesson_id = LessonId::from_string(form.lesson_id.clone())
        .map_err(|e| anyhow::anyhow!("Invalid lesson ID: {}", e))?;
    let difficulty = match form.difficulty.as_str() {
        "Easy" => Difficulty::Beginner,
        "Medium" => Difficulty::Intermediate,
        "Hard" => Difficulty::Advanced,
        _ => return Err(anyhow::anyhow!("Invalid difficulty level").into()),
    };
    let points = Points::new(form.points);

    // Create code practice
    let mut code_practice = CodePractice::new(
        title,
        description,
        form.initial_code,
        form.solution,
        difficulty,
        form.category,
        lesson_id,
        topic_id,
        points,
    );

    // Add expected output if provided
    if !form.expected_output.is_empty() {
        code_practice.set_expected_output(form.expected_output);
    }

    // Add hints if provided
    if !form.hints_en.is_empty() || !form.hints_id.is_empty() {
        let hints_text = LocalizedText::new(form.hints_en, form.hints_id);
        code_practice.add_hint(hints_text);
    }

    // Save to database
    state
        .code_practice_repository
        .create(&code_practice)
        .await?;

    // Return success page
    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Code Practice Created</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; text-align: center; }}
        .success {{ color: #28a745; font-size: 24px; margin-bottom: 20px; }}
        .info {{ background-color: #f8f9fa; padding: 20px; border-radius: 5px; margin: 20px 0; }}
        a {{ color: #007bff; text-decoration: none; }}
        a:hover {{ text-decoration: underline; }}
    </style>
</head>
<body>
    <div class="success">✅ Code Practice Created Successfully!</div>
    <div class="info">
        <h3>Code Practice Details:</h3>
        <p><strong>Title:</strong> {}</p>
        <p><strong>Category:</strong> {}</p>
        <p><strong>Difficulty:</strong> {}</p>
        <p><strong>Points:</strong> {}</p>
        <p><strong>Topic ID:</strong> {}</p>
        <p><strong>Lesson ID:</strong> {}</p>
    </div>
    <p><a href="/admin/code-practices">← Back to Code Practices</a></p>
    <p><a href="/admin/code-practices/new">Create Another Code Practice</a></p>
</body>
</html>"#,
        code_practice.title.get("en"),
        code_practice.category,
        match code_practice.difficulty {
            Difficulty::Beginner => "Easy",
            Difficulty::Intermediate => "Medium",
            Difficulty::Advanced => "Hard",
        },
        code_practice.points.value(),
        code_practice.topic_id.to_string(),
        code_practice.lesson_id.to_string()
    );

    Ok(Html(html))
}

pub async fn edit_code_practice_handler(
    State(state): State<AppState>,
    axum::extract::Path(code_practice_id): axum::extract::Path<String>,
) -> Result<Html<String>> {
    // Parse code practice ID
    let code_practice_id = CodePracticeId::from_string(&code_practice_id)
        .map_err(|e| anyhow::anyhow!("Invalid code practice ID: {}", e))?;

    // Fetch code practice
    let code_practice = state
        .code_practice_repository
        .find_by_id(&code_practice_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Code practice not found"))?;

    // Fetch topics and lessons for dropdowns
    let topics = state.topic_repository.list(100, 0).await?;
    let lessons = state.lesson_repository.list(100, 0).await?;

    let mut topic_options = String::new();
    for topic in &topics {
        let selected = if topic.id == code_practice.topic_id {
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

    let mut lesson_options = String::new();
    for lesson in &lessons {
        let selected = if lesson.id == code_practice.lesson_id {
            "selected"
        } else {
            ""
        };
        lesson_options.push_str(&format!(
            "<option value=\"{}\" {}>{}</option>",
            lesson.id.to_string(),
            selected,
            lesson.title.get("en")
        ));
    }

    // Get current difficulty
    let current_difficulty = match code_practice.difficulty {
        Difficulty::Beginner => "Easy",
        Difficulty::Intermediate => "Medium",
        Difficulty::Advanced => "Hard",
    };

    // Get hints
    let hints_en = code_practice
        .hints
        .first()
        .map(|h| h.get("en"))
        .unwrap_or_default();
    let hints_id = code_practice
        .hints
        .first()
        .map(|h| h.get("id"))
        .unwrap_or_default();

    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Edit Code Practice</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; }}
        .form-container {{ max-width: 800px; margin: 0 auto; }}
        .form-group {{ margin-bottom: 20px; }}
        label {{ display: block; margin-bottom: 5px; font-weight: bold; }}
        input, textarea, select {{ width: 100%; padding: 8px; border: 1px solid #ddd; border-radius: 4px; }}
        textarea {{ height: 100px; font-family: monospace; }}
        .code-textarea {{ height: 200px; }}
        button {{ background-color: #007bff; color: white; padding: 10px 20px; border: none; border-radius: 4px; cursor: pointer; }}
        button:hover {{ background-color: #0056b3; }}
        .btn-danger {{ background-color: #dc3545; }}
        .btn-danger:hover {{ background-color: #c82333; }}
        .back-link {{ color: #007bff; text-decoration: none; }}
        .back-link:hover {{ text-decoration: underline; }}
        .form-row {{ display: flex; gap: 20px; }}
        .form-row .form-group {{ flex: 1; }}
        .delete-section {{ margin-top: 30px; padding-top: 20px; border-top: 1px solid #ddd; }}
    </style>
</head>
<body>
    <div class="form-container">
        <h1>💻 Edit Code Practice</h1>

        <form action="/admin/code-practices/{}/update" method="POST">
            <div class="form-row">
                <div class="form-group">
                    <label for="title_en">Title (English):</label>
                    <input type="text" id="title_en" name="title_en" value="{}" required>
                </div>
                <div class="form-group">
                    <label for="title_id">Title (Indonesian):</label>
                    <input type="text" id="title_id" name="title_id" value="{}" required>
                </div>
            </div>

            <div class="form-row">
                <div class="form-group">
                    <label for="description_en">Description (English):</label>
                    <textarea id="description_en" name="description_en" required>{}</textarea>
                </div>
                <div class="form-group">
                    <label for="description_id">Description (Indonesian):</label>
                    <textarea id="description_id" name="description_id" required>{}</textarea>
                </div>
            </div>

            <div class="form-group">
                <label for="initial_code">Initial Code:</label>
                <textarea id="initial_code" name="initial_code" class="code-textarea" required>{}</textarea>
            </div>

            <div class="form-group">
                <label for="expected_output">Expected Output (optional):</label>
                <input type="text" id="expected_output" name="expected_output" value="{}">
            </div>

            <div class="form-group">
                <label for="solution">Solution Code:</label>
                <textarea id="solution" name="solution" class="code-textarea" required>{}</textarea>
            </div>

            <div class="form-row">
                <div class="form-group">
                    <label for="hints_en">Hints (English):</label>
                    <textarea id="hints_en" name="hints_en">{}</textarea>
                </div>
                <div class="form-group">
                    <label for="hints_id">Hints (Indonesian):</label>
                    <textarea id="hints_id" name="hints_id">{}</textarea>
                </div>
            </div>

            <div class="form-row">
                <div class="form-group">
                    <label for="difficulty">Difficulty:</label>
                    <select id="difficulty" name="difficulty" required>
                        <option value="Easy" {}>Easy</option>
                        <option value="Medium" {}>Medium</option>
                        <option value="Hard" {}>Hard</option>
                    </select>
                </div>
                <div class="form-group">
                    <label for="category">Category:</label>
                    <input type="text" id="category" name="category" value="{}" required>
                </div>
            </div>

            <div class="form-row">
                <div class="form-group">
                    <label for="topic_id">Topic:</label>
                    <select id="topic_id" name="topic_id" required>
                        {}
                    </select>
                </div>
                <div class="form-group">
                    <label for="lesson_id">Lesson:</label>
                    <select id="lesson_id" name="lesson_id" required>
                        {}
                    </select>
                </div>
            </div>

            <div class="form-group">
                <label for="points">Points:</label>
                <input type="number" id="points" name="points" min="1" max="1000" value="{}" required>
            </div>

            <button type="submit">Update Code Practice</button>
        </form>

        <div class="delete-section">
            <h3>⚠️ Danger Zone</h3>
            <p>Once you delete a code practice, there is no going back. Please be certain.</p>
            <a href="/admin/code-practices/{}/delete" class="btn btn-danger" onclick="return confirm('Are you sure you want to delete this code practice? This action cannot be undone.')">Delete Code Practice</a>
        </div>

        <p><a href="/admin/code-practices" class="back-link">← Back to Code Practices</a></p>
    </div>
</body>
</html>"#,
        code_practice.id.to_string(),
        code_practice.title.get("en"),
        code_practice.title.get("id"),
        code_practice.description.get("en"),
        code_practice.description.get("id"),
        code_practice.initial_code,
        code_practice.expected_output.as_deref().unwrap_or(""),
        code_practice.solution,
        hints_en,
        hints_id,
        if current_difficulty == "Easy" {
            "selected"
        } else {
            ""
        },
        if current_difficulty == "Medium" {
            "selected"
        } else {
            ""
        },
        if current_difficulty == "Hard" {
            "selected"
        } else {
            ""
        },
        code_practice.category,
        topic_options,
        lesson_options,
        code_practice.points.value(),
        code_practice.id.to_string()
    );

    Ok(Html(html))
}

pub async fn update_code_practice_post_handler(
    State(state): State<AppState>,
    axum::extract::Path(code_practice_id): axum::extract::Path<String>,
    Form(form): Form<CreateCodePracticeForm>,
) -> Result<Html<String>> {
    // Parse code practice ID
    let code_practice_id = CodePracticeId::from_string(&code_practice_id)
        .map_err(|e| anyhow::anyhow!("Invalid code practice ID: {}", e))?;

    // Parse form data
    let title = LocalizedText::new(form.title_en, form.title_id);
    let description = LocalizedText::new(form.description_en, form.description_id);
    let topic_id = TopicId::from_string(form.topic_id.clone())
        .map_err(|e| anyhow::anyhow!("Invalid topic ID: {}", e))?;
    let lesson_id = LessonId::from_string(form.lesson_id.clone())
        .map_err(|e| anyhow::anyhow!("Invalid lesson ID: {}", e))?;
    let difficulty = match form.difficulty.as_str() {
        "Easy" => Difficulty::Beginner,
        "Medium" => Difficulty::Intermediate,
        "Hard" => Difficulty::Advanced,
        _ => return Err(anyhow::anyhow!("Invalid difficulty level").into()),
    };
    let points = Points::new(form.points);

    // Fetch existing code practice
    let mut code_practice = state
        .code_practice_repository
        .find_by_id(&code_practice_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Code practice not found"))?;

    // Update fields
    code_practice.title = title;
    code_practice.description = description;
    code_practice.initial_code = form.initial_code;
    code_practice.solution = form.solution;
    code_practice.difficulty = difficulty;
    code_practice.category = form.category;
    code_practice.lesson_id = lesson_id;
    code_practice.topic_id = topic_id;
    code_practice.points = points;
    code_practice.updated_at = Utc::now();

    // Update expected output
    if !form.expected_output.is_empty() {
        code_practice.set_expected_output(form.expected_output);
    } else {
        code_practice.expected_output = None;
    }

    // Update hints
    code_practice.hints.clear();
    if !form.hints_en.is_empty() || !form.hints_id.is_empty() {
        let hints_text = LocalizedText::new(form.hints_en, form.hints_id);
        code_practice.add_hint(hints_text);
    }

    // Save to database
    state
        .code_practice_repository
        .update(&code_practice)
        .await?;

    // Return success page
    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Code Practice Updated</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; text-align: center; }}
        .success {{ color: #28a745; font-size: 24px; margin-bottom: 20px; }}
        .info {{ background-color: #f8f9fa; padding: 20px; border-radius: 5px; margin: 20px 0; }}
        a {{ color: #007bff; text-decoration: none; }}
        a:hover {{ text-decoration: underline; }}
    </style>
</head>
<body>
    <div class="success">✅ Code Practice Updated Successfully!</div>
    <div class="info">
        <h3>Updated Code Practice Details:</h3>
        <p><strong>Title:</strong> {}</p>
        <p><strong>Category:</strong> {}</p>
        <p><strong>Difficulty:</strong> {}</p>
        <p><strong>Points:</strong> {}</p>
        <p><strong>Topic ID:</strong> {}</p>
        <p><strong>Lesson ID:</strong> {}</p>
    </div>
    <p><a href="/admin/code-practices">← Back to Code Practices</a></p>
    <p><a href="/admin/code-practices/{}/edit">Edit Again</a></p>
</body>
</html>"#,
        code_practice.title.get("en"),
        code_practice.category,
        match code_practice.difficulty {
            Difficulty::Beginner => "Easy",
            Difficulty::Intermediate => "Medium",
            Difficulty::Advanced => "Hard",
        },
        code_practice.points.value(),
        code_practice.topic_id.to_string(),
        code_practice.lesson_id.to_string(),
        code_practice.id.to_string()
    );

    Ok(Html(html))
}

pub async fn delete_code_practice_handler(
    State(state): State<AppState>,
    axum::extract::Path(code_practice_id): axum::extract::Path<String>,
) -> Result<Html<String>> {
    // Parse code practice ID
    let code_practice_id = CodePracticeId::from_string(&code_practice_id)
        .map_err(|e| anyhow::anyhow!("Invalid code practice ID: {}", e))?;

    // Delete from database
    state
        .code_practice_repository
        .delete(&code_practice_id)
        .await?;

    // Return success page
    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Code Practice Deleted</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; text-align: center; }}
        .success {{ color: #28a745; font-size: 24px; margin-bottom: 20px; }}
        .info {{ background-color: #f8f9fa; padding: 20px; border-radius: 5px; margin: 20px 0; }}
        a {{ color: #007bff; text-decoration: none; }}
        a:hover {{ text-decoration: underline; }}
    </style>
</head>
<body>
    <div class="success">✅ Code Practice Deleted Successfully!</div>
    <div class="info">
        <h3>Deleted Code Practice:</h3>
        <p><strong>ID:</strong> {}</p>
        <p>The code practice has been permanently removed from the database.</p>
    </div>
    <p><a href="/admin/code-practices">← Back to Code Practices</a></p>
    <p><a href="/admin/code-practices/new">Create New Code Practice</a></p>
</body>
</html>"#,
        code_practice_id.to_string()
    );

    Ok(Html(html))
}
