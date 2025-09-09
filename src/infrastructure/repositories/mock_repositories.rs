use crate::domain::entities::*;
use crate::domain::repositories::*;
use crate::domain::value_objects::*;
use crate::infrastructure::database::mock_connection::MockDatabaseConnection;
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

// Mock User Repository
pub struct MockUserRepositoryImpl {
    db: Arc<MockDatabaseConnection>,
}

impl MockUserRepositoryImpl {
    pub fn new(db: Arc<MockDatabaseConnection>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserRepository for MockUserRepositoryImpl {
    async fn create(&self, user: &User) -> Result<()> {
        self.db.create_user(user).await
    }

    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>> {
        self.db.find_user_by_id(id).await
    }

    async fn find_by_email(&self, email: &Email) -> Result<Option<User>> {
        self.db.find_user_by_email(email).await
    }

    async fn update(&self, user: &User) -> Result<()> {
        self.db.update_user(user).await
    }

    async fn delete(&self, id: &UserId) -> Result<()> {
        self.db.delete_user(id).await
    }

    async fn list(&self, limit: u32, offset: u32) -> Result<Vec<User>> {
        self.db.list_users(limit, offset).await
    }
}

// Mock Topic Repository
pub struct MockTopicRepositoryImpl {
    db: Arc<MockDatabaseConnection>,
}

impl MockTopicRepositoryImpl {
    pub fn new(db: Arc<MockDatabaseConnection>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl TopicRepository for MockTopicRepositoryImpl {
    async fn create(&self, topic: &Topic) -> Result<()> {
        self.db.create_topic(topic).await
    }

    async fn find_by_id(&self, id: &TopicId) -> Result<Option<Topic>> {
        self.db.find_topic_by_id(id).await
    }

    async fn update(&self, topic: &Topic) -> Result<()> {
        self.db.update_topic(topic).await
    }

    async fn delete(&self, id: &TopicId) -> Result<()> {
        self.db.delete_topic(id).await
    }

    async fn list(&self, limit: u32, offset: u32) -> Result<Vec<Topic>> {
        self.db.list_topics(limit, offset).await
    }

    async fn list_by_order(&self) -> Result<Vec<Topic>> {
        self.db.list_topics_by_order().await
    }
}

// Mock Lesson Repository
pub struct MockLessonRepositoryImpl {
    db: Arc<MockDatabaseConnection>,
}

impl MockLessonRepositoryImpl {
    pub fn new(db: Arc<MockDatabaseConnection>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl LessonRepository for MockLessonRepositoryImpl {
    async fn create(&self, lesson: &Lesson) -> Result<()> {
        self.db.create_lesson(lesson).await
    }

    async fn find_by_id(&self, id: &LessonId) -> Result<Option<Lesson>> {
        self.db.find_lesson_by_id(id).await
    }

    async fn find_by_topic_id(&self, topic_id: &TopicId) -> Result<Vec<Lesson>> {
        self.db.find_lessons_by_topic_id(topic_id).await
    }

    async fn update(&self, lesson: &Lesson) -> Result<()> {
        self.db.update_lesson(lesson).await
    }

    async fn delete(&self, id: &LessonId) -> Result<()> {
        self.db.delete_lesson(id).await
    }

    async fn list(&self, limit: u32, offset: u32) -> Result<Vec<Lesson>> {
        self.db.list_lessons(limit, offset).await
    }
}

// Mock Question Repository
pub struct MockQuestionRepositoryImpl {
    db: Arc<MockDatabaseConnection>,
}

impl MockQuestionRepositoryImpl {
    pub fn new(db: Arc<MockDatabaseConnection>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl QuestionRepository for MockQuestionRepositoryImpl {
    async fn create(&self, question: &Question) -> Result<()> {
        self.db.create_question(question).await
    }

    async fn find_by_id(&self, id: &QuestionId) -> Result<Option<Question>> {
        self.db.find_question_by_id(id).await
    }

    async fn find_by_topic_id(&self, topic_id: &TopicId) -> Result<Vec<Question>> {
        self.db.find_questions_by_topic_id(topic_id).await
    }

    async fn find_by_difficulty(&self, difficulty: &Difficulty) -> Result<Vec<Question>> {
        self.db.find_questions_by_difficulty(difficulty).await
    }

    async fn update(&self, question: &Question) -> Result<()> {
        self.db.update_question(question).await
    }

    async fn delete(&self, id: &QuestionId) -> Result<()> {
        self.db.delete_question(id).await
    }

    async fn list(&self, limit: u32, offset: u32) -> Result<Vec<Question>> {
        self.db.list_questions(limit, offset).await
    }
}

// Mock Code Practice Repository
pub struct MockCodePracticeRepositoryImpl {
    db: Arc<MockDatabaseConnection>,
}

impl MockCodePracticeRepositoryImpl {
    pub fn new(db: Arc<MockDatabaseConnection>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl CodePracticeRepository for MockCodePracticeRepositoryImpl {
    async fn create(&self, code_practice: &CodePractice) -> Result<()> {
        self.db.create_code_practice(code_practice).await
    }

    async fn find_by_id(&self, id: &CodePracticeId) -> Result<Option<CodePractice>> {
        self.db.find_code_practice_by_id(id).await
    }

    async fn find_by_topic_id(&self, topic_id: &TopicId) -> Result<Vec<CodePractice>> {
        self.db.find_code_practices_by_topic_id(topic_id).await
    }

    async fn find_by_lesson_id(&self, lesson_id: &LessonId) -> Result<Vec<CodePractice>> {
        self.db.find_code_practices_by_lesson_id(lesson_id).await
    }

    async fn find_by_difficulty(&self, difficulty: &Difficulty) -> Result<Vec<CodePractice>> {
        self.db.find_code_practices_by_difficulty(difficulty).await
    }

    async fn update(&self, code_practice: &CodePractice) -> Result<()> {
        self.db.update_code_practice(code_practice).await
    }

    async fn delete(&self, id: &CodePracticeId) -> Result<()> {
        self.db.delete_code_practice(id).await
    }

    async fn list(&self, limit: u32, offset: u32) -> Result<Vec<CodePractice>> {
        self.db.list_code_practices(limit, offset).await
    }
}

// Mock User Progress Repository
pub struct MockUserProgressRepositoryImpl {
    db: Arc<MockDatabaseConnection>,
}

impl MockUserProgressRepositoryImpl {
    pub fn new(db: Arc<MockDatabaseConnection>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserProgressRepository for MockUserProgressRepositoryImpl {
    async fn create(&self, progress: &UserProgress) -> Result<()> {
        self.db.create_user_progress(progress).await
    }

    async fn find_by_user_id(&self, user_id: &UserId) -> Result<Option<UserProgress>> {
        self.db.find_user_progress_by_user_id(user_id).await
    }

    async fn update(&self, progress: &UserProgress) -> Result<()> {
        self.db.update_user_progress(progress).await
    }

    async fn delete(&self, user_id: &UserId) -> Result<()> {
        self.db.delete_user_progress(user_id).await
    }

    async fn list(&self, _limit: u32, _offset: u32) -> Result<Vec<UserProgress>> {
        // For demo purposes, return empty list
        Ok(Vec::new())
    }
}

// Mock Leaderboard Repository
pub struct MockLeaderboardRepositoryImpl {
    db: Arc<MockDatabaseConnection>,
}

impl MockLeaderboardRepositoryImpl {
    pub fn new(db: Arc<MockDatabaseConnection>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl LeaderboardRepository for MockLeaderboardRepositoryImpl {
    async fn get_weekly_leaderboard(&self, limit: u32) -> Result<Vec<LeaderboardEntry>> {
        self.db.get_weekly_leaderboard(limit).await
    }

    async fn get_all_time_leaderboard(&self, limit: u32) -> Result<Vec<LeaderboardEntry>> {
        self.db.get_all_time_leaderboard(limit).await
    }

    async fn get_user_rank(&self, user_id: &UserId) -> Result<Option<u32>> {
        self.db.get_user_rank(user_id).await
    }
}
