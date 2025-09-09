#[cfg(test)]
mod tests {
    use crate::domain::entities::*;
    use crate::domain::repositories::*;
    use crate::domain::value_objects::*;
    use crate::infrastructure::database::mock_connection::MockDatabaseConnection;
    use crate::infrastructure::repositories::mock_repositories::*;
    use std::sync::Arc;

    // Mock database connection for testing
    async fn setup_test_db() -> Arc<MockDatabaseConnection> {
        let db = Arc::new(MockDatabaseConnection::new());
        let _ = db.clear_all().await; // Clear any existing data
        db
    }

    #[tokio::test]
    async fn test_user_repository_crud() {
        let db = setup_test_db().await;
        let user_repo = MockUserRepositoryImpl::new(db);

        // Test create
        let user = User::new(
            Email::new("test@example.com".to_string()).unwrap(),
            Password::new("MySecure123!").unwrap(),
            "Test User".to_string(),
        );

        let result = user_repo.create(&user).await;
        assert!(result.is_ok());

        // Test find_by_email
        let found_user = user_repo
            .find_by_email(&Email::new("test@example.com".to_string()).unwrap())
            .await;
        assert!(found_user.is_ok());
        assert_eq!(
            found_user.unwrap().unwrap().email.as_str(),
            "test@example.com"
        );

        // Test find_by_id
        let found_user_by_id = user_repo.find_by_id(&user.id).await;
        assert!(found_user_by_id.is_ok());

        // Test update
        let mut updated_user = user.clone();
        updated_user.display_name = "Updated User".to_string();
        let update_result = user_repo.update(&updated_user).await;
        assert!(update_result.is_ok());

        // Test delete
        let delete_result = user_repo.delete(&user.id).await;
        assert!(delete_result.is_ok());
    }

    #[tokio::test]
    async fn test_topic_repository_crud() {
        let db = setup_test_db().await;
        let topic_repo = MockTopicRepositoryImpl::new(db);

        // Test create
        let topic = Topic::new(
            LocalizedText::new("Rust Basics".to_string(), "Dasar Rust".to_string()),
            LocalizedText::new(
                "Learn the basics of Rust".to_string(),
                "Pelajari dasar-dasar Rust".to_string(),
            ),
            1,
            LocalizedText::new(
                "Programming knowledge".to_string(),
                "Pengetahuan pemrograman".to_string(),
            ),
        );

        let result = topic_repo.create(&topic).await;
        assert!(result.is_ok());

        // Test find_by_id
        let found_topic = topic_repo.find_by_id(&topic.id).await;
        assert!(found_topic.is_ok());

        // Test list
        let topics = topic_repo.list(10, 0).await;
        assert!(topics.is_ok());
        assert!(!topics.unwrap().is_empty());

        // Test list_by_order
        let ordered_topics = topic_repo.list_by_order().await;
        assert!(ordered_topics.is_ok());

        // Test update
        let mut updated_topic = topic.clone();
        updated_topic.title =
            LocalizedText::new("Advanced Rust".to_string(), "Rust Lanjutan".to_string());
        let update_result = topic_repo.update(&updated_topic).await;
        assert!(update_result.is_ok());

        // Test delete
        let delete_result = topic_repo.delete(&topic.id).await;
        assert!(delete_result.is_ok());
    }

