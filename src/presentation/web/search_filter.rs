use crate::application::state::AppState;
use crate::shared::errors::Result;
use axum::{
    extract::{Query, State},
    response::Html,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SearchParams {
    pub q: Option<String>,          // Search query
    pub category: Option<String>,   // Filter by category
    pub difficulty: Option<String>, // Filter by difficulty
    pub page: Option<u32>,          // Page number
    pub limit: Option<u32>,         // Items per page
}

pub async fn search_topics_handler(
    State(state): State<AppState>,
    Query(params): Query<SearchParams>,
) -> Result<Html<String>> {
    let search_query = params.q.unwrap_or_default();
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(20);
    let offset = (page - 1) * limit;

    // Fetch topics with search and pagination
    let topics = state.topic_repository.list(limit, offset).await?;

    // Filter topics based on search query
    let filtered_topics: Vec<_> = if search_query.is_empty() {
        topics
    } else {
        topics
            .into_iter()
            .filter(|topic| {
                topic
                    .title
                    .get("en")
                    .to_lowercase()
                    .contains(&search_query.to_lowercase())
                    || topic
                        .title
                        .get("id")
                        .to_lowercase()
                        .contains(&search_query.to_lowercase())
                    || topic
                        .description
                        .get("en")
                        .to_lowercase()
                        .contains(&search_query.to_lowercase())
                    || topic
                        .description
                        .get("id")
                        .to_lowercase()
                        .contains(&search_query.to_lowercase())
            })
            .collect()
    };

    let mut topics_html = String::new();
    for topic in &filtered_topics {
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
    <title>Search Topics</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; }}
        .search-container {{ margin-bottom: 30px; padding: 20px; background-color: #f8f9fa; border-radius: 8px; }}
        .search-form {{ display: flex; gap: 10px; align-items: center; }}
        .search-input {{ flex: 1; padding: 10px; border: 1px solid #ddd; border-radius: 4px; }}
        .search-btn {{ padding: 10px 20px; background-color: #007bff; color: white; border: none; border-radius: 4px; cursor: pointer; }}
        .search-btn:hover {{ background-color: #0056b3; }}
        .results-info {{ margin-bottom: 20px; color: #6c757d; }}
        .btn {{ padding: 8px 16px; text-decoration: none; border-radius: 4px; font-size: 14px; }}
        .btn-primary {{ background-color: #007bff; color: white; }}
        .btn-danger {{ background-color: #dc3545; color: white; }}
        .btn-sm {{ padding: 6px 12px; font-size: 12px; }}
        table {{ width: 100%; border-collapse: collapse; margin-top: 20px; }}
        th, td {{ border: 1px solid #ddd; padding: 12px; text-align: left; }}
        th {{ background-color: #f8f9fa; font-weight: bold; }}
        tr:nth-child(even) {{ background-color: #f9f9f9; }}
        .pagination {{ margin-top: 20px; text-align: center; }}
        .pagination a {{ padding: 8px 16px; margin: 0 4px; text-decoration: none; border: 1px solid #ddd; border-radius: 4px; }}
        .pagination a:hover {{ background-color: #f8f9fa; }}
        .pagination .current {{ background-color: #007bff; color: white; }}
    </style>
</head>
<body>
    <h1>🔍 Search Topics</h1>

    <div class="search-container">
        <form class="search-form" method="GET" action="/admin/search/topics">
            <input type="text" name="q" class="search-input" placeholder="Search topics..." value="{}">
            <button type="submit" class="search-btn">Search</button>
        </form>
    </div>

    <div class="results-info">
        Found {} topics
        {}
    </div>

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
        <a href="/admin/topics" class="btn btn-primary">← Back to Topics</a>
    </div>
</body>
</html>"#,
        search_query,
        filtered_topics.len(),
        if !search_query.is_empty() {
            format!("for '{search_query}'")
        } else {
            String::new()
        },
        topics_html,
        generate_pagination(page, limit, filtered_topics.len())
    );

    Ok(Html(html))
}

pub async fn search_lessons_handler(
    State(state): State<AppState>,
    Query(params): Query<SearchParams>,
) -> Result<Html<String>> {
    let search_query = params.q.unwrap_or_default();
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(20);
    let offset = (page - 1) * limit;

    // Fetch lessons with search and pagination
    let lessons = state.lesson_repository.list(limit, offset).await?;

    // Filter lessons based on search query
    let filtered_lessons: Vec<_> = if search_query.is_empty() {
        lessons
    } else {
        lessons
            .into_iter()
            .filter(|lesson| {
                lesson
                    .title
                    .get("en")
                    .to_lowercase()
                    .contains(&search_query.to_lowercase())
                    || lesson
                        .title
                        .get("id")
                        .to_lowercase()
                        .contains(&search_query.to_lowercase())
                    || lesson
                        .summary
                        .get("en")
                        .to_lowercase()
                        .contains(&search_query.to_lowercase())
                    || lesson
                        .summary
                        .get("id")
                        .to_lowercase()
                        .contains(&search_query.to_lowercase())
            })
            .collect()
    };

    let mut lessons_html = String::new();
    for lesson in &filtered_lessons {
        lessons_html.push_str(&format!(
            r#"<tr>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td>
                    <a href="/admin/lessons/{}/edit" class="btn btn-sm btn-primary">Edit</a>
                    <a href="/admin/lessons/{}/delete" class="btn btn-sm btn-danger" onclick="return confirm('Are you sure?')">Delete</a>
                </td>
            </tr>"#,
            lesson.title.get("en"),
            lesson.summary.get("en"),
            lesson.order,
            lesson.topic_id.to_string(),
            lesson.id.to_string(),
            lesson.id.to_string()
        ));
    }

    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Search Lessons</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; }}
        .search-container {{ margin-bottom: 30px; padding: 20px; background-color: #f8f9fa; border-radius: 8px; }}
        .search-form {{ display: flex; gap: 10px; align-items: center; }}
        .search-input {{ flex: 1; padding: 10px; border: 1px solid #ddd; border-radius: 4px; }}
        .search-btn {{ padding: 10px 20px; background-color: #007bff; color: white; border: none; border-radius: 4px; cursor: pointer; }}
        .search-btn:hover {{ background-color: #0056b3; }}
        .results-info {{ margin-bottom: 20px; color: #6c757d; }}
        .btn {{ padding: 8px 16px; text-decoration: none; border-radius: 4px; font-size: 14px; }}
        .btn-primary {{ background-color: #007bff; color: white; }}
        .btn-danger {{ background-color: #dc3545; color: white; }}
        .btn-sm {{ padding: 6px 12px; font-size: 12px; }}
        table {{ width: 100%; border-collapse: collapse; margin-top: 20px; }}
        th, td {{ border: 1px solid #ddd; padding: 12px; text-align: left; }}
        th {{ background-color: #f8f9fa; font-weight: bold; }}
        tr:nth-child(even) {{ background-color: #f9f9f9; }}
        .pagination {{ margin-top: 20px; text-align: center; }}
        .pagination a {{ padding: 8px 16px; margin: 0 4px; text-decoration: none; border: 1px solid #ddd; border-radius: 4px; }}
        .pagination a:hover {{ background-color: #f8f9fa; }}
        .pagination .current {{ background-color: #007bff; color: white; }}
    </style>
</head>
<body>
    <h1>🔍 Search Lessons</h1>

    <div class="search-container">
        <form class="search-form" method="GET" action="/admin/search/lessons">
            <input type="text" name="q" class="search-input" placeholder="Search lessons..." value="{}">
            <button type="submit" class="search-btn">Search</button>
        </form>
    </div>

    <div class="results-info">
        Found {} lessons
        {}
    </div>

    <table>
        <thead>
            <tr>
                <th>Title</th>
                <th>Summary</th>
                <th>Order</th>
                <th>Topic ID</th>
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
        <a href="/admin/lessons" class="btn btn-primary">← Back to Lessons</a>
    </div>
</body>
</html>"#,
        search_query,
        filtered_lessons.len(),
        if !search_query.is_empty() {
            format!("for '{search_query}'")
        } else {
            String::new()
        },
        lessons_html,
        generate_pagination(page, limit, filtered_lessons.len())
    );

    Ok(Html(html))
}

pub async fn search_questions_handler(
    State(state): State<AppState>,
    Query(params): Query<SearchParams>,
) -> Result<Html<String>> {
    let search_query = params.q.unwrap_or_default();
    let difficulty = params.difficulty.unwrap_or_default();
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(20);
    let offset = (page - 1) * limit;

    // Fetch questions with search and pagination
    let questions = state.question_repository.list(limit, offset).await?;

    // Filter questions based on search query and difficulty
    let filtered_questions: Vec<_> = questions
        .into_iter()
        .filter(|question| {
            let matches_search = search_query.is_empty()
                || question
                    .prompt
                    .get("en")
                    .to_lowercase()
                    .contains(&search_query.to_lowercase())
                || question
                    .prompt
                    .get("id")
                    .to_lowercase()
                    .contains(&search_query.to_lowercase());

            let matches_difficulty = difficulty.is_empty()
                || match difficulty.as_str() {
                    "Easy" => matches!(
                        question.difficulty,
                        crate::domain::value_objects::Difficulty::Beginner
                    ),
                    "Medium" => matches!(
                        question.difficulty,
                        crate::domain::value_objects::Difficulty::Intermediate
                    ),
                    "Hard" => matches!(
                        question.difficulty,
                        crate::domain::value_objects::Difficulty::Advanced
                    ),
                    _ => true,
                };

            matches_search && matches_difficulty
        })
        .collect();

    let mut questions_html = String::new();
    for question in &filtered_questions {
        questions_html.push_str(&format!(
            r#"<tr>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td>
                    <a href="/admin/questions/{}/edit" class="btn btn-sm btn-primary">Edit</a>
                    <a href="/admin/questions/{}/delete" class="btn btn-sm btn-danger" onclick="return confirm('Are you sure?')">Delete</a>
                </td>
            </tr>"#,
            question.prompt.get("en"),
            match question.difficulty {
                crate::domain::value_objects::Difficulty::Beginner => "Easy",
                crate::domain::value_objects::Difficulty::Intermediate => "Medium",
                crate::domain::value_objects::Difficulty::Advanced => "Hard",
            },
            question.points.value(),
            question.topic_id.to_string(),
            question.id.to_string(),
            question.id.to_string()
        ));
    }

    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Search Questions</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; }}
        .search-container {{ margin-bottom: 30px; padding: 20px; background-color: #f8f9fa; border-radius: 8px; }}
        .search-form {{ display: flex; gap: 10px; align-items: center; }}
        .search-input {{ flex: 1; padding: 10px; border: 1px solid #ddd; border-radius: 4px; }}
        .search-select {{ padding: 10px; border: 1px solid #ddd; border-radius: 4px; }}
        .search-btn {{ padding: 10px 20px; background-color: #007bff; color: white; border: none; border-radius: 4px; cursor: pointer; }}
        .search-btn:hover {{ background-color: #0056b3; }}
        .results-info {{ margin-bottom: 20px; color: #6c757d; }}
        .btn {{ padding: 8px 16px; text-decoration: none; border-radius: 4px; font-size: 14px; }}
        .btn-primary {{ background-color: #007bff; color: white; }}
        .btn-danger {{ background-color: #dc3545; color: white; }}
        .btn-sm {{ padding: 6px 12px; font-size: 12px; }}
        table {{ width: 100%; border-collapse: collapse; margin-top: 20px; }}
        th, td {{ border: 1px solid #ddd; padding: 12px; text-align: left; }}
        th {{ background-color: #f8f9fa; font-weight: bold; }}
        tr:nth-child(even) {{ background-color: #f9f9f9; }}
        .pagination {{ margin-top: 20px; text-align: center; }}
        .pagination a {{ padding: 8px 16px; margin: 0 4px; text-decoration: none; border: 1px solid #ddd; border-radius: 4px; }}
        .pagination a:hover {{ background-color: #f8f9fa; }}
        .pagination .current {{ background-color: #007bff; color: white; }}
    </style>
</head>
<body>
    <h1>🔍 Search Questions</h1>

    <div class="search-container">
        <form class="search-form" method="GET" action="/admin/search/questions">
            <input type="text" name="q" class="search-input" placeholder="Search questions..." value="{}">
            <select name="difficulty" class="search-select">
                <option value="">All Difficulties</option>
                <option value="Easy" {}>Easy</option>
                <option value="Medium" {}>Medium</option>
                <option value="Hard" {}>Hard</option>
            </select>
            <button type="submit" class="search-btn">Search</button>
        </form>
    </div>

    <div class="results-info">
        Found {} questions
        {}
    </div>

    <table>
        <thead>
            <tr>
                <th>Prompt</th>
                <th>Difficulty</th>
                <th>Points</th>
                <th>Topic ID</th>
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
        <a href="/admin/questions" class="btn btn-primary">← Back to Questions</a>
    </div>
</body>
</html>"#,
        search_query,
        if difficulty == "Easy" { "selected" } else { "" },
        if difficulty == "Medium" {
            "selected"
        } else {
            ""
        },
        if difficulty == "Hard" { "selected" } else { "" },
        filtered_questions.len(),
        if !search_query.is_empty() || !difficulty.is_empty() {
            format!(
                "for '{}' {}",
                search_query,
                if !difficulty.is_empty() {
                    format!("({difficulty})")
                } else {
                    String::new()
                }
            )
        } else {
            String::new()
        },
        questions_html,
        generate_pagination(page, limit, filtered_questions.len())
    );

    Ok(Html(html))
}

pub async fn search_code_practices_handler(
    State(state): State<AppState>,
    Query(params): Query<SearchParams>,
) -> Result<Html<String>> {
    let search_query = params.q.unwrap_or_default();
    let category = params.category.unwrap_or_default();
    let difficulty = params.difficulty.unwrap_or_default();
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(20);
    let offset = (page - 1) * limit;

    // Fetch code practices with search and pagination
    let code_practices = state.code_practice_repository.list(limit, offset).await?;

    // Filter code practices based on search query, category, and difficulty
    let filtered_practices: Vec<_> = code_practices
        .into_iter()
        .filter(|practice| {
            let matches_search = search_query.is_empty()
                || practice
                    .title
                    .get("en")
                    .to_lowercase()
                    .contains(&search_query.to_lowercase())
                || practice
                    .title
                    .get("id")
                    .to_lowercase()
                    .contains(&search_query.to_lowercase())
                || practice
                    .description
                    .get("en")
                    .to_lowercase()
                    .contains(&search_query.to_lowercase())
                || practice
                    .description
                    .get("id")
                    .to_lowercase()
                    .contains(&search_query.to_lowercase())
                || practice
                    .category
                    .to_lowercase()
                    .contains(&search_query.to_lowercase());

            let matches_category = category.is_empty()
                || practice
                    .category
                    .to_lowercase()
                    .contains(&category.to_lowercase());

            let matches_difficulty = difficulty.is_empty()
                || match difficulty.as_str() {
                    "Easy" => matches!(
                        practice.difficulty,
                        crate::domain::value_objects::Difficulty::Beginner
                    ),
                    "Medium" => matches!(
                        practice.difficulty,
                        crate::domain::value_objects::Difficulty::Intermediate
                    ),
                    "Hard" => matches!(
                        practice.difficulty,
                        crate::domain::value_objects::Difficulty::Advanced
                    ),
                    _ => true,
                };

            matches_search && matches_category && matches_difficulty
        })
        .collect();

    let mut practices_html = String::new();
    for practice in &filtered_practices {
        practices_html.push_str(&format!(
            r#"<tr>
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
            practice.category,
            match practice.difficulty {
                crate::domain::value_objects::Difficulty::Beginner => "Easy",
                crate::domain::value_objects::Difficulty::Intermediate => "Medium",
                crate::domain::value_objects::Difficulty::Advanced => "Hard",
            },
            practice.points.value(),
            practice.lesson_id.to_string(),
            practice.id.to_string(),
            practice.id.to_string()
        ));
    }

    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Search Code Practices</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; }}
        .search-container {{ margin-bottom: 30px; padding: 20px; background-color: #f8f9fa; border-radius: 8px; }}
        .search-form {{ display: flex; gap: 10px; align-items: center; }}
        .search-input {{ flex: 1; padding: 10px; border: 1px solid #ddd; border-radius: 4px; }}
        .search-select {{ padding: 10px; border: 1px solid #ddd; border-radius: 4px; }}
        .search-btn {{ padding: 10px 20px; background-color: #007bff; color: white; border: none; border-radius: 4px; cursor: pointer; }}
        .search-btn:hover {{ background-color: #0056b3; }}
        .results-info {{ margin-bottom: 20px; color: #6c757d; }}
        .btn {{ padding: 8px 16px; text-decoration: none; border-radius: 4px; font-size: 14px; }}
        .btn-primary {{ background-color: #007bff; color: white; }}
        .btn-danger {{ background-color: #dc3545; color: white; }}
        .btn-sm {{ padding: 6px 12px; font-size: 12px; }}
        table {{ width: 100%; border-collapse: collapse; margin-top: 20px; }}
        th, td {{ border: 1px solid #ddd; padding: 12px; text-align: left; }}
        th {{ background-color: #f8f9fa; font-weight: bold; }}
        tr:nth-child(even) {{ background-color: #f9f9f9; }}
        .pagination {{ margin-top: 20px; text-align: center; }}
        .pagination a {{ padding: 8px 16px; margin: 0 4px; text-decoration: none; border: 1px solid #ddd; border-radius: 4px; }}
        .pagination a:hover {{ background-color: #f8f9fa; }}
        .pagination .current {{ background-color: #007bff; color: white; }}
    </style>
</head>
<body>
    <h1>🔍 Search Code Practices</h1>

    <div class="search-container">
        <form class="search-form" method="GET" action="/admin/search/code-practices">
            <input type="text" name="q" class="search-input" placeholder="Search code practices..." value="{}">
            <input type="text" name="category" class="search-input" placeholder="Category filter..." value="{}">
            <select name="difficulty" class="search-select">
                <option value="">All Difficulties</option>
                <option value="Easy" {}>Easy</option>
                <option value="Medium" {}>Medium</option>
                <option value="Hard" {}>Hard</option>
            </select>
            <button type="submit" class="search-btn">Search</button>
        </form>
    </div>

    <div class="results-info">
        Found {} code practices
        {}
    </div>

    <table>
        <thead>
            <tr>
                <th>Title</th>
                <th>Category</th>
                <th>Difficulty</th>
                <th>Points</th>
                <th>Lesson ID</th>
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
        <a href="/admin/code-practices" class="btn btn-primary">← Back to Code Practices</a>
    </div>
</body>
</html>"#,
        search_query,
        category,
        if difficulty == "Easy" { "selected" } else { "" },
        if difficulty == "Medium" {
            "selected"
        } else {
            ""
        },
        if difficulty == "Hard" { "selected" } else { "" },
        filtered_practices.len(),
        if !search_query.is_empty() || !category.is_empty() || !difficulty.is_empty() {
            format!(
                "for '{}' {} {}",
                search_query,
                if !category.is_empty() {
                    format!("({category})")
                } else {
                    String::new()
                },
                if !difficulty.is_empty() {
                    format!("({difficulty})")
                } else {
                    String::new()
                }
            )
        } else {
            String::new()
        },
        practices_html,
        generate_pagination(page, limit, filtered_practices.len())
    );

    Ok(Html(html))
}

pub async fn search_users_handler(
    State(state): State<AppState>,
    Query(params): Query<SearchParams>,
) -> Result<Html<String>> {
    let search_query = params.q.unwrap_or_default();
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(20);
    let offset = (page - 1) * limit;

    // Fetch users with search and pagination
    let users = state.user_repository.list(limit, offset).await?;

    // Filter users based on search query
    let filtered_users: Vec<_> = if search_query.is_empty() {
        users
    } else {
        users
            .into_iter()
            .filter(|user| {
                user.display_name
                    .to_lowercase()
                    .contains(&search_query.to_lowercase())
                    || user
                        .email
                        .clone()
                        .into_string()
                        .to_lowercase()
                        .contains(&search_query.to_lowercase())
            })
            .collect()
    };

    let mut users_html = String::new();
    for user in &filtered_users {
        users_html.push_str(&format!(
            r#"<tr>
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
            user.last_active_date,
            user.id.to_string(),
            user.id.to_string()
        ));
    }

    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Search Users</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; }}
        .search-container {{ margin-bottom: 30px; padding: 20px; background-color: #f8f9fa; border-radius: 8px; }}
        .search-form {{ display: flex; gap: 10px; align-items: center; }}
        .search-input {{ flex: 1; padding: 10px; border: 1px solid #ddd; border-radius: 4px; }}
        .search-btn {{ padding: 10px 20px; background-color: #007bff; color: white; border: none; border-radius: 4px; cursor: pointer; }}
        .search-btn:hover {{ background-color: #0056b3; }}
        .results-info {{ margin-bottom: 20px; color: #6c757d; }}
        .btn {{ padding: 8px 16px; text-decoration: none; border-radius: 4px; font-size: 14px; }}
        .btn-primary {{ background-color: #007bff; color: white; }}
        .btn-danger {{ background-color: #dc3545; color: white; }}
        .btn-sm {{ padding: 6px 12px; font-size: 12px; }}
        table {{ width: 100%; border-collapse: collapse; margin-top: 20px; }}
        th, td {{ border: 1px solid #ddd; padding: 12px; text-align: left; }}
        th {{ background-color: #f8f9fa; font-weight: bold; }}
        tr:nth-child(even) {{ background-color: #f9f9f9; }}
        .pagination {{ margin-top: 20px; text-align: center; }}
        .pagination a {{ padding: 8px 16px; margin: 0 4px; text-decoration: none; border: 1px solid #ddd; border-radius: 4px; }}
        .pagination a:hover {{ background-color: #f8f9fa; }}
        .pagination .current {{ background-color: #007bff; color: white; }}
    </style>
</head>
<body>
    <h1>🔍 Search Users</h1>

    <div class="search-container">
        <form class="search-form" method="GET" action="/admin/search/users">
            <input type="text" name="q" class="search-input" placeholder="Search users..." value="{}">
            <button type="submit" class="search-btn">Search</button>
        </form>
    </div>

    <div class="results-info">
        Found {} users
        {}
    </div>

    <table>
        <thead>
            <tr>
                <th>Display Name</th>
                <th>Email</th>
                <th>Total XP</th>
                <th>Current Streak</th>
                <th>Last Active</th>
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
        <a href="/admin/users" class="btn btn-primary">← Back to Users</a>
    </div>
</body>
</html>"#,
        search_query,
        filtered_users.len(),
        if !search_query.is_empty() {
            format!("for '{search_query}'")
        } else {
            String::new()
        },
        users_html,
        generate_pagination(page, limit, filtered_users.len())
    );

    Ok(Html(html))
}

fn generate_pagination(current_page: u32, limit: u32, total_items: usize) -> String {
    let total_pages = (total_items as f64 / limit as f64).ceil() as u32;

    if total_pages <= 1 {
        return String::new();
    }

    let mut pagination_html = String::new();

    // Previous page
    if current_page > 1 {
        pagination_html.push_str(&format!(
            r#"<a href="?page={}">Previous</a>"#,
            current_page - 1
        ));
    }

    // Page numbers
    let start_page = if current_page > 3 {
        current_page - 2
    } else {
        1
    };
    let end_page = if current_page + 2 <= total_pages {
        current_page + 2
    } else {
        total_pages
    };

    for page in start_page..=end_page {
        if page == current_page {
            pagination_html.push_str(&format!(r#"<span class="current">{page}</span>"#));
        } else {
            pagination_html.push_str(&format!(r#"<a href="?page={page}">{page}</a>"#));
        }
    }

    // Next page
    if current_page < total_pages {
        pagination_html.push_str(&format!(r#"<a href="?page={}">Next</a>"#, current_page + 1));
    }

    pagination_html
}
