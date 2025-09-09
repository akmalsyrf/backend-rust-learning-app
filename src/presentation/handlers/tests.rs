#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::state::AppState;
    use crate::domain::entities::*;
    use crate::domain::value_objects::*;
    use crate::domain::services::*;
    use crate::domain::repositories::*;
    use crate::application::use_cases::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        Router,
    };
    use tower::ServiceExt;
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
        UserProgressRepository {}

        #[async_trait]
        impl UserProgressRepository for UserProgressRepository {
            async fn create(&self, progress: &UserProgress) -> anyhow::Result<()>;
            async fn find_by_user_id(&self, user_id: &UserId) -> anyhow::Result<Option<UserProgress>>;
            async fn update(&self, progress: &UserProgress) -> anyhow::Result<()>;
            async fn delete(&self, user_id: &UserId) -> anyhow::Result<()>;
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

    mock! {
        ProgressService {}

        #[async_trait]
        impl ProgressService for ProgressService {
            async fn submit_question_result(&self, user_id: &UserId, question_id: &QuestionId, result: QuestionResult) -> anyhow::Result<()>;
            async fn complete_code_practice(&self, user_id: &UserId, code_practice_id: &CodePracticeId, completion: CompletedCodePractice) -> anyhow::Result<()>;
            async fn update_user_progress(&self, user_id: &UserId, progress: &UserProgress) -> anyhow::Result<()>;
        }
    }

    mock! {
        LeaderboardService {}

        #[async_trait]
        impl LeaderboardService for LeaderboardService {
            async fn get_weekly_leaderboard(&self, limit: u32) -> anyhow::Result<Vec<LeaderboardEntry>>;
            async fn get_all_time_leaderboard(&self, limit: u32) -> anyhow::Result<Vec<LeaderboardEntry>>;
            async fn get_user_rank(&self, user_id: &UserId) -> anyhow::Result<Option<u32>>;
        }
    }

    // Helper function to create test app state
    fn create_test_app_state() -> AppState {
        let auth_service = Arc::new(MockAuthService::new());
        let topic_repository = Arc::new(MockTopicRepository::new());
        let lesson_repository = Arc::new(MockLessonRepository::new());
        let question_repository = Arc::new(MockQuestionRepository::new());
        let code_practice_repository = Arc::new(MockCodePracticeRepository::new());
        let user_progress_repository = Arc::new(MockUserProgressRepository::new());
        let leaderboard_repository = Arc::new(MockLeaderboardRepository::new());
        let progress_service = Arc::new(MockProgressService::new());
        let leaderboard_service = Arc::new(MockLeaderboardService::new());

        AppState::new(
            topic_repository,
            lesson_repository,
            question_repository,
            code_practice_repository,
            user_progress_repository,
            leaderboard_repository,
            auth_service,
            progress_service,
            leaderboard_service,
        )
    }

    // Helper function to create test router
    fn create_test_router() -> Router {
        let app_state = create_test_app_state();
        Router::new()
            .route("/api/auth/register", axum::routing::post(register_handler))
            .route("/api/auth/login", axum::routing::post(login_handler))
            .route("/api/topics", axum::routing::get(list_topics_handler))
            .route("/api/lessons", axum::routing::get(list_lessons_handler))
            .route("/api/questions", axum::routing::get(list_questions_handler))
            .route("/api/code-practices", axum::routing::get(list_code_practices_handler))
            .route("/api/leaderboard", axum::routing::get(get_leaderboard_handler))
            .with_state(app_state)
    }

    // Auth Handler Tests
    #[tokio::test]
    async fn test_register_handler_success() {
        let mut mock_auth_service = MockAuthService::new();
        let test_user = User::new(
            Email::new("test@example.com".to_string()).unwrap(),
            Password::new("MySecure123!").unwrap(),
            "Test User".to_string(),
        );
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

        let app_state = AppState::new(
            Arc::new(MockTopicRepository::new()),
            Arc::new(MockLessonRepository::new()),
            Arc::new(MockQuestionRepository::new()),
            Arc::new(MockCodePracticeRepository::new()),
            Arc::new(MockUserProgressRepository::new()),
            Arc::new(MockLeaderboardRepository::new()),
            Arc::new(mock_auth_service),
            Arc::new(MockProgressService::new()),
            Arc::new(MockLeaderboardService::new()),
        );

        let router = Router::new()
            .route("/api/auth/register", axum::routing::post(register_handler))
            .with_state(app_state);

        let request_body = serde_json::json!({
            "email": "test@example.com",
            "password": "password123",
            "display_name": "Test User"
        });

        let request = Request::builder()
            .method("POST")
            .uri("/api/auth/register")
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_vec(&request_body).unwrap()))
            .unwrap();

        let response = router.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_register_handler_invalid_email() {
        let app_state = create_test_app_state();
        let router = Router::new()
            .route("/api/auth/register", axum::routing::post(register_handler))
            .with_state(app_state);

        let request_body = serde_json::json!({
            "email": "invalid-email",
            "password": "password123",
            "display_name": "Test User"
        });

        let request = Request::builder()
            .method("POST")
            .uri("/api/auth/register")
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_vec(&request_body).unwrap()))
            .unwrap();

        let response = router.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_login_handler_success() {
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

        let app_state = AppState::new(
            Arc::new(MockTopicRepository::new()),
            Arc::new(MockLessonRepository::new()),
            Arc::new(MockQuestionRepository::new()),
            Arc::new(MockCodePracticeRepository::new()),
            Arc::new(MockUserProgressRepository::new()),
            Arc::new(MockLeaderboardRepository::new()),
            Arc::new(mock_auth_service),
            Arc::new(MockProgressService::new()),
            Arc::new(MockLeaderboardService::new()),
        );

        let router = Router::new()
            .route("/api/auth/login", axum::routing::post(login_handler))
            .with_state(app_state);

        let request_body = serde_json::json!({
            "email": "test@example.com",
            "password": "password123"
        });

        let request = Request::builder()
            .method("POST")
            .uri("/api/auth/login")
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_vec(&request_body).unwrap()))
            .unwrap();

        let response = router.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    // Content Handler Tests
    #[tokio::test]
    async fn test_list_topics_handler_success() {
        let mut mock_topic_repo = MockTopicRepository::new();
        let test_topic = Topic::new(
            LocalizedText::new("Rust Basics".to_string(), "Dasar Rust".to_string()),
            LocalizedText::new("Learn the basics of Rust".to_string(), "Pelajari dasar-dasar Rust".to_string()),
            1,
            LocalizedText::new("Programming knowledge".to_string(), "Pengetahuan pemrograman".to_string()),
        );

        mock_topic_repo
            .expect_list()
            .times(1)
            .returning(move |_, _| Ok(vec![test_topic.clone()]));

        let app_state = AppState::new(
            Arc::new(mock_topic_repo),
            Arc::new(MockLessonRepository::new()),
            Arc::new(MockQuestionRepository::new()),
            Arc::new(MockCodePracticeRepository::new()),
            Arc::new(MockUserProgressRepository::new()),
            Arc::new(MockLeaderboardRepository::new()),
            Arc::new(MockAuthService::new()),
            Arc::new(MockProgressService::new()),
            Arc::new(MockLeaderboardService::new()),
        );

        let router = Router::new()
            .route("/api/topics", axum::routing::get(list_topics_handler))
            .with_state(app_state);

        let request = Request::builder()
            .method("GET")
            .uri("/api/topics")
            .body(Body::empty())
            .unwrap();

        let response = router.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_list_lessons_handler_success() {
        let mut mock_lesson_repo = MockLessonRepository::new();
        let test_topic_id = TopicId::new();
        let test_lesson = Lesson::new(
            LocalizedText::new("Variables".to_string(), "Variabel".to_string()),
            test_topic_id.clone(),
            LocalizedText::new("Learn about variables".to_string(), "Pelajari tentang variabel".to_string()),
            "https://example.com".to_string(),
            1,
        );

        mock_lesson_repo
            .expect_find_by_topic_id()
            .times(1)
            .returning(move |_| Ok(vec![test_lesson.clone()]));

        let app_state = AppState::new(
            Arc::new(MockTopicRepository::new()),
            Arc::new(mock_lesson_repo),
            Arc::new(MockQuestionRepository::new()),
            Arc::new(MockCodePracticeRepository::new()),
            Arc::new(MockUserProgressRepository::new()),
            Arc::new(MockLeaderboardRepository::new()),
            Arc::new(MockAuthService::new()),
            Arc::new(MockProgressService::new()),
            Arc::new(MockLeaderboardService::new()),
        );

        let router = Router::new()
            .route("/api/lessons", axum::routing::get(list_lessons_handler))
            .with_state(app_state);

        let request = Request::builder()
            .method("GET")
            .uri("/api/lessons?topic_id=123")
            .body(Body::empty())
            .unwrap();

        let response = router.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_list_questions_handler_success() {
        let mut mock_question_repo = MockQuestionRepository::new();
        let test_topic_id = TopicId::new();
        let test_question = Question::new(
            LocalizedText::new("What is a variable?".to_string(), "Apa itu variabel?".to_string()),
            Some(LocalizedText::new("A variable stores data".to_string(), "Variabel menyimpan data".to_string())),
            test_topic_id.clone(),
            Difficulty::Beginner,
            Points::new(10),
            QuestionType::MultipleChoice,
        );

        mock_question_repo
            .expect_find_by_topic_id()
            .times(1)
            .returning(move |_| Ok(vec![test_question.clone()]));

        let app_state = AppState::new(
            Arc::new(MockTopicRepository::new()),
            Arc::new(MockLessonRepository::new()),
            Arc::new(mock_question_repo),
            Arc::new(MockCodePracticeRepository::new()),
            Arc::new(MockUserProgressRepository::new()),
            Arc::new(MockLeaderboardRepository::new()),
            Arc::new(MockAuthService::new()),
            Arc::new(MockProgressService::new()),
            Arc::new(MockLeaderboardService::new()),
        );

        let router = Router::new()
            .route("/api/questions", axum::routing::get(list_questions_handler))
            .with_state(app_state);

        let request = Request::builder()
            .method("GET")
            .uri("/api/questions?topic_id=123")
            .body(Body::empty())
            .unwrap();

        let response = router.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_list_code_practices_handler_success() {
        let mut mock_code_practice_repo = MockCodePracticeRepository::new();
        let test_topic_id = TopicId::new();
        let test_lesson_id = LessonId::new();
        let test_code_practice = CodePractice::new(
            LocalizedText::new("Create a variable".to_string(), "Buat variabel".to_string()),
            LocalizedText::new("Create a variable and assign a value".to_string(), "Buat variabel dan beri nilai".to_string()),
            "let x = 5;".to_string(),
            Some("5".to_string()),
            "let x = 5;".to_string(),
            vec![LocalizedText::new("Use let keyword".to_string(), "Gunakan kata kunci let".to_string())],
            Difficulty::Beginner,
            "variables".to_string(),
            test_lesson_id.clone(),
            test_topic_id.clone(),
            Points::new(15),
        );

        mock_code_practice_repo
            .expect_find_by_topic_id()
            .times(1)
            .returning(move |_| Ok(vec![test_code_practice.clone()]));

        let app_state = AppState::new(
            Arc::new(MockTopicRepository::new()),
            Arc::new(MockLessonRepository::new()),
            Arc::new(MockQuestionRepository::new()),
            Arc::new(mock_code_practice_repo),
            Arc::new(MockUserProgressRepository::new()),
            Arc::new(MockLeaderboardRepository::new()),
            Arc::new(MockAuthService::new()),
            Arc::new(MockProgressService::new()),
            Arc::new(MockLeaderboardService::new()),
        );

        let router = Router::new()
            .route("/api/code-practices", axum::routing::get(list_code_practices_handler))
            .with_state(app_state);

        let request = Request::builder()
            .method("GET")
            .uri("/api/code-practices?topic_id=123")
            .body(Body::empty())
            .unwrap();

        let response = router.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    // Progress Handler Tests
    #[tokio::test]
    async fn test_get_leaderboard_handler_success() {
        let mut mock_leaderboard_repo = MockLeaderboardRepository::new();
        let test_entry = LeaderboardEntry::new(
            UserId::new(),
            "Test User".to_string(),
            Points::new(100),
            Points::new(500),
            1,
        );

        mock_leaderboard_repo
            .expect_get_all_time_leaderboard()
            .times(1)
            .returning(move |_| Ok(vec![test_entry.clone()]));

        let app_state = AppState::new(
            Arc::new(MockTopicRepository::new()),
            Arc::new(MockLessonRepository::new()),
            Arc::new(MockQuestionRepository::new()),
            Arc::new(MockCodePracticeRepository::new()),
            Arc::new(MockUserProgressRepository::new()),
            Arc::new(mock_leaderboard_repo),
            Arc::new(MockAuthService::new()),
            Arc::new(MockProgressService::new()),
            Arc::new(MockLeaderboardService::new()),
        );

        let router = Router::new()
            .route("/api/leaderboard", axum::routing::get(get_leaderboard_handler))
            .with_state(app_state);

        let request = Request::builder()
            .method("GET")
            .uri("/api/leaderboard?limit=10")
            .body(Body::empty())
            .unwrap();

        let response = router.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    // Error handling tests
    #[tokio::test]
    async fn test_handlers_database_error() {
        let mut mock_topic_repo = MockTopicRepository::new();

        mock_topic_repo
            .expect_list()
            .times(1)
            .returning(move |_, _| Err(anyhow::anyhow!("Database connection failed")));

        let app_state = AppState::new(
            Arc::new(mock_topic_repo),
            Arc::new(MockLessonRepository::new()),
            Arc::new(MockQuestionRepository::new()),
            Arc::new(MockCodePracticeRepository::new()),
            Arc::new(MockUserProgressRepository::new()),
            Arc::new(MockLeaderboardRepository::new()),
            Arc::new(MockAuthService::new()),
            Arc::new(MockProgressService::new()),
            Arc::new(MockLeaderboardService::new()),
        );

        let router = Router::new()
            .route("/api/topics", axum::routing::get(list_topics_handler))
            .with_state(app_state);

        let request = Request::builder()
            .method("GET")
            .uri("/api/topics")
            .body(Body::empty())
            .unwrap();

        let response = router.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_handlers_invalid_json() {
        let app_state = create_test_app_state();
        let router = Router::new()
            .route("/api/auth/register", axum::routing::post(register_handler))
            .with_state(app_state);

        let request = Request::builder()
            .method("POST")
            .uri("/api/auth/register")
            .header("content-type", "application/json")
            .body(Body::from("invalid json"))
            .unwrap();

        let response = router.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
