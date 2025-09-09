use crate::domain::entities::*;
use crate::domain::value_objects::*;
use crate::infrastructure::database::models::*;
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct MockDatabaseConnection {
    pub users: Arc<Mutex<HashMap<UserId, UserModel>>>,
    pub topics: Arc<Mutex<HashMap<TopicId, TopicModel>>>,
    pub lessons: Arc<Mutex<HashMap<LessonId, LessonModel>>>,
    pub questions: Arc<Mutex<HashMap<QuestionId, QuestionModel>>>,
    pub code_practices: Arc<Mutex<HashMap<CodePracticeId, CodePracticeModel>>>,
    pub user_progress: Arc<Mutex<HashMap<UserId, UserProgressModel>>>,
}

impl MockDatabaseConnection {
    pub fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(HashMap::new())),
            topics: Arc::new(Mutex::new(HashMap::new())),
            lessons: Arc::new(Mutex::new(HashMap::new())),
            questions: Arc::new(Mutex::new(HashMap::new())),
            code_practices: Arc::new(Mutex::new(HashMap::new())),
            user_progress: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn clear_all(&self) -> Result<()> {
        self.users.lock().await.clear();
        self.topics.lock().await.clear();
        self.lessons.lock().await.clear();
        self.questions.lock().await.clear();
        self.code_practices.lock().await.clear();
        self.user_progress.lock().await.clear();
        Ok(())
    }
}

// Mock implementations for each repository
impl MockDatabaseConnection {
    // User operations
    pub async fn create_user(&self, user: &User) -> Result<()> {
        let user_model = UserModel::from(user.clone());
        self.users.lock().await.insert(user.id.clone(), user_model);
        Ok(())
    }

    pub async fn find_user_by_id(&self, id: &UserId) -> Result<Option<User>> {
        let users = self.users.lock().await;
        Ok(users.get(id).map(|model| User::from(model.clone())))
    }

    pub async fn find_user_by_email(&self, email: &Email) -> Result<Option<User>> {
        let users = self.users.lock().await;
        Ok(users
            .values()
            .find(|u| u.email == email.as_str())
            .map(|model| User::from(model.clone())))
    }

    pub async fn update_user(&self, user: &User) -> Result<()> {
        let user_model = UserModel::from(user.clone());
        self.users.lock().await.insert(user.id.clone(), user_model);
        Ok(())
    }

    pub async fn delete_user(&self, id: &UserId) -> Result<()> {
        self.users.lock().await.remove(id);
        Ok(())
    }

    pub async fn list_users(&self, limit: u32, offset: u32) -> Result<Vec<User>> {
        let users = self.users.lock().await;
        let users_vec: Vec<User> = users
            .values()
            .skip(offset as usize)
            .take(limit as usize)
            .map(|model| User::from(model.clone()))
            .collect();
        Ok(users_vec)
    }

    // Topic operations
    pub async fn create_topic(&self, topic: &Topic) -> Result<()> {
        let topic_model = TopicModel::from(topic.clone());
        self.topics
            .lock()
            .await
            .insert(topic.id.clone(), topic_model);
        Ok(())
    }

    pub async fn find_topic_by_id(&self, id: &TopicId) -> Result<Option<Topic>> {
        let topics = self.topics.lock().await;
        Ok(topics.get(id).map(|model| Topic::from(model.clone())))
    }

    pub async fn update_topic(&self, topic: &Topic) -> Result<()> {
        let topic_model = TopicModel::from(topic.clone());
        self.topics
            .lock()
            .await
            .insert(topic.id.clone(), topic_model);
        Ok(())
    }

    pub async fn delete_topic(&self, id: &TopicId) -> Result<()> {
        self.topics.lock().await.remove(id);
        Ok(())
    }

    pub async fn list_topics(&self, limit: u32, offset: u32) -> Result<Vec<Topic>> {
        let topics = self.topics.lock().await;
        let topics_vec: Vec<Topic> = topics
            .values()
            .skip(offset as usize)
            .take(limit as usize)
            .map(|model| Topic::from(model.clone()))
            .collect();
        Ok(topics_vec)
    }

    pub async fn list_topics_by_order(&self) -> Result<Vec<Topic>> {
        let topics = self.topics.lock().await;
        let mut topics_vec: Vec<Topic> = topics
            .values()
            .map(|model| Topic::from(model.clone()))
            .collect();
        topics_vec.sort_by_key(|t| t.order);
        Ok(topics_vec)
    }

    // Lesson operations
    pub async fn create_lesson(&self, lesson: &Lesson) -> Result<()> {
        let lesson_model = LessonModel::from(lesson.clone());
        self.lessons
            .lock()
            .await
            .insert(lesson.id.clone(), lesson_model);
        Ok(())
    }

    pub async fn find_lesson_by_id(&self, id: &LessonId) -> Result<Option<Lesson>> {
        let lessons = self.lessons.lock().await;
        Ok(lessons.get(id).map(|model| Lesson::from(model.clone())))
    }

