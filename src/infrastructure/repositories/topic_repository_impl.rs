use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::entities::Topic;
use crate::domain::repositories::TopicRepository;
use crate::domain::value_objects::TopicId;
use crate::infrastructure::database::models::TopicModel;

pub struct TopicRepositoryImpl {
    pool: PgPool,
}

impl TopicRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TopicRepository for TopicRepositoryImpl {
    async fn create(&self, topic: &Topic) -> Result<()> {
        let topic_model = TopicModel {
            id: topic.id.0,
            title_en: topic.title.en.clone(),
            title_id: topic.title.id.clone(),
            description_en: topic.description.en.clone(),
            description_id: topic.description.id.clone(),
            required_skills_en: topic.required_skills.en.clone(),
            required_skills_id: topic.required_skills.id.clone(),
            order: topic.order as i32,
            created_at: topic.created_at,
            updated_at: topic.updated_at,
        };

        sqlx::query!(
            r#"
            INSERT INTO topics (id, title_en, title_id, description_en, description_id, required_skills_en, required_skills_id, "order", created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
            topic_model.id,
            topic_model.title_en,
            topic_model.title_id,
            topic_model.description_en,
            topic_model.description_id,
            topic_model.required_skills_en,
            topic_model.required_skills_id,
            topic_model.order,
            topic_model.created_at,
            topic_model.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;

        Ok(())
    }

    async fn find_by_id(&self, id: &TopicId) -> Result<Option<Topic>> {
        let topic_model = sqlx::query_as!(TopicModel, "SELECT * FROM topics WHERE id = $1", id.0)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;

        Ok(topic_model.map(Topic::from))
    }

    async fn update(&self, topic: &Topic) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE topics 
            SET title_en = $2, title_id = $3, description_en = $4, description_id = $5, 
                required_skills_en = $6, required_skills_id = $7, "order" = $8, updated_at = $9
            WHERE id = $1
            "#,
            topic.id.0,
            topic.title.en,
            topic.title.id,
            topic.description.en,
            topic.description.id,
            topic.required_skills.en,
            topic.required_skills.id,
            topic.order as i32,
            topic.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;

        Ok(())
    }

    async fn delete(&self, id: &TopicId) -> Result<()> {
        sqlx::query!("DELETE FROM topics WHERE id = $1", id.0)
            .execute(&self.pool)
            .await
            .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;

        Ok(())
    }

    async fn list(&self, limit: u32, offset: u32) -> Result<Vec<Topic>> {
        let topic_models = sqlx::query_as!(
            TopicModel,
            "SELECT * FROM topics ORDER BY created_at DESC LIMIT $1 OFFSET $2",
            limit as i64,
            offset as i64
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;

        Ok(topic_models.into_iter().map(Topic::from).collect())
    }

    async fn list_by_order(&self) -> Result<Vec<Topic>> {
        let topic_models =
            sqlx::query_as!(TopicModel, "SELECT * FROM topics ORDER BY \"order\" ASC")
                .fetch_all(&self.pool)
                .await
                .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;

        Ok(topic_models.into_iter().map(Topic::from).collect())
    }
}