    #[tokio::test]
    async fn test_lesson_repository_crud() {
        let db = setup_test_db().await;
        let lesson_repo = MockLessonRepositoryImpl::new(db);

        // Create a topic first
        let topic = Topic::new(
            LocalizedText::new("Rust Basics".to_string(), "Dasar Rust".to_string()),
            LocalizedText::new(
                "Learn the basics of Rust".to_string(),
                "Pelajari dasar-dasar Rust".to_string(),
            ),
            1,
            LocalizedText::new(
                "Programming knowledge".to_string(),
                "Pengetahuan pemrograman".to_string(),
            ),
        );

        // Test create
        let lesson = Lesson::new(
            LocalizedText::new("Variables".to_string(), "Variabel".to_string()),
            topic.id.clone(),
            LocalizedText::new(
                "Learn about variables".to_string(),
                "Pelajari tentang variabel".to_string(),
            ),
            "https://example.com".to_string(),
            1,
        );

        let result = lesson_repo.create(&lesson).await;
        assert!(result.is_ok());

        // Test find_by_id
        let found_lesson = lesson_repo.find_by_id(&lesson.id).await;
        assert!(found_lesson.is_ok());

        // Test find_by_topic_id
        let lessons_by_topic = lesson_repo.find_by_topic_id(&topic.id).await;
        assert!(lessons_by_topic.is_ok());

        // Test list
        let lessons = lesson_repo.list(10, 0).await;
        assert!(lessons.is_ok());

        // Test update
        let mut updated_lesson = lesson.clone();
        updated_lesson.title = LocalizedText::new(
            "Advanced Variables".to_string(),
            "Variabel Lanjutan".to_string(),
        );
        let update_result = lesson_repo.update(&updated_lesson).await;
        assert!(update_result.is_ok());

        // Test delete
        let delete_result = lesson_repo.delete(&lesson.id).await;
        assert!(delete_result.is_ok());
    }

    #[tokio::test]
    async fn test_question_repository_crud() {
        let db = setup_test_db().await;
        let question_repo = MockQuestionRepositoryImpl::new(db);

        // Create a topic first
        let topic = Topic::new(
            LocalizedText::new("Rust Basics".to_string(), "Dasar Rust".to_string()),
            LocalizedText::new(
                "Learn the basics of Rust".to_string(),
                "Pelajari dasar-dasar Rust".to_string(),
            ),
            1,
            LocalizedText::new(
                "Programming knowledge".to_string(),
                "Pengetahuan pemrograman".to_string(),
            ),
        );

        // Test create
        let question = Question::new(
            LocalizedText::new(
                "What is a variable?".to_string(),
                "Apa itu variabel?".to_string(),
            ),
            topic.id.clone(),
            Difficulty::Beginner,
            Points::new(10),
            QuestionType::MultipleChoice {
                choices: LocalizedText::new(
                    "A variable stores data".to_string(),
                    "Variabel menyimpan data".to_string(),
                ),
                correct_index: 0,
            },
        );

        let result = question_repo.create(&question).await;
        assert!(result.is_ok());

        // Test find_by_id
        let found_question = question_repo.find_by_id(&question.id).await;
        assert!(found_question.is_ok());

        // Test find_by_topic_id
        let questions_by_topic = question_repo.find_by_topic_id(&topic.id).await;
        assert!(questions_by_topic.is_ok());

        // Test find_by_difficulty
        let questions_by_difficulty = question_repo
            .find_by_difficulty(&Difficulty::Beginner)
            .await;
        assert!(questions_by_difficulty.is_ok());

        // Test list
        let questions = question_repo.list(10, 0).await;
        assert!(questions.is_ok());

        // Test update
        let mut updated_question = question.clone();
        updated_question.points = Points::new(20);
        let update_result = question_repo.update(&updated_question).await;
        assert!(update_result.is_ok());

        // Test delete
        let delete_result = question_repo.delete(&question.id).await;
        assert!(delete_result.is_ok());
    }

