#[cfg(test)]
mod tests {
    use crate::application::TopicResponse;
    use crate::domain::entities::*;
    use crate::domain::value_objects::*;

    #[test]
    fn test_topic_response_from_topic() {
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
        let response = TopicResponse::from(topic);

        assert_eq!(response.title.en, "Rust Basics");
        assert_eq!(response.title.id, "Dasar Rust");
        assert_eq!(response.order, 1);
    }

    #[test]
    fn test_difficulty_parsing() {
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
    }

    #[test]
    fn test_email_validation() {
        assert!(Email::new("test@example.com".to_string()).is_ok());
        assert!(Email::new("invalid-email".to_string()).is_err());
    }

    #[test]
    fn test_password_validation() {
        // Test valid password (meets all requirements)
        assert!(Password::new("MySecure123!").is_ok());
        assert!(Password::new("TestPass456@").is_ok());

        // Test invalid passwords
        assert!(Password::new("password123").is_err()); // No uppercase, no special char
        assert!(Password::new("123").is_err()); // Too short
        assert!(Password::new("Password").is_err()); // No digit, no special char
        assert!(Password::new("password123!").is_err()); // No uppercase
        assert!(Password::new("PASSWORD123!").is_err()); // No lowercase
        assert!(Password::new("Password123").is_err()); // No special char
        assert!(Password::new("Password123!").is_err()); // Contains common pattern
    }
}
