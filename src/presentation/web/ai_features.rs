use crate::application::state::AppState;
use crate::shared::errors::Result;
use axum::{extract::State, response::Html, Form};

#[derive(serde::Deserialize)]
pub struct GenerateQuizForm {
    pub topic: String,
    pub difficulty: String,
    pub count: u32,
}

#[derive(serde::Deserialize)]
pub struct ValidateCodeForm {
    pub code: String,
    pub expected_output: String,
}

#[derive(serde::Deserialize)]
pub struct ExplainCodeForm {
    pub code: String,
}

#[derive(serde::Deserialize)]
pub struct ImproveCodeForm {
    pub code: String,
}

pub async fn ai_features_handler(State(_state): State<AppState>) -> Result<Html<String>> {
    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>AI-Powered Features</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; }}
        .container {{ max-width: 800px; margin: 0 auto; }}
        .feature-card {{ background-color: #f8f9fa; padding: 20px; border-radius: 8px; border: 1px solid #dee2e6; margin: 20px 0; }}
        .feature-card h3 {{ margin-top: 0; color: #495057; }}
        .button {{ display: inline-block; background-color: #007bff; color: white; padding: 10px 20px; text-decoration: none; border-radius: 4px; margin: 10px 5px; }}
        .button:hover {{ background-color: #0056b3; }}
        .button.secondary {{ background-color: #6c757d; }}
        .button.secondary:hover {{ background-color: #545b62; }}
        .header {{ text-align: center; margin-bottom: 40px; }}
        .header h1 {{ color: #007bff; }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🤖 AI-Powered Features</h1>
            <p>Leverage the power of AI to enhance your Rust learning experience</p>
        </div>

        <div class="feature-card">
            <h3>📝 Quiz Generator</h3>
            <p>Generate custom quiz questions on any Rust topic with AI assistance.</p>
            <a href="/admin/ai/generate-quiz" class="button">Generate Quiz Questions</a>
        </div>

        <div class="feature-card">
            <h3>✅ Code Validator</h3>
            <p>Validate your Rust code against expected outputs using AI analysis.</p>
            <a href="/admin/ai/validate-code" class="button">Validate Code</a>
        </div>

        <div class="feature-card">
            <h3>📚 Code Explainer</h3>
            <p>Get detailed explanations of Rust code with AI-powered analysis.</p>
            <a href="/admin/ai/explain-code" class="button">Explain Code</a>
        </div>

        <div class="feature-card">
            <h3>🚀 Code Improver</h3>
            <p>Get AI suggestions for improving your Rust code quality and performance.</p>
            <a href="/admin/ai/improve-code" class="button">Improve Code</a>
        </div>

        <div style="text-align: center; margin-top: 40px;">
            <a href="/admin" class="button secondary">← Back to Dashboard</a>
        </div>
    </div>
</body>
</html>"#
    );

    Ok(Html(html))
}

pub async fn generate_quiz_form_handler(State(_state): State<AppState>) -> Result<Html<String>> {
    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Generate Quiz Questions</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; }}
        .form-container {{ max-width: 600px; margin: 0 auto; }}
        .form-group {{ margin-bottom: 20px; }}
        label {{ display: block; margin-bottom: 5px; font-weight: bold; }}
        input, select, textarea {{ width: 100%; padding: 8px; border: 1px solid #ddd; border-radius: 4px; }}
        button {{ background-color: #007bff; color: white; padding: 10px 20px; border: none; border-radius: 4px; cursor: pointer; }}
        button:hover {{ background-color: #0056b3; }}
        .back-link {{ color: #007bff; text-decoration: none; }}
        .back-link:hover {{ text-decoration: underline; }}
    </style>
</head>
<body>
    <div class="form-container">
        <h1>📝 Generate Quiz Questions</h1>
        <p>Use AI to generate custom quiz questions on any Rust topic.</p>

        <form action="/admin/ai/generate-quiz" method="POST">
            <div class="form-group">
                <label for="topic">Topic:</label>
                <input type="text" id="topic" name="topic" required placeholder="e.g., Ownership, Borrowing, Error Handling">
            </div>

            <div class="form-group">
                <label for="difficulty">Difficulty:</label>
                <select id="difficulty" name="difficulty" required>
                    <option value="">Select difficulty...</option>
                    <option value="beginner">Beginner</option>
                    <option value="intermediate">Intermediate</option>
                    <option value="advanced">Advanced</option>
                </select>
            </div>

            <div class="form-group">
                <label for="count">Number of Questions:</label>
                <input type="number" id="count" name="count" min="1" max="10" value="5" required>
            </div>

            <button type="submit">Generate Questions</button>
        </form>

        <p><a href="/admin/ai" class="back-link">← Back to AI Features</a></p>
    </div>
</body>
</html>"#
    );

    Ok(Html(html))
}

pub async fn generate_quiz_post_handler(
    State(state): State<AppState>,
    Form(form): Form<GenerateQuizForm>,
) -> Result<Html<String>> {
    // Generate quiz questions using Gemini
    let questions = state
        .gemini_service
        .generate_quiz_questions(&form.topic, &form.difficulty, form.count)
        .await?;

    let mut questions_html = String::new();
    for (i, question) in questions.iter().enumerate() {
        questions_html.push_str(&format!(
            "<div class=\"question\"><h4>Question {}</h4><pre>{}</pre></div>",
            i + 1,
            question
        ));
    }

    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Generated Quiz Questions</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; }}
        .container {{ max-width: 800px; margin: 0 auto; }}
        .success {{ color: #28a745; font-size: 24px; margin-bottom: 20px; }}
        .question {{ background-color: #f8f9fa; padding: 20px; border-radius: 8px; border: 1px solid #dee2e6; margin: 20px 0; }}
        .question h4 {{ margin-top: 0; color: #495057; }}
        .question pre {{ white-space: pre-wrap; font-family: monospace; background-color: #e9ecef; padding: 15px; border-radius: 4px; }}
        .button {{ display: inline-block; background-color: #007bff; color: white; padding: 10px 20px; text-decoration: none; border-radius: 4px; margin: 10px 5px; }}
        .button:hover {{ background-color: #0056b3; }}
        .button.secondary {{ background-color: #6c757d; }}
        .button.secondary:hover {{ background-color: #545b62; }}
    </style>
</head>
<body>
    <div class="container">
        <div class="success">✅ Quiz Questions Generated Successfully!</div>

        <h2>Generated Questions for: {} ({})</h2>
        <p><strong>Topic:</strong> {}</p>
        <p><strong>Difficulty:</strong> {}</p>
        <p><strong>Count:</strong> {}</p>

        <div class="questions">
            {}
        </div>

        <div style="text-align: center; margin-top: 40px;">
            <a href="/admin/ai/generate-quiz" class="button">Generate More Questions</a>
            <a href="/admin/ai" class="button secondary">← Back to AI Features</a>
        </div>
    </div>
</body>
</html>"#,
        form.topic, form.difficulty, form.topic, form.difficulty, form.count, questions_html
    );

    Ok(Html(html))
}

pub async fn validate_code_form_handler(State(_state): State<AppState>) -> Result<Html<String>> {
    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Validate Code</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; }}
        .form-container {{ max-width: 800px; margin: 0 auto; }}
        .form-group {{ margin-bottom: 20px; }}
        label {{ display: block; margin-bottom: 5px; font-weight: bold; }}
        input, textarea {{ width: 100%; padding: 8px; border: 1px solid #ddd; border-radius: 4px; }}
        textarea {{ height: 200px; font-family: monospace; }}
        button {{ background-color: #007bff; color: white; padding: 10px 20px; border: none; border-radius: 4px; cursor: pointer; }}
        button:hover {{ background-color: #0056b3; }}
        .back-link {{ color: #007bff; text-decoration: none; }}
        .back-link:hover {{ text-decoration: underline; }}
    </style>
</head>
<body>
    <div class="form-container">
        <h1>✅ Validate Code</h1>
        <p>Use AI to validate your Rust code against expected outputs.</p>

        <form action="/admin/ai/validate-code" method="POST">
            <div class="form-group">
                <label for="code">Rust Code:</label>
                <textarea id="code" name="code" required placeholder="fn main() {{&#10;    println!(&quot;Hello, world!&quot;);&#10;}}"></textarea>
            </div>

            <div class="form-group">
                <label for="expected_output">Expected Output:</label>
                <input type="text" id="expected_output" name="expected_output" required placeholder="e.g., Hello, world!">
            </div>

            <button type="submit">Validate Code</button>
        </form>

        <p><a href="/admin/ai" class="back-link">← Back to AI Features</a></p>
    </div>
</body>
</html>"#
    );

    Ok(Html(html))
}

pub async fn validate_code_post_handler(
    State(state): State<AppState>,
    Form(form): Form<ValidateCodeForm>,
) -> Result<Html<String>> {
    // Validate code using Gemini
    let is_valid = state
        .gemini_service
        .validate_code(&form.code, &form.expected_output)
        .await?;

    let result_class = if is_valid { "success" } else { "error" };
    let result_text = if is_valid {
        "✅ Code is Valid"
    } else {
        "❌ Code is Invalid"
    };

    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Code Validation Result</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; }}
        .container {{ max-width: 800px; margin: 0 auto; }}
        .result {{ font-size: 24px; margin-bottom: 20px; }}
        .success {{ color: #28a745; }}
        .error {{ color: #dc3545; }}
        .code-block {{ background-color: #f8f9fa; padding: 20px; border-radius: 8px; border: 1px solid #dee2e6; margin: 20px 0; }}
        .code-block pre {{ white-space: pre-wrap; font-family: monospace; background-color: #e9ecef; padding: 15px; border-radius: 4px; }}
        .button {{ display: inline-block; background-color: #007bff; color: white; padding: 10px 20px; text-decoration: none; border-radius: 4px; margin: 10px 5px; }}
        .button:hover {{ background-color: #0056b3; }}
        .button.secondary {{ background-color: #6c757d; }}
        .button.secondary:hover {{ background-color: #545b62; }}
    </style>
</head>
<body>
    <div class="container">
        <div class="result {}">{}</div>

        <div class="code-block">
            <h3>Your Code:</h3>
            <pre>{}</pre>
        </div>

        <div class="code-block">
            <h3>Expected Output:</h3>
            <pre>{}</pre>
        </div>

        <div style="text-align: center; margin-top: 40px;">
            <a href="/admin/ai/validate-code" class="button">Validate More Code</a>
            <a href="/admin/ai" class="button secondary">← Back to AI Features</a>
        </div>
    </div>
</body>
</html>"#,
        result_class, result_text, form.code, form.expected_output
    );

    Ok(Html(html))
}

pub async fn explain_code_form_handler(State(_state): State<AppState>) -> Result<Html<String>> {
    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Explain Code</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; }}
        .form-container {{ max-width: 800px; margin: 0 auto; }}
        .form-group {{ margin-bottom: 20px; }}
        label {{ display: block; margin-bottom: 5px; font-weight: bold; }}
        textarea {{ width: 100%; padding: 8px; border: 1px solid #ddd; border-radius: 4px; height: 300px; font-family: monospace; }}
        button {{ background-color: #007bff; color: white; padding: 10px 20px; border: none; border-radius: 4px; cursor: pointer; }}
        button:hover {{ background-color: #0056b3; }}
        .back-link {{ color: #007bff; text-decoration: none; }}
        .back-link:hover {{ text-decoration: underline; }}
    </style>
</head>
<body>
    <div class="form-container">
        <h1>📚 Explain Code</h1>
        <p>Get detailed explanations of your Rust code with AI-powered analysis.</p>

        <form action="/admin/ai/explain-code" method="POST">
            <div class="form-group">
                <label for="code">Rust Code to Explain:</label>
                <textarea id="code" name="code" required placeholder="fn main() {{&#10;    let mut vec = Vec::new();&#10;    vec.push(1);&#10;    vec.push(2);&#10;    println!(&quot;{{:?}}&quot;, vec);&#10;}}"></textarea>
            </div>

            <button type="submit">Explain Code</button>
        </form>

        <p><a href="/admin/ai" class="back-link">← Back to AI Features</a></p>
    </div>
</body>
</html>"#
    );

    Ok(Html(html))
}

pub async fn explain_code_post_handler(
    State(state): State<AppState>,
    Form(form): Form<ExplainCodeForm>,
) -> Result<Html<String>> {
    // Explain code using Gemini
    let explanation = state
        .gemini_service
        .generate_code_explanation(&form.code)
        .await?;

    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Code Explanation</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; }}
        .container {{ max-width: 800px; margin: 0 auto; }}
        .success {{ color: #28a745; font-size: 24px; margin-bottom: 20px; }}
        .code-block {{ background-color: #f8f9fa; padding: 20px; border-radius: 8px; border: 1px solid #dee2e6; margin: 20px 0; }}
        .code-block pre {{ white-space: pre-wrap; font-family: monospace; background-color: #e9ecef; padding: 15px; border-radius: 4px; }}
        .explanation {{ background-color: #f8f9fa; padding: 20px; border-radius: 8px; border: 1px solid #dee2e6; margin: 20px 0; }}
        .explanation pre {{ white-space: pre-wrap; font-family: Arial, sans-serif; }}
        .button {{ display: inline-block; background-color: #007bff; color: white; padding: 10px 20px; text-decoration: none; border-radius: 4px; margin: 10px 5px; }}
        .button:hover {{ background-color: #0056b3; }}
        .button.secondary {{ background-color: #6c757d; }}
        .button.secondary:hover {{ background-color: #545b62; }}
    </style>
</head>
<body>
    <div class="container">
        <div class="success">✅ Code Explanation Generated!</div>

        <div class="code-block">
            <h3>Your Code:</h3>
            <pre>{}</pre>
        </div>

        <div class="explanation">
            <h3>AI Explanation:</h3>
            <pre>{}</pre>
        </div>

        <div style="text-align: center; margin-top: 40px;">
            <a href="/admin/ai/explain-code" class="button">Explain More Code</a>
            <a href="/admin/ai" class="button secondary">← Back to AI Features</a>
        </div>
    </div>
</body>
</html>"#,
        form.code, explanation
    );

    Ok(Html(html))
}

pub async fn improve_code_form_handler(State(_state): State<AppState>) -> Result<Html<String>> {
    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Improve Code</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; }}
        .form-container {{ max-width: 800px; margin: 0 auto; }}
        .form-group {{ margin-bottom: 20px; }}
        label {{ display: block; margin-bottom: 5px; font-weight: bold; }}
        textarea {{ width: 100%; padding: 8px; border: 1px solid #ddd; border-radius: 4px; height: 300px; font-family: monospace; }}
        button {{ background-color: #007bff; color: white; padding: 10px 20px; border: none; border-radius: 4px; cursor: pointer; }}
        button:hover {{ background-color: #0056b3; }}
        .back-link {{ color: #007bff; text-decoration: none; }}
        .back-link:hover {{ text-decoration: underline; }}
    </style>
</head>
<body>
    <div class="form-container">
        <h1>🚀 Improve Code</h1>
        <p>Get AI suggestions for improving your Rust code quality and performance.</p>

        <form action="/admin/ai/improve-code" method="POST">
            <div class="form-group">
                <label for="code">Rust Code to Improve:</label>
                <textarea id="code" name="code" required placeholder="fn main() {{&#10;    let mut numbers = Vec::new();&#10;    for i in 0..1000 {{&#10;        numbers.push(i);&#10;    }}&#10;    println!(&quot;{{:?}}&quot;, numbers);&#10;}}"></textarea>
            </div>

            <button type="submit">Get Improvements</button>
        </form>

        <p><a href="/admin/ai" class="back-link">← Back to AI Features</a></p>
    </div>
</body>
</html>"#
    );

    Ok(Html(html))
}

pub async fn improve_code_post_handler(
    State(state): State<AppState>,
    Form(form): Form<ImproveCodeForm>,
) -> Result<Html<String>> {
    // Get code improvements using Gemini
    let improvements = state
        .gemini_service
        .suggest_code_improvements(&form.code)
        .await?;

    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Code Improvements</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; }}
        .container {{ max-width: 800px; margin: 0 auto; }}
        .success {{ color: #28a745; font-size: 24px; margin-bottom: 20px; }}
        .code-block {{ background-color: #f8f9fa; padding: 20px; border-radius: 8px; border: 1px solid #dee2e6; margin: 20px 0; }}
        .code-block pre {{ white-space: pre-wrap; font-family: monospace; background-color: #e9ecef; padding: 15px; border-radius: 4px; }}
        .improvements {{ background-color: #f8f9fa; padding: 20px; border-radius: 8px; border: 1px solid #dee2e6; margin: 20px 0; }}
        .improvements pre {{ white-space: pre-wrap; font-family: Arial, sans-serif; }}
        .button {{ display: inline-block; background-color: #007bff; color: white; padding: 10px 20px; text-decoration: none; border-radius: 4px; margin: 10px 5px; }}
        .button:hover {{ background-color: #0056b3; }}
        .button.secondary {{ background-color: #6c757d; }}
        .button.secondary:hover {{ background-color: #545b62; }}
    </style>
</head>
<body>
    <div class="container">
        <div class="success">✅ Code Improvements Generated!</div>

        <div class="code-block">
            <h3>Your Original Code:</h3>
            <pre>{}</pre>
        </div>

        <div class="improvements">
            <h3>AI Suggestions for Improvement:</h3>
            <pre>{}</pre>
        </div>

        <div style="text-align: center; margin-top: 40px;">
            <a href="/admin/ai/improve-code" class="button">Improve More Code</a>
            <a href="/admin/ai" class="button secondary">← Back to AI Features</a>
        </div>
    </div>
</body>
</html>"#,
        form.code, improvements
    );

    Ok(Html(html))
}
