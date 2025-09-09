use crate::application::state::AppState;
use crate::domain::entities::{Question, QuestionType};
use crate::domain::value_objects::{Difficulty, LocalizedText, Points, QuestionId, TopicId};
use crate::shared::errors::Result;
use axum::{extract::State, response::Html, Form};

#[derive(serde::Deserialize)]
pub struct CreateQuestionForm {
    pub prompt_en: String,
    pub prompt_id: String,
    pub topic_id: String,
    pub difficulty: String,
    pub points: u32,
    pub question_type: String,
    pub explanation_en: String,
    pub explanation_id: String,
}

pub async fn questions_handler(State(state): State<AppState>) -> Result<Html<String>> {
    // Fetch real questions from database
    let questions = state.question_repository.list(100, 0).await?;

    let mut questions_html = String::new();
    for question in &questions {
        let explanation_text = match &question.explanation {
            Some(exp) => exp.get("en"),
            None => "No explanation available",
        };

        questions_html.push_str(&format!(
            "<div><h3>{}</h3><p>{}</p><p>Topic ID: {}</p><p>Difficulty: {:?}</p><p>Points: {}</p></div>",
            question.prompt.get("en"),
            explanation_text,
            question.topic_id.to_string(),
            question.difficulty,
            question.points.value()
        ));
    }

    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head><title>Questions Management</title></head>
<body>
    <h1>Questions Management</h1>
    <p>Total Questions: {}</p>
    {}
</body>
</html>"#,
        questions.len(),
        questions_html
    );

    Ok(Html(html))
}

