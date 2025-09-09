use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::entities::Question;
use crate::domain::repositories::QuestionRepository;
use crate::domain::value_objects::{Difficulty, QuestionId, TopicId};
use crate::infrastructure::database::models::QuestionModel;

pub struct QuestionRepositoryImpl {
    pool: PgPool,
}

impl QuestionRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl QuestionRepository for QuestionRepositoryImpl {
    async fn create(&self, question: &Question) -> Result<()> {
        let question_model = QuestionModel {
            id: question.id.0,
            prompt_en: question.prompt.en.clone(),
            prompt_id: question.prompt.id.clone(),
            explanation_en: question.explanation.as_ref().map(|e| e.en.clone()),
            explanation_id: question.explanation.as_ref().map(|e| e.id.clone()),
            topic_id: question.topic_id.0,
            difficulty: question.difficulty.to_string(),
            points: question.points.value() as i32,
            question_type: format!("{:?}", question.question_type),
            question_data: serde_json::Value::Object(serde_json::Map::new()), // TODO: Store actual question data
            created_at: question.created_at,
            updated_at: question.updated_at,
        };

        sqlx::query!(
            r#"
            INSERT INTO questions (id, prompt_en, prompt_id, explanation_en, explanation_id, topic_id, difficulty, points, question_type, question_data, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            "#,
            question_model.id,
            question_model.prompt_en,
            question_model.prompt_id,
            question_model.explanation_en,
            question_model.explanation_id,
            question_model.topic_id,
            question_model.difficulty,
            question_model.points,
            question_model.question_type,
            question_model.question_data,
            question_model.created_at,
            question_model.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;

        Ok(())
    }

    async fn find_by_id(&self, id: &QuestionId) -> Result<Option<Question>> {
        let question_model =
            sqlx::query_as!(QuestionModel, "SELECT * FROM questions WHERE id = $1", id.0)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;

        Ok(question_model.map(|m| Question::from(m)))
    }

    async fn find_by_topic_id(&self, topic_id: &TopicId) -> Result<Vec<Question>> {
        let question_models = sqlx::query_as!(
            QuestionModel,
            "SELECT * FROM questions WHERE topic_id = $1 ORDER BY created_at ASC",
            topic_id.0
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;

        Ok(question_models.into_iter().map(Question::from).collect())
    }

    async fn find_by_difficulty(&self, difficulty: &Difficulty) -> Result<Vec<Question>> {
        let question_models = sqlx::query_as!(
            QuestionModel,
            "SELECT * FROM questions WHERE difficulty = $1 ORDER BY created_at ASC",
            difficulty.to_string()
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;

        Ok(question_models.into_iter().map(Question::from).collect())
    }

    async fn update(&self, question: &Question) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE questions 
            SET prompt_en = $2, prompt_id = $3, explanation_en = $4, explanation_id = $5, 
                topic_id = $6, difficulty = $7, points = $8, question_type = $9, updated_at = $10
            WHERE id = $1
            "#,
            question.id.0,
            question.prompt.en,
            question.prompt.id,
            question.explanation.as_ref().map(|e| e.en.as_str()),
            question.explanation.as_ref().map(|e| e.id.as_str()),
            question.topic_id.0,
            question.difficulty.to_string(),
            question.points.value() as i32,
            format!("{:?}", question.question_type),
            question.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;

        Ok(())
    }

    async fn delete(&self, id: &QuestionId) -> Result<()> {
        sqlx::query!("DELETE FROM questions WHERE id = $1", id.0)
            .execute(&self.pool)
            .await
            .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;

        Ok(())
    }

    async fn list(&self, limit: u32, offset: u32) -> Result<Vec<Question>> {
        let question_models = sqlx::query_as!(
            QuestionModel,
            "SELECT * FROM questions ORDER BY created_at DESC LIMIT $1 OFFSET $2",
            limit as i64,
            offset as i64
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;

        Ok(question_models.into_iter().map(Question::from).collect())
    }
}