    pub async fn find_lessons_by_topic_id(&self, topic_id: &TopicId) -> Result<Vec<Lesson>> {
        let lessons = self.lessons.lock().await;
        let lessons_vec: Vec<Lesson> = lessons
            .values()
            .filter(|l| l.topic_id == topic_id.0)
            .map(|model| Lesson::from(model.clone()))
            .collect();
        Ok(lessons_vec)
    }

    pub async fn update_lesson(&self, lesson: &Lesson) -> Result<()> {
        let lesson_model = LessonModel::from(lesson.clone());
        self.lessons
            .lock()
            .await
            .insert(lesson.id.clone(), lesson_model);
        Ok(())
    }

    pub async fn delete_lesson(&self, id: &LessonId) -> Result<()> {
        self.lessons.lock().await.remove(id);
        Ok(())
    }

    pub async fn list_lessons(&self, limit: u32, offset: u32) -> Result<Vec<Lesson>> {
        let lessons = self.lessons.lock().await;
        let lessons_vec: Vec<Lesson> = lessons
            .values()
            .skip(offset as usize)
            .take(limit as usize)
            .map(|model| Lesson::from(model.clone()))
            .collect();
        Ok(lessons_vec)
    }

    // Question operations
    pub async fn create_question(&self, question: &Question) -> Result<()> {
        let question_model = QuestionModel::from(question.clone());
        self.questions
            .lock()
            .await
            .insert(question.id.clone(), question_model);
        Ok(())
    }

    pub async fn find_question_by_id(&self, id: &QuestionId) -> Result<Option<Question>> {
        let questions = self.questions.lock().await;
        Ok(questions.get(id).map(|model| Question::from(model.clone())))
    }

    pub async fn find_questions_by_topic_id(&self, topic_id: &TopicId) -> Result<Vec<Question>> {
        let questions = self.questions.lock().await;
        let questions_vec: Vec<Question> = questions
            .values()
            .filter(|q| q.topic_id == topic_id.0)
            .map(|model| Question::from(model.clone()))
            .collect();
        Ok(questions_vec)
    }

    pub async fn find_questions_by_difficulty(
        &self,
        difficulty: &Difficulty,
    ) -> Result<Vec<Question>> {
        let questions = self.questions.lock().await;
        let questions_vec: Vec<Question> = questions
            .values()
            .filter(|q| q.difficulty == difficulty.to_string())
            .map(|model| Question::from(model.clone()))
            .collect();
        Ok(questions_vec)
    }

    pub async fn update_question(&self, question: &Question) -> Result<()> {
        let question_model = QuestionModel::from(question.clone());
        self.questions
            .lock()
            .await
            .insert(question.id.clone(), question_model);
        Ok(())
    }

    pub async fn delete_question(&self, id: &QuestionId) -> Result<()> {
        self.questions.lock().await.remove(id);
        Ok(())
    }

    pub async fn list_questions(&self, limit: u32, offset: u32) -> Result<Vec<Question>> {
        let questions = self.questions.lock().await;
        let questions_vec: Vec<Question> = questions
            .values()
            .skip(offset as usize)
            .take(limit as usize)
            .map(|model| Question::from(model.clone()))
            .collect();
        Ok(questions_vec)
    }

    // Code Practice operations
    pub async fn create_code_practice(&self, code_practice: &CodePractice) -> Result<()> {
        let code_practice_model = CodePracticeModel::from(code_practice.clone());
        self.code_practices
            .lock()
            .await
            .insert(code_practice.id.clone(), code_practice_model);
        Ok(())
    }

    pub async fn find_code_practice_by_id(
        &self,
        id: &CodePracticeId,
    ) -> Result<Option<CodePractice>> {
        let code_practices = self.code_practices.lock().await;
        Ok(code_practices
            .get(id)
            .map(|model| CodePractice::from(model.clone())))
    }

    pub async fn find_code_practices_by_topic_id(
        &self,
        topic_id: &TopicId,
    ) -> Result<Vec<CodePractice>> {
        let code_practices = self.code_practices.lock().await;
        let code_practices_vec: Vec<CodePractice> = code_practices
            .values()
            .filter(|cp| cp.topic_id == topic_id.0)
            .map(|model| CodePractice::from(model.clone()))
            .collect();
        Ok(code_practices_vec)
    }

    pub async fn find_code_practices_by_lesson_id(
        &self,
        lesson_id: &LessonId,
    ) -> Result<Vec<CodePractice>> {
        let code_practices = self.code_practices.lock().await;
        let code_practices_vec: Vec<CodePractice> = code_practices
            .values()
            .filter(|cp| cp.lesson_id == lesson_id.0)
            .map(|model| CodePractice::from(model.clone()))
            .collect();
        Ok(code_practices_vec)
    }