pub async fn create_question_handler(State(state): State<AppState>) -> Result<Html<String>> {
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
    <title>Create New Question</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; }}
        .form-group {{ margin-bottom: 20px; }}
        label {{ display: block; margin-bottom: 5px; font-weight: bold; }}
        input, textarea, select {{ width: 100%; padding: 8px; border: 1px solid #ddd; border-radius: 4px; }}
        button {{ background-color: #007bff; color: white; padding: 10px 20px; border: none; border-radius: 4px; cursor: pointer; }}
        button:hover {{ background-color: #0056b3; }}
        .question-type {{ display: none; }}
        .question-type.active {{ display: block; }}
    </style>
</head>
<body>
    <h1>Create New Question</h1>
    <form action="/admin/questions" method="POST">
        <div class="form-group">
            <label for="prompt_en">Question Prompt (English):</label>
            <textarea id="prompt_en" name="prompt_en" rows="3" required></textarea>
        </div>
        <div class="form-group">
            <label for="prompt_id">Question Prompt (Indonesian):</label>
            <textarea id="prompt_id" name="prompt_id" rows="3" required></textarea>
        </div>
        <div class="form-group">
            <label for="topic_id">Topic:</label>
            <select id="topic_id" name="topic_id" required>
                <option value="">Select a topic...</option>
                {topic_options}
            </select>
        </div>
        <div class="form-group">
            <label for="difficulty">Difficulty:</label>
            <select id="difficulty" name="difficulty" required>
                <option value="Easy">Easy</option>
                <option value="Medium">Medium</option>
                <option value="Hard">Hard</option>
            </select>
        </div>
        <div class="form-group">
            <label for="points">Points:</label>
            <input type="number" id="points" name="points" min="1" value="10" required>
        </div>
        <div class="form-group">
            <label for="question_type">Question Type:</label>
            <select id="question_type" name="question_type" required onchange="toggleQuestionType()">
                <option value="">Select question type...</option>
                <option value="MultipleChoice">Multiple Choice</option>
                <option value="TrueFalse">True/False</option>
                <option value="FillInBlank">Fill in the Blank</option>
                <option value="CodeOutputPrediction">Code Output Prediction</option>
                <option value="CodeFix">Code Fix</option>
            </select>
        </div>
        <div class="form-group">
            <label for="explanation_en">Explanation (English):</label>
            <textarea id="explanation_en" name="explanation_en" rows="3"></textarea>
        </div>
        <div class="form-group">
            <label for="explanation_id">Explanation (Indonesian):</label>
            <textarea id="explanation_id" name="explanation_id" rows="3"></textarea>
        </div>
        <button type="submit">Create Question</button>
    </form>
    <p><a href="/admin/questions">← Back to Questions</a></p>

    <script>
        function toggleQuestionType() {{
            // This would show/hide additional fields based on question type
            // For now, just a placeholder
        }}
    </script>
</body>
</html>"#
    );

    Ok(Html(html))
}

pub async fn create_question_post_handler(
    State(state): State<AppState>,
    Form(form): Form<CreateQuestionForm>,
) -> Result<Html<String>> {
    // Parse topic_id
    let topic_id = TopicId::from_string(form.topic_id.clone())
        .map_err(|e| anyhow::anyhow!("Invalid topic ID: {}", e))?;

    // Parse difficulty
    let difficulty = match form.difficulty.as_str() {
        "Easy" => Difficulty::Beginner,
        "Medium" => Difficulty::Intermediate,
        "Hard" => Difficulty::Advanced,
        _ => return Err(anyhow::anyhow!("Invalid difficulty level"))?,
    };

    // Create LocalizedText objects
    let prompt = LocalizedText::new(form.prompt_en, form.prompt_id);
    let explanation = if !form.explanation_en.is_empty() || !form.explanation_id.is_empty() {
        Some(LocalizedText::new(form.explanation_en, form.explanation_id))
    } else {
        None
    };

    // Create question type (simplified - just TrueFalse for now)
    let question_type = match form.question_type.as_str() {
        "TrueFalse" => QuestionType::TrueFalse { answer: true }, // Default to true
        _ => QuestionType::TrueFalse { answer: true },           // Default fallback
    };

    // Create new question
    let question = Question::new(
        prompt,
        topic_id,
        difficulty,
        Points::new(form.points),
        question_type,
    );

    // Save to database
    state.question_repository.create(&question).await?;

    // Return success page
    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Question Created Successfully</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; text-align: center; }}
        .success {{ color: #28a745; font-size: 24px; margin-bottom: 20px; }}
        .info {{ background-color: #f8f9fa; padding: 20px; border-radius: 5px; margin: 20px 0; }}
        a {{ color: #007bff; text-decoration: none; }}
        a:hover {{ text-decoration: underline; }}
    </style>
</head>
<body>
    <div class="success">✅ Question Created Successfully!</div>
    <div class="info">
        <h3>Question Details:</h3>
        <p><strong>Prompt:</strong> {}</p>
        <p><strong>Topic ID:</strong> {}</p>
        <p><strong>Difficulty:</strong> {}</p>
        <p><strong>Points:</strong> {}</p>
        <p><strong>Question ID:</strong> {}</p>
    </div>
    <p><a href="/admin/questions">← Back to Questions</a></p>
    <p><a href="/admin/questions/new">Create Another Question</a></p>
</body>
</html>"#,
        question.prompt.get("en"),
        question.topic_id.to_string(),
        form.difficulty,
        question.points.value(),
        question.id.to_string()
    );

    Ok(Html(html))
}

pub async fn edit_question_handler(
    State(state): State<AppState>,
    axum::extract::Path(question_id): axum::extract::Path<String>,
) -> Result<Html<String>> {
    // Parse question_id
    let question_id = QuestionId::from_string(&question_id)
        .map_err(|e| anyhow::anyhow!("Invalid question ID: {}", e))?;

    // Fetch question from database
    let question = state
        .question_repository
        .find_by_id(&question_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Question not found"))?;

    // Fetch topics for dropdown
    let topics = state.topic_repository.list(100, 0).await?;

    let mut topic_options = String::new();
    for topic in &topics {
        let selected = if topic.id == question.topic_id {
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

    // Get current difficulty value
    let current_difficulty = match question.difficulty {
        Difficulty::Beginner => "Easy",
        Difficulty::Intermediate => "Medium",
        Difficulty::Advanced => "Hard",
    };

    // Get current question type
    let current_question_type = match &question.question_type {
        QuestionType::TrueFalse { .. } => "TrueFalse",
        QuestionType::MultipleChoice { .. } => "MultipleChoice",
        QuestionType::FillInBlank { .. } => "FillInBlank",
        QuestionType::CodeOutputPrediction { .. } => "CodeOutputPrediction",
        QuestionType::CodeFix { .. } => "CodeFix",
        QuestionType::CodeWriting { .. } => "CodeWriting",
    };

    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Edit Question</title>
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
    <h1>Edit Question</h1>
    <form action="/admin/questions/{}/update" method="POST">
        <div class="form-group">
            <label for="prompt_en">Prompt (English):</label>
            <textarea id="prompt_en" name="prompt_en" rows="4" required>{}</textarea>
        </div>
        <div class="form-group">
            <label for="prompt_id">Prompt (Indonesian):</label>
            <textarea id="prompt_id" name="prompt_id" rows="4" required>{}</textarea>
        </div>
        <div class="form-group">
            <label for="topic_id">Topic:</label>
            <select id="topic_id" name="topic_id" required>
                <option value="">Select a topic...</option>
                {}
            </select>
        </div>
        <div class="form-group">
            <label for="difficulty">Difficulty:</label>
            <select id="difficulty" name="difficulty" required>
                <option value="Easy" {}>Easy</option>
                <option value="Medium" {}>Medium</option>
                <option value="Hard" {}>Hard</option>
            </select>
        </div>
        <div class="form-group">
            <label for="points">Points:</label>
            <input type="number" id="points" name="points" min="1" value="{}" required>
        </div>
        <div class="form-group">
            <label for="question_type">Question Type:</label>
            <select id="question_type" name="question_type" required>
                <option value="TrueFalse" {}>True/False</option>
                <option value="MultipleChoice" {}>Multiple Choice</option>
                <option value="FillInBlank" {}>Fill in Blank</option>
                <option value="CodeOutputPrediction" {}>Code Output Prediction</option>
                <option value="CodeFix" {}>Code Fix</option>
                <option value="CodeWriting" {}>Code Writing</option>
            </select>
        </div>
        <div class="form-group">
            <label for="explanation_en">Explanation (English):</label>
            <textarea id="explanation_en" name="explanation_en" rows="3">{}</textarea>
        </div>
        <div class="form-group">
            <label for="explanation_id">Explanation (Indonesian):</label>
            <textarea id="explanation_id" name="explanation_id" rows="3">{}</textarea>
        </div>
        <button type="submit">Update Question</button>
        <button type="button" class="delete-btn" onclick="confirmDelete()">Delete Question</button>
    </form>
    <p><a href="/admin/questions">← Back to Questions</a></p>

    <script>
        function confirmDelete() {{
            if (confirm('Are you sure you want to delete this question? This action cannot be undone.')) {{
                window.location.href = '/admin/questions/{}/delete';
            }}
        }}
    </script>
</body>
</html>"#,
        question_id.to_string(),
        question.prompt.get("en"),
        question.prompt.get("id"),
        topic_options,
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
        question.points.value(),
        if current_question_type == "TrueFalse" {
            "selected"
        } else {
            ""
        },
        if current_question_type == "MultipleChoice" {
            "selected"
        } else {
            ""
        },
        if current_question_type == "FillInBlank" {
            "selected"
        } else {
            ""
        },
        if current_question_type == "CodeOutputPrediction" {
            "selected"
        } else {
            ""
        },
        if current_question_type == "CodeFix" {
            "selected"
        } else {
            ""
        },
        if current_question_type == "CodeWriting" {
            "selected"
        } else {
            ""
        },
        match &question.explanation {
            Some(exp) => exp.get("en"),
            None => "",
        },
        match &question.explanation {
            Some(exp) => exp.get("id"),
            None => "",
        },
        question_id.to_string()
    );

    Ok(Html(html))
}

pub async fn update_question_post_handler(
    State(state): State<AppState>,
    axum::extract::Path(question_id): axum::extract::Path<String>,
    Form(form): Form<CreateQuestionForm>,
) -> Result<Html<String>> {
    // Parse question_id
    let question_id = QuestionId::from_string(&question_id)
        .map_err(|e| anyhow::anyhow!("Invalid question ID: {}", e))?;

    // Parse topic_id
    let topic_id = TopicId::from_string(form.topic_id.clone())
        .map_err(|e| anyhow::anyhow!("Invalid topic ID: {}", e))?;

    // Parse difficulty
    let difficulty = match form.difficulty.as_str() {
        "Easy" => Difficulty::Beginner,
        "Medium" => Difficulty::Intermediate,
        "Hard" => Difficulty::Advanced,
        _ => return Err(anyhow::anyhow!("Invalid difficulty level"))?,
    };

    // Create LocalizedText objects
    let prompt = LocalizedText::new(form.prompt_en, form.prompt_id);
    let explanation = if !form.explanation_en.is_empty() || !form.explanation_id.is_empty() {
        Some(LocalizedText::new(form.explanation_en, form.explanation_id))
    } else {
        None
    };

    // Create question type (simplified - just TrueFalse for now)
    let question_type = match form.question_type.as_str() {
        "TrueFalse" => QuestionType::TrueFalse { answer: true }, // Default to true
        _ => QuestionType::TrueFalse { answer: true },           // Default fallback
    };

    // Create updated question
    let mut question = state
        .question_repository
        .find_by_id(&question_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Question not found"))?;

    // Update fields
    question.prompt = prompt;
    question.topic_id = topic_id;
    question.difficulty = difficulty;
    question.points = Points::new(form.points);
    question.question_type = question_type;
    question.explanation = explanation;

    // Save to database
    state.question_repository.update(&question).await?;

    // Return success page
    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Question Updated Successfully</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; text-align: center; }}
        .success {{ color: #28a745; font-size: 24px; margin-bottom: 20px; }}
        .info {{ background-color: #f8f9fa; padding: 20px; border-radius: 5px; margin: 20px 0; }}
        a {{ color: #007bff; text-decoration: none; }}
        a:hover {{ text-decoration: underline; }}
    </style>
</head>
<body>
    <div class="success">✅ Question Updated Successfully!</div>
    <div class="info">
        <h3>Updated Question Details:</h3>
        <p><strong>Prompt:</strong> {}</p>
        <p><strong>Topic ID:</strong> {}</p>
        <p><strong>Difficulty:</strong> {}</p>
        <p><strong>Points:</strong> {}</p>
        <p><strong>Question ID:</strong> {}</p>
    </div>
    <p><a href="/admin/questions">← Back to Questions</a></p>
    <p><a href="/admin/questions/{}/edit">Edit Again</a></p>
</body>
</html>"#,
        question.prompt.get("en"),
        question.topic_id.to_string(),
        form.difficulty, // Displaying original form difficulty string
        question.points.value(),
        question.id.to_string(),
        question.id.to_string()
    );

    Ok(Html(html))
}

pub async fn delete_question_handler(
    State(state): State<AppState>,
    axum::extract::Path(question_id): axum::extract::Path<String>,
) -> Result<Html<String>> {
    // Parse question_id
    let question_id = QuestionId::from_string(&question_id)
        .map_err(|e| anyhow::anyhow!("Invalid question ID: {}", e))?;

    // Delete from database
    state.question_repository.delete(&question_id).await?;

    // Return success page
    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Question Deleted Successfully</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; text-align: center; }}
        .success {{ color: #dc3545; font-size: 24px; margin-bottom: 20px; }}
        .info {{ background-color: #f8f9fa; padding: 20px; border-radius: 5px; margin: 20px 0; }}
        a {{ color: #007bff; text-decoration: none; }}
        a:hover {{ text-decoration: underline; }}
    </style>
</head>
<body>
    <div class="success">🗑️ Question Deleted Successfully!</div>
    <div class="info">
        <h3>Deleted Question ID:</h3>
        <p><strong>Question ID:</strong> {}</p>
    </div>
    <p><a href="/admin/questions">← Back to Questions</a></p>
    <p><a href="/admin/questions/new">Create New Question</a></p>
</body>
</html>"#,
        question_id.to_string()
    );

    Ok(Html(html))
}
