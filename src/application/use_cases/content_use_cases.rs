use anyhow::Result;
use serde::Serialize;
use std::sync::Arc;

use crate::domain::entities::{CodePractice, Lesson, Question, Topic};
use crate::domain::repositories::{
    CodePracticeRepository, LessonRepository, QuestionRepository, TopicRepository,
};
use crate::domain::value_objects::{Difficulty, LessonId, TopicId};

#[derive(Debug, Serialize)]
pub struct TopicResponse {
    pub id: String,
    pub title: LocalizedTextResponse,
    pub description: LocalizedTextResponse,
    pub order: u32,
    pub required_skills: LocalizedTextResponse,
}

#[derive(Debug, Serialize)]
pub struct LessonResponse {
    pub id: String,
    pub title: LocalizedTextResponse,
    pub topic_id: String,
    pub summary: LocalizedTextResponse,
    pub attribution_url: String,
    pub order: u32,
}

#[derive(Debug, Serialize)]
pub struct QuestionResponse {
    pub id: String,
    pub prompt: LocalizedTextResponse,
    pub explanation: Option<LocalizedTextResponse>,
    pub topic_id: String,
    pub difficulty: String,
    pub points: u32,
    pub question_type: String,
}

#[derive(Debug, Serialize)]
pub struct CodePracticeResponse {
    pub id: String,
    pub title: LocalizedTextResponse,
    pub description: LocalizedTextResponse,
    pub initial_code: String,
    pub expected_output: Option<String>,
    pub solution: String,
    pub hints: Vec<LocalizedTextResponse>,
    pub difficulty: String,
    pub category: String,
    pub lesson_id: String,
    pub topic_id: String,
    pub points: u32,
}

#[derive(Debug, Serialize)]
pub struct LocalizedTextResponse {
    pub en: String,
    pub id: String,
}

impl From<crate::domain::value_objects::LocalizedText> for LocalizedTextResponse {
    fn from(text: crate::domain::value_objects::LocalizedText) -> Self {
        Self {
            en: text.en,
            id: text.id,
        }
    }
}

impl From<Topic> for TopicResponse {
    fn from(topic: Topic) -> Self {
        Self {
            id: topic.id.to_string(),
            title: LocalizedTextResponse::from(topic.title),
            description: LocalizedTextResponse::from(topic.description),
            order: topic.order,
            required_skills: LocalizedTextResponse::from(topic.required_skills),
        }
    }
}

impl From<Lesson> for LessonResponse {
    fn from(lesson: Lesson) -> Self {
        Self {
            id: lesson.id.to_string(),
            title: LocalizedTextResponse::from(lesson.title),
            topic_id: lesson.topic_id.to_string(),
            summary: LocalizedTextResponse::from(lesson.summary),
            attribution_url: lesson.attribution_url,
            order: lesson.order,
        }
    }
}

impl From<Question> for QuestionResponse {
    fn from(question: Question) -> Self {
        Self {
            id: question.id.to_string(),
            prompt: LocalizedTextResponse::from(question.prompt),
            explanation: question.explanation.map(LocalizedTextResponse::from),
            topic_id: question.topic_id.to_string(),
            difficulty: question.difficulty.to_string(),
            points: question.points.value(),
            question_type: format!("{:?}", question.question_type),
        }
    }
}

impl From<CodePractice> for CodePracticeResponse {
    fn from(code_practice: CodePractice) -> Self {
        Self {
            id: code_practice.id.to_string(),
            title: LocalizedTextResponse::from(code_practice.title),
            description: LocalizedTextResponse::from(code_practice.description),
            initial_code: code_practice.initial_code,
            expected_output: code_practice.expected_output,
            solution: code_practice.solution,
            hints: code_practice
                .hints
                .into_iter()
                .map(LocalizedTextResponse::from)
                .collect(),
            difficulty: code_practice.difficulty.to_string(),
            category: code_practice.category,
            lesson_id: code_practice.lesson_id.to_string(),
            topic_id: code_practice.topic_id.to_string(),
            points: code_practice.points.value(),
        }
    }
}

#[derive(Clone)]
pub struct ContentUseCases {
    topic_repository: Arc<dyn TopicRepository>,
    lesson_repository: Arc<dyn LessonRepository>,
    question_repository: Arc<dyn QuestionRepository>,
    code_practice_repository: Arc<dyn CodePracticeRepository>,
}

impl ContentUseCases {
    pub fn new(
        topic_repository: Arc<dyn TopicRepository>,
        lesson_repository: Arc<dyn LessonRepository>,
        question_repository: Arc<dyn QuestionRepository>,
        code_practice_repository: Arc<dyn CodePracticeRepository>,
    ) -> Self {
        Self {
            topic_repository,
            lesson_repository,
            question_repository,
            code_practice_repository,
        }
    }

    pub async fn get_topics(&self) -> Result<Vec<TopicResponse>> {
        let topics = self.topic_repository.list(50, 0).await?;
        Ok(topics.into_iter().map(TopicResponse::from).collect())
    }

    pub async fn get_lessons(&self, topic_id: Option<String>) -> Result<Vec<LessonResponse>> {
        let lessons = if let Some(topic_id_str) = topic_id {
            let topic_id = TopicId::from_str(&topic_id_str).map_err(|e| anyhow::anyhow!(e))?;
            self.lesson_repository.find_by_topic_id(&topic_id).await?
        } else {
            self.lesson_repository.list(50, 0).await?
        };
        Ok(lessons.into_iter().map(LessonResponse::from).collect())
    }

    pub async fn get_questions(
        &self,
        topic_id: Option<String>,
        difficulty: Option<String>,
    ) -> Result<Vec<QuestionResponse>> {
        let questions = if let Some(topic_id_str) = topic_id {
            let topic_id = TopicId::from_str(&topic_id_str).map_err(|e| anyhow::anyhow!(e))?;
            self.question_repository.find_by_topic_id(&topic_id).await?
        } else if let Some(difficulty_str) = difficulty {
            let difficulty =
                Difficulty::from_str(&difficulty_str).map_err(|e| anyhow::anyhow!(e))?;
            self.question_repository
                .find_by_difficulty(&difficulty)
                .await?
        } else {
            self.question_repository.list(50, 0).await?
        };
        Ok(questions.into_iter().map(QuestionResponse::from).collect())
    }

    pub async fn get_code_practices(
        &self,
        topic_id: Option<String>,
        lesson_id: Option<String>,
    ) -> Result<Vec<CodePracticeResponse>> {
        let code_practices = if let Some(topic_id_str) = topic_id {
            let topic_id = TopicId::from_str(&topic_id_str).map_err(|e| anyhow::anyhow!(e))?;
            self.code_practice_repository
                .find_by_topic_id(&topic_id)
                .await?
        } else if let Some(lesson_id_str) = lesson_id {
            let lesson_id = LessonId::from_str(&lesson_id_str).map_err(|e| anyhow::anyhow!(e))?;
            self.code_practice_repository
                .find_by_lesson_id(&lesson_id)
                .await?
        } else {
            self.code_practice_repository.list(50, 0).await?
        };
        Ok(code_practices
            .into_iter()
            .map(CodePracticeResponse::from)
            .collect())
    }
}