    #[tokio::test]
    async fn test_code_practice_repository_crud() {
        let db = setup_test_db().await;
        let code_practice_repo = MockCodePracticeRepositoryImpl::new(db);

        // Create a topic and lesson first
        let topic = Topic::new(
            LocalizedText::new("Rust Basics".to_string(), "Dasar Rust".to_string()),
            LocalizedText::new(
                "Learn the basics of Rust".to_string(),
                "Pelajari dasar-dasar Rust".to_string(),
            ),
            1,
            LocalizedText::new(
                "Programming knowledge".to_string(),
                "Pengetahuan pemrograman".to_string(),
            ),
        );

        let lesson = Lesson::new(
            LocalizedText::new("Variables".to_string(), "Variabel".to_string()),
            topic.id.clone(),
            LocalizedText::new(
                "Learn about variables".to_string(),
                "Pelajari tentang variabel".to_string(),
            ),
            "https://example.com".to_string(),
            1,
        );

        // Test create
        let code_practice = CodePractice::new(
            LocalizedText::new("Create a variable".to_string(), "Buat variabel".to_string()),
            LocalizedText::new(
                "Create a variable and assign a value".to_string(),
                "Buat variabel dan beri nilai".to_string(),
            ),
            "let x = 5;".to_string(),
            "let x = 5;".to_string(),
            Difficulty::Beginner,
            "variables".to_string(),
            lesson.id.clone(),
            topic.id.clone(),
            Points::new(15),
        );

        let result = code_practice_repo.create(&code_practice).await;
        assert!(result.is_ok());

        // Test find_by_id
        let found_practice = code_practice_repo.find_by_id(&code_practice.id).await;
        assert!(found_practice.is_ok());

        // Test find_by_topic_id
        let practices_by_topic = code_practice_repo.find_by_topic_id(&topic.id).await;
        assert!(practices_by_topic.is_ok());

        // Test find_by_lesson_id
        let practices_by_lesson = code_practice_repo.find_by_lesson_id(&lesson.id).await;
        assert!(practices_by_lesson.is_ok());

        // Test find_by_difficulty
        let practices_by_difficulty = code_practice_repo
            .find_by_difficulty(&Difficulty::Beginner)
            .await;
        assert!(practices_by_difficulty.is_ok());

        // Test list
        let practices = code_practice_repo.list(10, 0).await;
        assert!(practices.is_ok());

        // Test update
        let mut updated_practice = code_practice.clone();
        updated_practice.points = Points::new(25);
        let update_result = code_practice_repo.update(&updated_practice).await;
        assert!(update_result.is_ok());

        // Test delete
        let delete_result = code_practice_repo.delete(&code_practice.id).await;
        assert!(delete_result.is_ok());
    }

    #[tokio::test]
    async fn test_user_progress_repository_crud() {
        let db = setup_test_db().await;
        let progress_repo = MockUserProgressRepositoryImpl::new(db);

        // Create a user first
        let user = User::new(
            Email::new("test@example.com".to_string()).unwrap(),
            Password::new("MySecure123!").unwrap(),
            "Test User".to_string(),
        );

        // Test create
        let progress = UserProgress::new(user.id.clone());

        let result = progress_repo.create(&progress).await;
        assert!(result.is_ok());

        // Test find_by_user_id
        let found_progress = progress_repo.find_by_user_id(&user.id).await;
        assert!(found_progress.is_ok());

        // Test update
        let mut updated_progress = progress.clone();
        updated_progress.total_xp = Points::new(200);
        updated_progress.current_streak_days = 5;
        let update_result = progress_repo.update(&updated_progress).await;
        assert!(update_result.is_ok());

        // Test delete
        let delete_result = progress_repo.delete(&user.id).await;
        assert!(delete_result.is_ok());
    }

    #[tokio::test]
    async fn test_leaderboard_repository() {
        let db = setup_test_db().await;
        let leaderboard_repo = MockLeaderboardRepositoryImpl::new(db);

        // Test get_weekly_leaderboard
        let weekly_leaderboard = leaderboard_repo.get_weekly_leaderboard(10).await;
        assert!(weekly_leaderboard.is_ok());

        // Test get_all_time_leaderboard
        let all_time_leaderboard = leaderboard_repo.get_all_time_leaderboard(10).await;
        assert!(all_time_leaderboard.is_ok());

        // Test get_user_rank
        let user_id = UserId::new();
        let user_rank = leaderboard_repo.get_user_rank(&user_id).await;
        assert!(user_rank.is_ok());
    }
}