    pub async fn find_code_practices_by_difficulty(
        &self,
        difficulty: &Difficulty,
    ) -> Result<Vec<CodePractice>> {
        let code_practices = self.code_practices.lock().await;
        let code_practices_vec: Vec<CodePractice> = code_practices
            .values()
            .filter(|cp| cp.difficulty == difficulty.to_string())
            .map(|model| CodePractice::from(model.clone()))
            .collect();
        Ok(code_practices_vec)
    }

    pub async fn update_code_practice(&self, code_practice: &CodePractice) -> Result<()> {
        let code_practice_model = CodePracticeModel::from(code_practice.clone());
        self.code_practices
            .lock()
            .await
            .insert(code_practice.id.clone(), code_practice_model);
        Ok(())
    }

    pub async fn delete_code_practice(&self, id: &CodePracticeId) -> Result<()> {
        self.code_practices.lock().await.remove(id);
        Ok(())
    }

    pub async fn list_code_practices(&self, limit: u32, offset: u32) -> Result<Vec<CodePractice>> {
        let code_practices = self.code_practices.lock().await;
        let code_practices_vec: Vec<CodePractice> = code_practices
            .values()
            .skip(offset as usize)
            .take(limit as usize)
            .map(|model| CodePractice::from(model.clone()))
            .collect();
        Ok(code_practices_vec)
    }

    // User Progress operations
    pub async fn create_user_progress(&self, progress: &UserProgress) -> Result<()> {
        let progress_model = UserProgressModel::from(progress.clone());
        self.user_progress
            .lock()
            .await
            .insert(progress.user_id.clone(), progress_model);
        Ok(())
    }

    pub async fn find_user_progress_by_user_id(
        &self,
        user_id: &UserId,
    ) -> Result<Option<UserProgress>> {
        let user_progress = self.user_progress.lock().await;
        Ok(user_progress
            .get(user_id)
            .map(|model| UserProgress::from(model.clone())))
    }

    pub async fn update_user_progress(&self, progress: &UserProgress) -> Result<()> {
        let progress_model = UserProgressModel::from(progress.clone());
        self.user_progress
            .lock()
            .await
            .insert(progress.user_id.clone(), progress_model);
        Ok(())
    }

    pub async fn delete_user_progress(&self, user_id: &UserId) -> Result<()> {
        self.user_progress.lock().await.remove(user_id);
        Ok(())
    }

    // Leaderboard operations
    pub async fn get_weekly_leaderboard(&self, limit: u32) -> Result<Vec<LeaderboardEntry>> {
        let user_progress = self.user_progress.lock().await;
        let users = self.users.lock().await;

        let mut entries: Vec<LeaderboardEntry> = user_progress
            .values()
            .filter_map(|progress_model| {
                users
                    .get(&UserId::from(progress_model.user_id))
                    .map(|user_model| {
                        let leaderboard_model = LeaderboardEntryModel {
                            user_id: progress_model.user_id,
                            display_name: user_model.display_name.clone(),
                            total_xp: progress_model.total_xp,
                            xp_this_week: progress_model.total_xp, // Simplified - in real implementation this would be calculated
                            rank: 0,                               // Will be set after sorting
                        };
                        LeaderboardEntry::from(leaderboard_model)
                    })
            })
            .collect();

        // Sort by weekly XP (simplified - in real implementation this would be more complex)
        entries.sort_by(|a, b| b.xp_this_week.cmp(&a.xp_this_week));
        entries.truncate(limit as usize);

        Ok(entries)
    }

    pub async fn get_all_time_leaderboard(&self, limit: u32) -> Result<Vec<LeaderboardEntry>> {
        let user_progress = self.user_progress.lock().await;
        let users = self.users.lock().await;

        let mut entries: Vec<LeaderboardEntry> = user_progress
            .values()
            .filter_map(|progress_model| {
                users
                    .get(&UserId::from(progress_model.user_id))
                    .map(|user_model| {
                        let leaderboard_model = LeaderboardEntryModel {
                            user_id: progress_model.user_id,
                            display_name: user_model.display_name.clone(),
                            total_xp: progress_model.total_xp,
                            xp_this_week: progress_model.total_xp, // Simplified - in real implementation this would be calculated
                            rank: 0,                               // Will be set after sorting
                        };
                        LeaderboardEntry::from(leaderboard_model)
                    })
            })
            .collect();

        // Sort by total XP
        entries.sort_by(|a, b| b.total_xp.cmp(&a.total_xp));
        entries.truncate(limit as usize);

        Ok(entries)
    }

    pub async fn get_user_rank(&self, user_id: &UserId) -> Result<Option<u32>> {
        let user_progress = self.user_progress.lock().await;
        let progress = match user_progress.get(user_id) {
            Some(p) => p,
            None => return Ok(None),
        };

        let mut rank = 1;
        for other_progress in user_progress.values() {
            if other_progress.total_xp > progress.total_xp {
                rank += 1;
            }
        }

        Ok(Some(rank))
    }
}
