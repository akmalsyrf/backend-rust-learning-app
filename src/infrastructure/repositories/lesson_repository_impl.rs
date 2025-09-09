use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::entities::Lesson;
use crate::domain::repositories::LessonRepository;
use crate::domain::value_objects::{LessonId, TopicId};
use crate::infrastructure::database::models::LessonModel;

pub struct LessonRepositoryImpl {
    pool: PgPool,
}

impl LessonRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl LessonRepository for LessonRepositoryImpl {
    async fn create(&self, lesson: &Lesson) -> Result<()> {
        let lesson_model = LessonModel {
            id: lesson.id.0,
            title_en: lesson.title.en.clone(),
            title_id: lesson.title.id.clone(),
            topic_id: lesson.topic_id.0,
            summary_en: lesson.summary.en.clone(),
            summary_id: lesson.summary.id.clone(),
            attribution_url: lesson.attribution_url.clone(),
            order: lesson.order as i32,
            created_at: lesson.created_at,
            updated_at: lesson.updated_at,
        };

        sqlx::query!(
            r#"
            INSERT INTO lessons (id, title_en, title_id, topic_id, summary_en, summary_id, attribution_url, "order", created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
            lesson_model.id,
            lesson_model.title_en,
            lesson_model.title_id,
            lesson_model.topic_id,
            lesson_model.summary_en,
            lesson_model.summary_id,
            lesson_model.attribution_url,
            lesson_model.order,
            lesson_model.created_at,
            lesson_model.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;

        Ok(())
    }

    async fn find_by_id(&self, id: &LessonId) -> Result<Option<Lesson>> {
        let lesson_model =
            sqlx::query_as!(LessonModel, "SELECT * FROM lessons WHERE id = $1", id.0)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;

        Ok(lesson_model.map(|m| Lesson::from(m)))
    }

    async fn find_by_topic_id(&self, topic_id: &TopicId) -> Result<Vec<Lesson>> {
        let lesson_models = sqlx::query_as!(
            LessonModel,
            "SELECT * FROM lessons WHERE topic_id = $1 ORDER BY \"order\" ASC",
            topic_id.0
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;

        Ok(lesson_models.into_iter().map(Lesson::from).collect())
    }

    async fn update(&self, lesson: &Lesson) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE lessons 
            SET title_en = $2, title_id = $3, topic_id = $4, summary_en = $5, summary_id = $6, 
                attribution_url = $7, "order" = $8, updated_at = $9
            WHERE id = $1
            "#,
            lesson.id.0,
            lesson.title.en,
            lesson.title.id,
            lesson.topic_id.0,
            lesson.summary.en,
            lesson.summary.id,
            lesson.attribution_url,
            lesson.order as i32,
            lesson.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;

        Ok(())
    }

    async fn delete(&self, id: &LessonId) -> Result<()> {
        sqlx::query!("DELETE FROM lessons WHERE id = $1", id.0)
            .execute(&self.pool)
            .await
            .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;

        Ok(())
    }

    async fn list(&self, limit: u32, offset: u32) -> Result<Vec<Lesson>> {
        let lesson_models = sqlx::query_as!(
            LessonModel,
            "SELECT * FROM lessons ORDER BY created_at DESC LIMIT $1 OFFSET $2",
            limit as i64,
            offset as i64
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;

        Ok(lesson_models.into_iter().map(Lesson::from).collect())
    }
}
