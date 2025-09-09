use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::entities::CodePractice;
use crate::domain::repositories::CodePracticeRepository;
use crate::domain::value_objects::{CodePracticeId, Difficulty, LessonId, TopicId};
use crate::infrastructure::database::models::CodePracticeModel;

pub struct CodePracticeRepositoryImpl {
    pool: PgPool,
}

impl CodePracticeRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CodePracticeRepository for CodePracticeRepositoryImpl {
    async fn create(&self, code_practice: &CodePractice) -> Result<()> {
        let code_practice_model = CodePracticeModel {
            id: code_practice.id.0,
            title_en: code_practice.title.en.clone(),
            title_id: code_practice.title.id.clone(),
            description_en: code_practice.description.en.clone(),
            description_id: code_practice.description.id.clone(),
            initial_code: code_practice.initial_code.clone(),
            expected_output: code_practice.expected_output.clone(),
            solution: code_practice.solution.clone(),
            hints: serde_json::to_value(&code_practice.hints)
                .unwrap_or(serde_json::Value::Array(vec![])),
            difficulty: code_practice.difficulty.to_string(),
            category: code_practice.category.clone(),
            lesson_id: code_practice.lesson_id.0,
            topic_id: code_practice.topic_id.0,
            points: code_practice.points.value() as i32,
            created_at: code_practice.created_at,
            updated_at: code_practice.updated_at,
        };

        sqlx::query!(
            r#"
            INSERT INTO code_practices (id, title_en, title_id, description_en, description_id, initial_code, expected_output, solution, hints, difficulty, category, lesson_id, topic_id, points, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
            "#,
            code_practice_model.id,
            code_practice_model.title_en,
            code_practice_model.title_id,
            code_practice_model.description_en,
            code_practice_model.description_id,
            code_practice_model.initial_code,
            code_practice_model.expected_output,
            code_practice_model.solution,
            code_practice_model.hints,
            code_practice_model.difficulty,
            code_practice_model.category,
            code_practice_model.lesson_id,
            code_practice_model.topic_id,
            code_practice_model.points,
            code_practice_model.created_at,
            code_practice_model.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;

        Ok(())
    }

    async fn find_by_id(&self, id: &CodePracticeId) -> Result<Option<CodePractice>> {
        let code_practice_model = sqlx::query_as!(
            CodePracticeModel,
            "SELECT * FROM code_practices WHERE id = $1",
            id.0
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;

        Ok(code_practice_model.map(|m| CodePractice::from(m)))
    }

    async fn find_by_topic_id(&self, topic_id: &TopicId) -> Result<Vec<CodePractice>> {
        let code_practice_models = sqlx::query_as!(
            CodePracticeModel,
            "SELECT * FROM code_practices WHERE topic_id = $1 ORDER BY created_at ASC",
            topic_id.0
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;

        Ok(code_practice_models
            .into_iter()
            .map(CodePractice::from)
            .collect())
    }

    async fn find_by_lesson_id(&self, lesson_id: &LessonId) -> Result<Vec<CodePractice>> {
        let code_practice_models = sqlx::query_as!(
            CodePracticeModel,
            "SELECT * FROM code_practices WHERE lesson_id = $1 ORDER BY created_at ASC",
            lesson_id.0
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;

        Ok(code_practice_models
            .into_iter()
            .map(CodePractice::from)
            .collect())
    }

    async fn find_by_difficulty(&self, difficulty: &Difficulty) -> Result<Vec<CodePractice>> {
        let code_practice_models = sqlx::query_as!(
            CodePracticeModel,
            "SELECT * FROM code_practices WHERE difficulty = $1 ORDER BY created_at ASC",
            difficulty.to_string()
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;

        Ok(code_practice_models
            .into_iter()
            .map(CodePractice::from)
            .collect())
    }

    async fn update(&self, code_practice: &CodePractice) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE code_practices 
            SET title_en = $2, title_id = $3, description_en = $4, description_id = $5, 
                initial_code = $6, expected_output = $7, solution = $8, hints = $9, 
                difficulty = $10, category = $11, lesson_id = $12, topic_id = $13, points = $14, updated_at = $15
            WHERE id = $1
            "#,
            code_practice.id.0,
            code_practice.title.en,
            code_practice.title.id,
            code_practice.description.en,
            code_practice.description.id,
            code_practice.initial_code,
            code_practice.expected_output,
            code_practice.solution,
            serde_json::to_value(&code_practice.hints).unwrap_or(serde_json::Value::Array(vec![])),
            code_practice.difficulty.to_string(),
            code_practice.category,
            code_practice.lesson_id.0,
            code_practice.topic_id.0,
            code_practice.points.value() as i32,
            code_practice.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;

        Ok(())
    }

    async fn delete(&self, id: &CodePracticeId) -> Result<()> {
        sqlx::query!("DELETE FROM code_practices WHERE id = $1", id.0)
            .execute(&self.pool)
            .await
            .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;

        Ok(())
    }

    async fn list(&self, limit: u32, offset: u32) -> Result<Vec<CodePractice>> {
        let code_practice_models = sqlx::query_as!(
            CodePracticeModel,
            "SELECT * FROM code_practices ORDER BY created_at DESC LIMIT $1 OFFSET $2",
            limit as i64,
            offset as i64
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;

        Ok(code_practice_models
            .into_iter()
            .map(CodePractice::from)
            .collect())
    }
}
