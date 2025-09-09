#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::*;
    use crate::domain::value_objects::*;
    use crate::domain::services::*;
    use crate::domain::repositories::*;
    use std::sync::Arc;
    use mockall::mock;

    // Mock implementations for testing
    mock! {
        AuthService {}

        #[async_trait]
        impl AuthService for AuthService {
            async fn register(&self, email: Email, password: Password, display_name: String) -> anyhow::Result<User>;
            async fn login(&self, email: Email, password: Password) -> anyhow::Result<AuthToken>;
            async fn validate_token(&self, token: &str) -> anyhow::Result<UserId>;
        }
    }

    mock! {
        TopicRepository {}

        #[async_trait]
        impl TopicRepository for TopicRepository {
            async fn create(&self, topic: &Topic) -> anyhow::Result<()>;
            async fn find_by_id(&self, id: &TopicId) -> anyhow::Result<Option<Topic>>;
            async fn update(&self, topic: &Topic) -> anyhow::Result<()>;
            async fn delete(&self, id: &TopicId) -> anyhow::Result<()>;
            async fn list(&self, limit: u32, offset: u32) -> anyhow::Result<Vec<Topic>>;
            async fn list_by_order(&self) -> anyhow::Result<Vec<Topic>>;
        }
    }

    mock! {
        LessonRepository {}

        #[async_trait]
        impl LessonRepository for LessonRepository {
            async fn create(&self, lesson: &Lesson) -> anyhow::Result<()>;
            async fn find_by_id(&self, id: &LessonId) -> anyhow::Result<Option<Lesson>>;
            async fn find_by_topic_id(&self, topic_id: &TopicId) -> anyhow::Result<Vec<Lesson>>;
            async fn update(&self, lesson: &Lesson) -> anyhow::Result<()>;
            async fn delete(&self, id: &LessonId) -> anyhow::Result<()>;
            async fn list(&self, limit: u32, offset: u32) -> anyhow::Result<Vec<Lesson>>;
        }
    }

    mock! {
        QuestionRepository {}

        #[async_trait]
        impl QuestionRepository for QuestionRepository {
            async fn create(&self, question: &Question) -> anyhow::Result<()>;
            async fn find_by_id(&self, id: &QuestionId) -> anyhow::Result<Option<Question>>;
            async fn find_by_topic_id(&self, topic_id: &TopicId) -> anyhow::Result<Vec<Question>>;
            async fn find_by_difficulty(&self, difficulty: &Difficulty) -> anyhow::Result<Vec<Question>>;
            async fn update(&self, question: &Question) -> anyhow::Result<()>;
            async fn delete(&self, id: &QuestionId) -> anyhow::Result<()>;
            async fn list(&self, limit: u32, offset: u32) -> anyhow::Result<Vec<Question>>;
        }
    }

    mock! {
        CodePracticeRepository {}

        #[async_trait]
        impl CodePracticeRepository for CodePracticeRepository {
            async fn create(&self, code_practice: &CodePractice) -> anyhow::Result<()>;
            async fn find_by_id(&self, id: &CodePracticeId) -> anyhow::Result<Option<CodePractice>>;
            async fn find_by_topic_id(&self, topic_id: &TopicId) -> anyhow::Result<Vec<CodePractice>>;
            async fn find_by_lesson_id(&self, lesson_id: &LessonId) -> anyhow::Result<Vec<CodePractice>>;
            async fn find_by_difficulty(&self, difficulty: &Difficulty) -> anyhow::Result<Vec<CodePractice>>;
            async fn update(&self, code_practice: &CodePractice) -> anyhow::Result<()>;
            async fn delete(&self, id: &CodePracticeId) -> anyhow::Result<()>;
            async fn list(&self, limit: u32, offset: u32) -> anyhow::Result<Vec<CodePractice>>;
        }
    }

    mock! {
        LeaderboardRepository {}

        #[async_trait]
        impl LeaderboardRepository for LeaderboardRepository {
            async fn get_weekly_leaderboard(&self, limit: u32) -> anyhow::Result<Vec<LeaderboardEntry>>;
            async fn get_all_time_leaderboard(&self, limit: u32) -> anyhow::Result<Vec<LeaderboardEntry>>;
            async fn get_user_rank(&self, user_id: &UserId) -> anyhow::Result<Option<u32>>;
        }
    }

    // Helper function to create test data
    fn create_test_topic() -> Topic {
        Topic::new(
            LocalizedText::new("Rust Basics".to_string(), "Dasar Rust".to_string()),
            LocalizedText::new("Learn the basics of Rust".to_string(), "Pelajari dasar-dasar Rust".to_string()),
            1,
            LocalizedText::new("Programming knowledge".to_string(), "Pengetahuan pemrograman".to_string()),
        )
    }

    fn create_test_lesson(topic_id: TopicId) -> Lesson {
        Lesson::new(
            LocalizedText::new("Variables".to_string(), "Variabel".to_string()),
            topic_id,
            LocalizedText::new("Learn about variables".to_string(), "Pelajari tentang variabel".to_string()),
            "https://example.com".to_string(),
            1,
        )
    }

    fn create_test_question(topic_id: TopicId) -> Question {
        Question::new(
            LocalizedText::new("What is a variable?".to_string(), "Apa itu variabel?".to_string()),
            Some(LocalizedText::new("A variable stores data".to_string(), "Variabel menyimpan data".to_string())),
            topic_id,
            Difficulty::Beginner,
            Points::new(10),
            QuestionType::MultipleChoice,
        )
    }

    fn create_test_code_practice(topic_id: TopicId, lesson_id: LessonId) -> CodePractice {
        CodePractice::new(
            LocalizedText::new("Create a variable".to_string(), "Buat variabel".to_string()),
            LocalizedText::new("Create a variable and assign a value".to_string(), "Buat variabel dan beri nilai".to_string()),
            "let x = 5;".to_string(),
            Some("5".to_string()),
            "let x = 5;".to_string(),
            vec![LocalizedText::new("Use let keyword".to_string(), "Gunakan kata kunci let".to_string())],
            Difficulty::Beginner,
            "variables".to_string(),
            lesson_id,
            topic_id,
            Points::new(15),
        )
    }

    fn create_test_user() -> User {
        User::new(
            Email::new("test@example.com".to_string()).unwrap(),
            Password::new("MySecure123!").unwrap(),
            "Test User".to_string(),
        )
    }

    fn create_test_leaderboard_entry() -> LeaderboardEntry {
        LeaderboardEntry::new(
            UserId::new(),
            "Test User".to_string(),
            Points::new(100),
            Points::new(500),
            1,
        )
    }

    // Auth Use Cases Tests
    #[tokio::test]
    async fn test_auth_use_cases_register() {
        let mut mock_auth_service = MockAuthService::new();
        let test_user = create_test_user();
        let test_token = AuthToken {
            access_token: "test_token".to_string(),
            token_type: "Bearer".to_string(),
            expires_in: 3600,
        };

        mock_auth_service
            .expect_register()
            .times(1)
            .returning(move |_, _, _| Ok(test_user.clone()));

        mock_auth_service
            .expect_login()
            .times(1)
            .returning(move |_, _| Ok(test_token.clone()));

        let auth_use_cases = AuthUseCases::new(Arc::new(mock_auth_service));

        let register_request = RegisterRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            display_name: "Test User".to_string(),
        };

        let result = auth_use_cases.register(register_request).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.token_type, "Bearer");
        assert_eq!(response.user.email.as_str(), "test@example.com");
    }

    #[tokio::test]
    async fn test_auth_use_cases_login() {
        let mut mock_auth_service = MockAuthService::new();
        let test_token = AuthToken {
            access_token: "test_token".to_string(),
            token_type: "Bearer".to_string(),
            expires_in: 3600,
        };

        mock_auth_service
            .expect_login()
            .times(1)
            .returning(move |_, _| Ok(test_token.clone()));

        let auth_use_cases = AuthUseCases::new(Arc::new(mock_auth_service));

        let login_request = LoginRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };

        let result = auth_use_cases.login(login_request).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.token_type, "Bearer");
        assert_eq!(response.user.email, "test@example.com");
    }

    // Content Use Cases Tests
    #[tokio::test]
    async fn test_content_use_cases_get_topics() {
        let mut mock_topic_repo = MockTopicRepository::new();
        let test_topic = create_test_topic();

        mock_topic_repo
            .expect_list()
            .times(1)
            .returning(move |_, _| Ok(vec![test_topic.clone()]));

        let content_use_cases = ContentUseCases::new(
            Arc::new(mock_topic_repo),
            Arc::new(MockLessonRepository::new()),
            Arc::new(MockQuestionRepository::new()),
            Arc::new(MockCodePracticeRepository::new()),
        );

        let result = content_use_cases.get_topics().await;
        assert!(result.is_ok());

        let topics = result.unwrap();
        assert_eq!(topics.len(), 1);
        assert_eq!(topics[0].title.en, "Rust Basics");
    }

    #[tokio::test]
    async fn test_content_use_cases_get_lessons() {
        let mut mock_lesson_repo = MockLessonRepository::new();
        let test_topic = create_test_topic();
        let test_lesson = create_test_lesson(test_topic.id.clone());

        mock_lesson_repo
            .expect_find_by_topic_id()
            .times(1)
            .returning(move |_| Ok(vec![test_lesson.clone()]));

        let content_use_cases = ContentUseCases::new(
            Arc::new(MockTopicRepository::new()),
            Arc::new(mock_lesson_repo),
            Arc::new(MockQuestionRepository::new()),
            Arc::new(MockCodePracticeRepository::new()),
        );

        let result = content_use_cases.get_lessons(Some(test_topic.id.to_string())).await;
        assert!(result.is_ok());

        let lessons = result.unwrap();
        assert_eq!(lessons.len(), 1);
        assert_eq!(lessons[0].title.en, "Variables");
    }

    #[tokio::test]
    async fn test_content_use_cases_get_questions() {
        let mut mock_question_repo = MockQuestionRepository::new();
        let test_topic = create_test_topic();
        let test_question = create_test_question(test_topic.id.clone());

        mock_question_repo
            .expect_find_by_topic_id()
            .times(1)
            .returning(move |_| Ok(vec![test_question.clone()]));

        let content_use_cases = ContentUseCases::new(
            Arc::new(MockTopicRepository::new()),
            Arc::new(MockLessonRepository::new()),
            Arc::new(mock_question_repo),
            Arc::new(MockCodePracticeRepository::new()),
        );

        let result = content_use_cases.get_questions(Some(test_topic.id.to_string()), None).await;
        assert!(result.is_ok());

        let questions = result.unwrap();
        assert_eq!(questions.len(), 1);
        assert_eq!(questions[0].prompt.en, "What is a variable?");
    }

    #[tokio::test]
    async fn test_content_use_cases_get_code_practices() {
        let mut mock_code_practice_repo = MockCodePracticeRepository::new();
        let test_topic = create_test_topic();
        let test_lesson = create_test_lesson(test_topic.id.clone());
        let test_code_practice = create_test_code_practice(test_topic.id.clone(), test_lesson.id.clone());

        mock_code_practice_repo
            .expect_find_by_topic_id()
            .times(1)
            .returning(move |_| Ok(vec![test_code_practice.clone()]));

        let content_use_cases = ContentUseCases::new(
            Arc::new(MockTopicRepository::new()),
            Arc::new(MockLessonRepository::new()),
            Arc::new(MockQuestionRepository::new()),
            Arc::new(mock_code_practice_repo),
        );

        let result = content_use_cases.get_code_practices(Some(test_topic.id.to_string()), None).await;
        assert!(result.is_ok());

        let code_practices = result.unwrap();
        assert_eq!(code_practices.len(), 1);
        assert_eq!(code_practices[0].title.en, "Create a variable");
    }

    // Progress Use Cases Tests
    #[tokio::test]
    async fn test_progress_use_cases_get_leaderboard() {
        let mut mock_leaderboard_repo = MockLeaderboardRepository::new();
        let test_entry = create_test_leaderboard_entry();

        mock_leaderboard_repo
            .expect_get_all_time_leaderboard()
            .times(1)
            .returning(move |_| Ok(vec![test_entry.clone()]));

        let progress_use_cases = ProgressUseCases::new(Arc::new(mock_leaderboard_repo));

        let result = progress_use_cases.get_leaderboard(Some(10)).await;
        assert!(result.is_ok());

        let leaderboard = result.unwrap();
        assert_eq!(leaderboard.entries.len(), 1);
        assert_eq!(leaderboard.entries[0].display_name, "Test User");
    }

    #[tokio::test]
    async fn test_progress_use_cases_get_user_rank() {
        let mut mock_leaderboard_repo = MockLeaderboardRepository::new();

        mock_leaderboard_repo
            .expect_get_user_rank()
            .times(1)
            .returning(move |_| Ok(Some(1)));

        let progress_use_cases = ProgressUseCases::new(Arc::new(mock_leaderboard_repo));

        let result = progress_use_cases.get_user_rank("test-user-id").await;
        assert!(result.is_ok());

        let rank = result.unwrap();
        assert_eq!(rank, Some(1));
    }

    // Error handling tests
    #[tokio::test]
    async fn test_auth_use_cases_register_invalid_email() {
        let mock_auth_service = MockAuthService::new();
        let auth_use_cases = AuthUseCases::new(Arc::new(mock_auth_service));

        let register_request = RegisterRequest {
            email: "invalid-email".to_string(),
            password: "password123".to_string(),
            display_name: "Test User".to_string(),
        };

        let result = auth_use_cases.register(register_request).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_content_use_cases_get_topics_error() {
        let mut mock_topic_repo = MockTopicRepository::new();

        mock_topic_repo
            .expect_list()
            .times(1)
            .returning(move |_, _| Err(anyhow::anyhow!("Database error")));

        let content_use_cases = ContentUseCases::new(
            Arc::new(mock_topic_repo),
            Arc::new(MockLessonRepository::new()),
            Arc::new(MockQuestionRepository::new()),
            Arc::new(MockCodePracticeRepository::new()),
        );

        let result = content_use_cases.get_topics().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_progress_use_cases_get_leaderboard_error() {
        let mut mock_leaderboard_repo = MockLeaderboardRepository::new();

        mock_leaderboard_repo
            .expect_get_all_time_leaderboard()
            .times(1)
            .returning(move |_| Err(anyhow::anyhow!("Database error")));

        let progress_use_cases = ProgressUseCases::new(Arc::new(mock_leaderboard_repo));

        let result = progress_use_cases.get_leaderboard(Some(10)).await;
        assert!(result.is_err());
    }
}
