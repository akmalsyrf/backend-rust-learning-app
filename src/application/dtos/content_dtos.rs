use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct TopicDto {
    pub id: String,
    pub title: LocalizedTextDto,
    pub description: LocalizedTextDto,
    pub order: u32,
    pub required_skills: LocalizedTextDto,
}

#[derive(Debug, Serialize)]
pub struct LessonDto {
    pub id: String,
    pub title: LocalizedTextDto,
    pub topic_id: String,
    pub summary: LocalizedTextDto,
    pub attribution_url: String,
    pub order: u32,
}

#[derive(Debug, Serialize)]
pub struct QuestionDto {
    pub id: String,
    pub prompt: LocalizedTextDto,
    pub explanation: Option<LocalizedTextDto>,
    pub topic_id: String,
    pub difficulty: String,
    pub points: u32,
    pub question_type: String,
}

#[derive(Debug, Serialize)]
pub struct CodePracticeDto {
    pub id: String,
    pub title: LocalizedTextDto,
    pub description: LocalizedTextDto,
    pub initial_code: String,
    pub expected_output: Option<String>,
    pub solution: String,
    pub hints: Vec<LocalizedTextDto>,
    pub difficulty: String,
    pub category: String,
    pub lesson_id: String,
    pub topic_id: String,
    pub points: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocalizedTextDto {
    pub en: String,
    pub id: String,
}
