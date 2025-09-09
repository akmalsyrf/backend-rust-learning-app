#[cfg(test)]
mod tests {
    use crate::domain::entities::*;
    use crate::domain::value_objects::*;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    // Simple in-memory mock database for testing
    struct MockDatabase {
        users: Arc<Mutex<HashMap<String, User>>>,
        topics: Arc<Mutex<HashMap<String, Topic>>>,
    }

    impl MockDatabase {
        fn new() -> Self {
            Self {
                users: Arc::new(Mutex::new(HashMap::new())),
                topics: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        fn create_user(&self, user: User) -> Result<(), String> {
            let mut users = self.users.lock().unwrap();
            users.insert(user.id.to_string(), user);
            Ok(())
        }

        fn find_user_by_email(&self, email: &str) -> Result<Option<User>, String> {
            let users = self.users.lock().unwrap();
            for user in users.values() {
                if user.email.as_str() == email {
                    return Ok(Some(user.clone()));
                }
            }
            Ok(None)
        }

        fn create_topic(&self, topic: Topic) -> Result<(), String> {
            let mut topics = self.topics.lock().unwrap();
            topics.insert(topic.id.to_string(), topic);
            Ok(())
        }

        fn find_topic_by_id(&self, id: &str) -> Result<Option<Topic>, String> {
            let topics = self.topics.lock().unwrap();
            Ok(topics.get(id).cloned())
        }

        fn list_topics(&self) -> Result<Vec<Topic>, String> {
            let topics = self.topics.lock().unwrap();
            Ok(topics.values().cloned().collect())
        }
    }

    #[test]
    fn test_mock_database_user_operations() {
        let db = MockDatabase::new();

        // Test creating a user
        let user = User::new(
            Email::new("test@example.com".to_string()).unwrap(),
            Password::new("MySecure123!").unwrap(),
            "Test User".to_string(),
        );

        let result = db.create_user(user.clone());
        assert!(result.is_ok());

        // Test finding user by email
        let found_user = db.find_user_by_email("test@example.com").unwrap();
        assert!(found_user.is_some());
        assert_eq!(found_user.unwrap().email.as_str(), "test@example.com");
    }

    #[test]
    fn test_mock_database_topic_operations() {
        let db = MockDatabase::new();

        // Test creating a topic
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

        let result = db.create_topic(topic.clone());
        assert!(result.is_ok());

        // Test finding topic by ID
        let found_topic = db.find_topic_by_id(&topic.id.to_string()).unwrap();
        assert!(found_topic.is_some());
        assert_eq!(found_topic.unwrap().title.en, "Rust Basics");

        // Test listing topics
        let topics = db.list_topics().unwrap();
        assert_eq!(topics.len(), 1);
        assert_eq!(topics[0].title.en, "Rust Basics");
    }

    #[test]
    fn test_value_objects() {
        // Test Email validation
        assert!(Email::new("test@example.com".to_string()).is_ok());
        assert!(Email::new("invalid-email".to_string()).is_err());

        // Test Password validation
        assert!(Password::new("MySecure123!").is_ok());
        assert!(Password::new("123").is_err());

        // Test Difficulty parsing
        assert_eq!(
            Difficulty::from_str("beginner").unwrap(),
            Difficulty::Beginner
        );
        assert_eq!(
            Difficulty::from_str("intermediate").unwrap(),
            Difficulty::Intermediate
        );
        assert_eq!(
            Difficulty::from_str("advanced").unwrap(),
            Difficulty::Advanced
        );
        assert!(Difficulty::from_str("invalid").is_err());

        // Test Points creation
        let points = Points::new(100);
        assert_eq!(points.value(), 100);

        // Test LocalizedText creation
        let text = LocalizedText::new("Hello".to_string(), "Halo".to_string());
        assert_eq!(text.en, "Hello");
        assert_eq!(text.id, "Halo");
    }

    #[test]
    fn test_entity_creation() {
        // Test User creation
        let user = User::new(
            Email::new("test@example.com".to_string()).unwrap(),
            Password::new("MySecure123!").unwrap(),
            "Test User".to_string(),
        );
        assert_eq!(user.email.as_str(), "test@example.com");
        assert_eq!(user.display_name, "Test User");

        // Test Topic creation
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
        assert_eq!(topic.title.en, "Rust Basics");
        assert_eq!(topic.order, 1);

        // Test Lesson creation
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
        assert_eq!(lesson.title.en, "Variables");
        assert_eq!(lesson.topic_id, topic.id);

        // Test Question creation
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
        assert_eq!(question.prompt.en, "What is a variable?");
        assert_eq!(question.difficulty, Difficulty::Beginner);
        assert_eq!(question.points.value(), 10);

        // Test CodePractice creation
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
        assert_eq!(code_practice.title.en, "Create a variable");
        assert_eq!(code_practice.difficulty, Difficulty::Beginner);
        assert_eq!(code_practice.points.value(), 15);

        // Test UserProgress creation
        let progress = UserProgress::new(user.id.clone());
        assert_eq!(progress.user_id, user.id);
        assert_eq!(progress.total_xp.value(), 0);
        assert_eq!(progress.current_streak_days, 0);
    }

    #[test]
    fn test_crud_operations() {
        let db = MockDatabase::new();

        // Create a user
        let user = User::new(
            Email::new("test@example.com".to_string()).unwrap(),
            Password::new("MySecure123!").unwrap(),
            "Test User".to_string(),
        );
        db.create_user(user.clone()).unwrap();

        // Create a topic
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
        db.create_topic(topic.clone()).unwrap();

        // Verify user exists
        let found_user = db.find_user_by_email("test@example.com").unwrap();
        assert!(found_user.is_some());
        assert_eq!(found_user.unwrap().display_name, "Test User");

        // Verify topic exists
        let found_topic = db.find_topic_by_id(&topic.id.to_string()).unwrap();
        assert!(found_topic.is_some());
        assert_eq!(found_topic.unwrap().title.en, "Rust Basics");

        // Verify topic list
        let topics = db.list_topics().unwrap();
        assert_eq!(topics.len(), 1);
        assert_eq!(topics[0].title.en, "Rust Basics");
    }
}
