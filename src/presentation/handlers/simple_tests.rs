#[cfg(test)]
mod tests {
    use crate::application::{
        AuthResponse, LeaderboardEntryResponse, LeaderboardResponse, LoginRequest, RegisterRequest,
        UserResponse,
    };
    use crate::domain::entities::*;
    use crate::domain::value_objects::*;
    use crate::presentation::{CodePracticeQuery, LeaderboardQuery, QuestionQuery, TopicQuery};

    #[test]
    fn test_register_request_deserialization() {
        let json = r#"{"email": "test@example.com", "password": "password123", "display_name": "Test User"}"#;
        let request: RegisterRequest = serde_json::from_str(json).unwrap();

        assert_eq!(request.email, "test@example.com");
        assert_eq!(request.password, "password123");
        assert_eq!(request.display_name, "Test User");
    }

    #[test]
    fn test_login_request_deserialization() {
        let json = r#"{"email": "test@example.com", "password": "password123"}"#;
        let request: LoginRequest = serde_json::from_str(json).unwrap();

        assert_eq!(request.email, "test@example.com");
        assert_eq!(request.password, "password123");
    }

    #[test]
    fn test_auth_response_serialization() {
        let user = User::new(
            Email::new("test@example.com".to_string()).unwrap(),
            Password::new("MySecure123!").unwrap(),
            "Test User".to_string(),
        );

        let response = AuthResponse {
            access_token: "test_token".to_string(),
            token_type: "Bearer".to_string(),
            expires_in: 3600,
            user: UserResponse::from(user),
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("test_token"));
        assert!(json.contains("Bearer"));
        assert!(json.contains("test@example.com"));
    }

    #[test]
    fn test_topic_query_deserialization() {
        let json = r#"{"topic_id": "123e4567-e89b-12d3-a456-426614174000"}"#;
        let query: TopicQuery = serde_json::from_str(json).unwrap();

        assert_eq!(
            query.topic_id,
            Some("123e4567-e89b-12d3-a456-426614174000".to_string())
        );
    }

    #[test]
    fn test_question_query_deserialization() {
        let json =
            r#"{"topic_id": "123e4567-e89b-12d3-a456-426614174000", "difficulty": "beginner"}"#;
        let query: QuestionQuery = serde_json::from_str(json).unwrap();

        assert_eq!(
            query.topic_id,
            Some("123e4567-e89b-12d3-a456-426614174000".to_string())
        );
        assert_eq!(query.difficulty, Some("beginner".to_string()));
    }

    #[test]
    fn test_code_practice_query_deserialization() {
        let json = r#"{"topic_id": "123e4567-e89b-12d3-a456-426614174000", "lesson_id": "987fcdeb-51a2-43d1-b456-426614174000"}"#;
        let query: CodePracticeQuery = serde_json::from_str(json).unwrap();

        assert_eq!(
            query.topic_id,
            Some("123e4567-e89b-12d3-a456-426614174000".to_string())
        );
        assert_eq!(
            query.lesson_id,
            Some("987fcdeb-51a2-43d1-b456-426614174000".to_string())
        );
    }

    #[test]
    fn test_leaderboard_query_deserialization() {
        let json = r#"{"limit": 10}"#;
        let query: LeaderboardQuery = serde_json::from_str(json).unwrap();

        assert_eq!(query.limit, Some(10));
    }

    #[test]
    fn test_leaderboard_response_serialization() {
        let entry = LeaderboardEntryResponse {
            user_id: "123e4567-e89b-12d3-a456-426614174000".to_string(),
            display_name: "Test User".to_string(),
            xp_this_week: 100,
            total_xp: 500,
            rank: 1,
        };

        let response = LeaderboardResponse {
            entries: vec![entry],
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("Test User"));
        assert!(json.contains("100"));
        assert!(json.contains("500"));
    }
}
