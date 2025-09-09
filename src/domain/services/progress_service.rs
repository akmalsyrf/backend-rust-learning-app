use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

use crate::domain::entities::user_progress::{CompletedCodePractice, QuestionResult};
use crate::domain::entities::UserProgress;
use crate::domain::value_objects::{Points, UserId};

#[async_trait]
pub trait ProgressService: Send + Sync + 'static {
    async fn get_user_progress(&self, user_id: &UserId) -> Result<Option<UserProgress>>;
    async fn update_user_progress(&self, progress: &UserProgress) -> Result<()>;
    async fn add_question_result(&self, user_id: &UserId, result: QuestionResult) -> Result<()>;
    async fn add_code_practice_completion(
        &self,
        user_id: &UserId,
        completion: CompletedCodePractice,
    ) -> Result<()>;
    async fn get_user_xp(&self, user_id: &UserId) -> Result<Points>;
    async fn update_streak(&self, user_id: &UserId, days: u32) -> Result<()>;
}

pub struct ProgressServiceImpl {
    user_progress_repository: Arc<dyn crate::domain::repositories::UserProgressRepository>,
}

impl ProgressServiceImpl {
    pub fn new(
        user_progress_repository: Arc<dyn crate::domain::repositories::UserProgressRepository>,
    ) -> Self {
        Self {
            user_progress_repository,
        }
    }
}

#[async_trait]
impl ProgressService for ProgressServiceImpl {
    async fn get_user_progress(&self, user_id: &UserId) -> Result<Option<UserProgress>> {
        self.user_progress_repository.find_by_user_id(user_id).await
    }

    async fn update_user_progress(&self, progress: &UserProgress) -> Result<()> {
        self.user_progress_repository.update(progress).await
    }

    async fn add_question_result(&self, user_id: &UserId, result: QuestionResult) -> Result<()> {
        // Get or create user progress
        let mut progress = match self
            .user_progress_repository
            .find_by_user_id(user_id)
            .await?
        {
            Some(progress) => progress,
            None => {
                let new_progress = UserProgress::new(user_id.clone());
                self.user_progress_repository.create(&new_progress).await?;
                new_progress
            }
        };

        // Add question result
        progress.add_question_result(result);

        // Update in database
        self.user_progress_repository.update(&progress).await?;

        Ok(())
    }

    async fn add_code_practice_completion(
        &self,
        user_id: &UserId,
        completion: CompletedCodePractice,
    ) -> Result<()> {
        // Get or create user progress
        let mut progress = match self
            .user_progress_repository
            .find_by_user_id(user_id)
            .await?
        {
            Some(progress) => progress,
            None => {
                let new_progress = UserProgress::new(user_id.clone());
                self.user_progress_repository.create(&new_progress).await?;
                new_progress
            }
        };

        // Add code practice completion
        progress.add_code_practice_completion(completion);

        // Update in database
        self.user_progress_repository.update(&progress).await?;

        Ok(())
    }

    async fn get_user_xp(&self, user_id: &UserId) -> Result<Points> {
        match self
            .user_progress_repository
            .find_by_user_id(user_id)
            .await?
        {
            Some(progress) => Ok(progress.total_xp),
            None => Ok(Points::new(0)), // Return 0 XP if no progress found
        }
    }

    async fn update_streak(&self, user_id: &UserId, days: u32) -> Result<()> {
        // Get or create user progress
        let mut progress = match self
            .user_progress_repository
            .find_by_user_id(user_id)
            .await?
        {
            Some(progress) => progress,
            None => {
                let new_progress = UserProgress::new(user_id.clone());
                self.user_progress_repository.create(&new_progress).await?;
                new_progress
            }
        };

        // Update streak
        progress.update_streak(days);
        progress.update_last_active();

        // Update in database
        self.user_progress_repository.update(&progress).await?;

        Ok(())
    }
}
