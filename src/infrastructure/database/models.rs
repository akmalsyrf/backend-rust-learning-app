use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::value_objects::{
    CodePracticeId, LessonId, LocalizedText, Points, QuestionId, TopicId, UserId,
};

// Database models that map to PostgreSQL tables

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserModel {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub display_name: String,
    pub total_xp: i32,
    pub current_streak_days: i32,
    pub highest_streak_days: i32,
    pub last_active_date: NaiveDate,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TopicModel {
    pub id: Uuid,
    pub title_en: String,
    pub title_id: String,
    pub description_en: String,
    pub description_id: String,
    pub order: i32,
    pub required_skills_en: String,
    pub required_skills_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct LessonModel {
    pub id: Uuid,
    pub title_en: String,
    pub title_id: String,
    pub topic_id: Uuid,
    pub summary_en: String,
    pub summary_id: String,
    pub attribution_url: String,
    pub order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct QuestionModel {
    pub id: Uuid,
    pub prompt_en: String,
    pub prompt_id: String,
    pub explanation_en: Option<String>,
    pub explanation_id: Option<String>,
    pub topic_id: Uuid,
    pub difficulty: String,
    pub points: i32,
    pub question_type: String,
    pub question_data: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct CodePracticeModel {
    pub id: Uuid,
    pub title_en: String,
    pub title_id: String,
    pub description_en: String,
    pub description_id: String,
    pub initial_code: String,
    pub expected_output: Option<String>,
    pub solution: String,
    pub hints: serde_json::Value,
    pub difficulty: String,
    pub category: String,
    pub lesson_id: Uuid,
    pub topic_id: Uuid,
    pub points: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserProgressModel {
    pub user_id: Uuid,
    pub total_xp: i32,
    pub current_streak_days: i32,
    pub highest_streak_days: i32,
    pub last_active_date: NaiveDate,
    pub daily_xp_cap: i32,
    pub last_xp_reset_date: NaiveDate,
    pub completed_questions: serde_json::Value,
    pub completed_code_practices: serde_json::Value,
    pub lesson_stars: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct QuestionResultModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub question_id: Uuid,
    pub correct: bool,
    pub user_answer: String,
    pub time_spent_ms: i64,
    pub points: i32,
    pub completed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct LessonResultModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub lesson_id: Uuid,
    pub xp_earned: i32,
    pub perfect_score: bool,
    pub completed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct LessonStarModel {
    pub user_id: Uuid,
    pub lesson_id: Uuid,
    pub stars: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct CompletedCodePracticeModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub code_practice_id: Uuid,
    pub user_code: String,
    pub is_correct: bool,
    pub xp_earned: i32,
    pub completed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct LeaderboardEntryModel {
    pub user_id: Uuid,
    pub display_name: String,
    pub total_xp: i32,
    pub xp_this_week: i32,
    pub rank: i64,
}

// Conversion traits from database models to domain entities

impl From<UserModel> for crate::domain::entities::User {
    fn from(model: UserModel) -> Self {
        use crate::domain::value_objects::{Email, Password};

        Self {
            id: UserId::from(model.id),
            email: Email::new(model.email)
                .unwrap_or_else(|_| Email::new("invalid@example.com".to_string()).unwrap()),
            password: Password::from_hash(model.password_hash),
            display_name: model.display_name,
            total_xp: Points::new(model.total_xp as u32),
            current_streak_days: model.current_streak_days as u32,
            highest_streak_days: model.highest_streak_days as u32,
            last_active_date: model.last_active_date,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

impl From<TopicModel> for crate::domain::entities::Topic {
    fn from(model: TopicModel) -> Self {
        Self {
            id: TopicId::from(model.id),
            title: LocalizedText::new(model.title_en, model.title_id),
            description: LocalizedText::new(model.description_en, model.description_id),
            order: model.order as u32,
            lessons: Vec::new(), // Will be populated separately
            required_skills: LocalizedText::new(model.required_skills_en, model.required_skills_id),
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

impl From<LessonModel> for crate::domain::entities::Lesson {
    fn from(model: LessonModel) -> Self {
        Self {
            id: LessonId::from(model.id),
            title: LocalizedText::new(model.title_en, model.title_id),
            topic_id: TopicId::from(model.topic_id),
            summary: LocalizedText::new(model.summary_en, model.summary_id),
            questions: Vec::new(), // Will be populated separately
            attribution_url: model.attribution_url,
            order: model.order as u32,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

impl From<QuestionModel> for crate::domain::entities::Question {
    fn from(model: QuestionModel) -> Self {
        use crate::domain::entities::QuestionType;

        let question_type = match model.question_data.get("type").and_then(|v| v.as_str()) {
            Some("multiple_choice") => {
                let choices = model
                    .question_data
                    .get("choices")
                    .and_then(|v| serde_json::from_value(v.clone()).ok())
                    .unwrap_or_else(|| LocalizedText::new("".to_string(), "".to_string()));
                let correct_index = model
                    .question_data
                    .get("correct_index")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0) as u32;
                QuestionType::MultipleChoice {
                    choices,
                    correct_index,
                }
            }
            Some("true_false") => {
                let answer = model
                    .question_data
                    .get("answer")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                QuestionType::TrueFalse { answer }
            }
            _ => QuestionType::TrueFalse { answer: false }, // Default fallback
        };

        let explanation = match (model.explanation_en, model.explanation_id) {
            (Some(en), Some(id)) => Some(LocalizedText::new(en, id)),
            _ => None,
        };

        Self {
            id: QuestionId::from(model.id),
            prompt: LocalizedText::new(model.prompt_en, model.prompt_id),
            explanation,
            topic_id: TopicId::from(model.topic_id),
            difficulty: model
                .difficulty
                .parse()
                .unwrap_or(crate::domain::value_objects::Difficulty::Beginner),
            points: Points::new(model.points as u32),
            question_type,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

impl From<CodePracticeModel> for crate::domain::entities::CodePractice {
    fn from(model: CodePracticeModel) -> Self {
        let hints: Vec<LocalizedText> = serde_json::from_value(model.hints).unwrap_or_default();

        Self {
            id: CodePracticeId::from(model.id),
            title: LocalizedText::new(model.title_en, model.title_id),
            description: LocalizedText::new(model.description_en, model.description_id),
            initial_code: model.initial_code,
            expected_output: model.expected_output,
            solution: model.solution,
            hints,
            difficulty: model
                .difficulty
                .parse()
                .unwrap_or(crate::domain::value_objects::Difficulty::Beginner),
            category: model.category,
            lesson_id: LessonId::from(model.lesson_id),
            topic_id: TopicId::from(model.topic_id),
            points: Points::new(model.points as u32),
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

impl From<UserProgressModel> for crate::domain::entities::UserProgress {
    fn from(model: UserProgressModel) -> Self {
        let completed_questions: Vec<crate::domain::entities::user_progress::QuestionResult> =
            serde_json::from_value(model.completed_questions).unwrap_or_default();
        let completed_code_practices: Vec<
            crate::domain::entities::user_progress::CompletedCodePractice,
        > = serde_json::from_value(model.completed_code_practices).unwrap_or_default();
        let lesson_stars: Vec<(LessonId, u32)> =
            serde_json::from_value(model.lesson_stars).unwrap_or_default();

        Self {
            user_id: UserId::from(model.user_id),
            completed_questions,
            lesson_stars,
            completed_code_practices,
            total_xp: Points::new(model.total_xp as u32),
            current_streak_days: model.current_streak_days as u32,
            highest_streak_days: model.highest_streak_days as u32,
            last_active_date: model.last_active_date,
            daily_xp_cap: model.daily_xp_cap as u32,
            last_xp_reset_date: model.last_xp_reset_date,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

impl From<LeaderboardEntryModel> for crate::domain::entities::LeaderboardEntry {
    fn from(model: LeaderboardEntryModel) -> Self {
        Self {
            user_id: UserId::from(model.user_id),
            display_name: model.display_name,
            xp_this_week: Points::new(model.xp_this_week as u32),
            total_xp: Points::new(model.total_xp as u32),
            rank: model.rank as u32,
        }
    }
}

// Reverse conversions from domain entities to database models

impl From<crate::domain::entities::User> for UserModel {
    fn from(entity: crate::domain::entities::User) -> Self {
        Self {
            id: entity.id.0,
            email: entity.email.as_str().to_string(),
            password_hash: entity.password.as_str().to_string(),
            display_name: entity.display_name,
            total_xp: entity.total_xp.value() as i32,
            current_streak_days: entity.current_streak_days as i32,
            highest_streak_days: entity.highest_streak_days as i32,
            last_active_date: entity.last_active_date,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}

impl From<crate::domain::entities::Topic> for TopicModel {
    fn from(entity: crate::domain::entities::Topic) -> Self {
        Self {
            id: entity.id.0,
            title_en: entity.title.en,
            title_id: entity.title.id,
            description_en: entity.description.en,
            description_id: entity.description.id,
            order: entity.order as i32,
            required_skills_en: entity.required_skills.en,
            required_skills_id: entity.required_skills.id,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}

impl From<crate::domain::entities::Lesson> for LessonModel {
    fn from(entity: crate::domain::entities::Lesson) -> Self {
        Self {
            id: entity.id.0,
            title_en: entity.title.en,
            title_id: entity.title.id,
            topic_id: entity.topic_id.0,
            summary_en: entity.summary.en,
            summary_id: entity.summary.id,
            attribution_url: entity.attribution_url,
            order: entity.order as i32,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}

impl From<crate::domain::entities::Question> for QuestionModel {
    fn from(entity: crate::domain::entities::Question) -> Self {
        let question_type_json = match &entity.question_type {
            crate::domain::entities::QuestionType::MultipleChoice {
                choices,
                correct_index,
            } => serde_json::json!({
                "type": "multiple_choice",
                "choices": choices,
                "correct_index": correct_index
            })
            .to_string(),
            crate::domain::entities::QuestionType::TrueFalse { answer } => serde_json::json!({
                "type": "true_false",
                "answer": answer
            })
            .to_string(),
            _ => serde_json::json!({"type": "unknown"}).to_string(),
        };

        let (explanation_en, explanation_id) = match &entity.explanation {
            Some(exp) => (Some(exp.en.clone()), Some(exp.id.clone())),
            None => (None, None),
        };

        Self {
            id: entity.id.0,
            prompt_en: entity.prompt.en,
            prompt_id: entity.prompt.id,
            topic_id: entity.topic_id.0,
            difficulty: entity.difficulty.to_string(),
            points: entity.points.value() as i32,
            question_type: match &entity.question_type {
                crate::domain::entities::QuestionType::MultipleChoice { .. } => {
                    "multiple_choice".to_string()
                }
                crate::domain::entities::QuestionType::TrueFalse { .. } => "true_false".to_string(),
                _ => "unknown".to_string(),
            },
            question_data: serde_json::Value::String(question_type_json),
            explanation_en,
            explanation_id,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}

impl From<crate::domain::entities::CodePractice> for CodePracticeModel {
    fn from(entity: crate::domain::entities::CodePractice) -> Self {
        Self {
            id: entity.id.0,
            title_en: entity.title.en,
            title_id: entity.title.id,
            description_en: entity.description.en,
            description_id: entity.description.id,
            initial_code: entity.initial_code,
            expected_output: entity.expected_output,
            solution: entity.solution,
            hints: serde_json::to_value(&entity.hints).unwrap_or_default(),
            difficulty: entity.difficulty.to_string(),
            category: entity.category,
            lesson_id: entity.lesson_id.0,
            topic_id: entity.topic_id.0,
            points: entity.points.value() as i32,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}

impl From<crate::domain::entities::UserProgress> for UserProgressModel {
    fn from(entity: crate::domain::entities::UserProgress) -> Self {
        Self {
            user_id: entity.user_id.0,
            total_xp: entity.total_xp.value() as i32,
            current_streak_days: entity.current_streak_days as i32,
            highest_streak_days: entity.highest_streak_days as i32,
            last_active_date: entity.last_active_date,
            completed_questions: serde_json::to_value(&entity.completed_questions)
                .unwrap_or_default(),
            completed_code_practices: serde_json::to_value(&entity.completed_code_practices)
                .unwrap_or_default(),
            daily_xp_cap: entity.daily_xp_cap as i32,
            last_xp_reset_date: entity.last_xp_reset_date,
            lesson_stars: serde_json::to_value(&entity.lesson_stars).unwrap_or_default(),
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}
