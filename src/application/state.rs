use axum::extract::FromRef;
use std::sync::Arc;

use crate::application::use_cases::{AuthUseCases, ContentUseCases, ProgressUseCases};
use crate::domain::repositories::{
    CodePracticeRepository, LeaderboardRepository, LessonRepository, NotificationRepository,
    QuestionRepository, TopicRepository, UserProgressRepository, UserRepository,
};
use crate::domain::services::{
    auth_service::JwtAuthService, leaderboard_service::LeaderboardServiceImpl,
    progress_service::ProgressServiceImpl,
};
use crate::domain::services::{AuthService, LeaderboardService, ProgressService};
use crate::infrastructure::external_services::{EmailService, GeminiService};
use crate::infrastructure::repositories::{
    CodePracticeRepositoryImpl, LeaderboardRepositoryImpl, LessonRepositoryImpl,
    NotificationRepositoryImpl, QuestionRepositoryImpl, TopicRepositoryImpl,
    UserProgressRepositoryImpl, UserRepositoryImpl,
};
use crate::shared::config::Config;
use crate::shared::errors::AppError;

#[derive(Clone)]
pub struct AppState {
    // Repositories
    pub user_repository: Arc<dyn UserRepository>,
    pub topic_repository: Arc<dyn TopicRepository>,
    pub lesson_repository: Arc<dyn LessonRepository>,
    pub question_repository: Arc<dyn QuestionRepository>,
    pub code_practice_repository: Arc<dyn CodePracticeRepository>,
    pub user_progress_repository: Arc<dyn UserProgressRepository>,
    pub leaderboard_repository: Arc<dyn LeaderboardRepository>,
    pub notification_repository: Arc<dyn NotificationRepository>,

    // Services
    pub auth_service: Arc<dyn AuthService>,
    pub progress_service: Arc<dyn ProgressService>,
    pub leaderboard_service: Arc<dyn LeaderboardService>,

    // External Services
    pub gemini_service: GeminiService,
    pub email_service: EmailService,

    // Use Cases
    pub auth_use_cases: AuthUseCases,
    pub content_use_cases: ContentUseCases,
    pub progress_use_cases: ProgressUseCases,
}

impl AppState {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        topic_repository: Arc<dyn TopicRepository>,
        lesson_repository: Arc<dyn LessonRepository>,
        question_repository: Arc<dyn QuestionRepository>,
        code_practice_repository: Arc<dyn CodePracticeRepository>,
        user_progress_repository: Arc<dyn UserProgressRepository>,
        leaderboard_repository: Arc<dyn LeaderboardRepository>,
        notification_repository: Arc<dyn NotificationRepository>,
        auth_service: Arc<dyn AuthService>,
        progress_service: Arc<dyn ProgressService>,
        leaderboard_service: Arc<dyn LeaderboardService>,
        gemini_service: GeminiService,
        email_service: EmailService,
    ) -> Self {
        let auth_use_cases = AuthUseCases::new(auth_service.clone());
        let content_use_cases = ContentUseCases::new(
            topic_repository.clone(),
            lesson_repository.clone(),
            question_repository.clone(),
            code_practice_repository.clone(),
        );
        let progress_use_cases = ProgressUseCases::new(leaderboard_repository.clone());

        Self {
            user_repository,
            topic_repository,
            lesson_repository,
            question_repository,
            code_practice_repository,
            user_progress_repository,
            leaderboard_repository,
            notification_repository,
            auth_service,
            progress_service,
            leaderboard_service,
            gemini_service,
            email_service,
            auth_use_cases,
            content_use_cases,
            progress_use_cases,
        }
    }

    /// Create AppState from configuration
    pub async fn from_config(config: &Config) -> Result<Self, AppError> {
        // Create database connection
        let pool = sqlx::PgPool::connect(&config.database_url)
            .await
            .map_err(|e| AppError::Internal(format!("Failed to connect to database: {e}")))?;

        // Create repositories
        let user_repository = Arc::new(UserRepositoryImpl::new(pool.clone()));
        let topic_repository = Arc::new(TopicRepositoryImpl::new(pool.clone()));
        let lesson_repository = Arc::new(LessonRepositoryImpl::new(pool.clone()));
        let question_repository = Arc::new(QuestionRepositoryImpl::new(pool.clone()));
        let code_practice_repository = Arc::new(CodePracticeRepositoryImpl::new(pool.clone()));
        let user_progress_repository = Arc::new(UserProgressRepositoryImpl::new(pool.clone()));
        let leaderboard_repository = Arc::new(LeaderboardRepositoryImpl::new(pool.clone()));
        let notification_repository = Arc::new(NotificationRepositoryImpl::new());

        // Create services
        let auth_service = Arc::new(JwtAuthService::new(
            config.jwt_secret.clone(),
            user_repository.clone(),
        ));
        let progress_service = Arc::new(ProgressServiceImpl::new(user_progress_repository.clone()));
        let leaderboard_service =
            Arc::new(LeaderboardServiceImpl::new(leaderboard_repository.clone()));

        // Create external services
        let gemini_service =
            GeminiService::new(config.gemini_api_key.clone(), config.gemini_api_url.clone());
        let email_service = EmailService::new(
            config.smtp_host.clone(),
            config.smtp_port,
            config.smtp_username.clone(),
            config.smtp_password.clone(),
            config.from_email.clone(),
            config.from_name.clone(),
        )
        .map_err(|e| AppError::Internal(format!("Failed to create email service: {e}")))?;

        // Create AppState
        Ok(Self::new(
            user_repository,
            topic_repository,
            lesson_repository,
            question_repository,
            code_practice_repository,
            user_progress_repository,
            leaderboard_repository,
            notification_repository,
            auth_service,
            progress_service,
            leaderboard_service,
            gemini_service,
            email_service,
        ))
    }
}

// Implement FromRef for all components to enable Axum 0.7 compatibility
impl FromRef<AppState> for Arc<dyn UserRepository> {
    fn from_ref(app_state: &AppState) -> Self {
        Arc::clone(&app_state.user_repository)
    }
}

impl FromRef<AppState> for Arc<dyn TopicRepository> {
    fn from_ref(app_state: &AppState) -> Self {
        Arc::clone(&app_state.topic_repository)
    }
}

impl FromRef<AppState> for Arc<dyn LessonRepository> {
    fn from_ref(app_state: &AppState) -> Self {
        Arc::clone(&app_state.lesson_repository)
    }
}

impl FromRef<AppState> for Arc<dyn QuestionRepository> {
    fn from_ref(app_state: &AppState) -> Self {
        Arc::clone(&app_state.question_repository)
    }
}

impl FromRef<AppState> for Arc<dyn CodePracticeRepository> {
    fn from_ref(app_state: &AppState) -> Self {
        Arc::clone(&app_state.code_practice_repository)
    }
}

impl FromRef<AppState> for Arc<dyn UserProgressRepository> {
    fn from_ref(app_state: &AppState) -> Self {
        Arc::clone(&app_state.user_progress_repository)
    }
}

impl FromRef<AppState> for Arc<dyn LeaderboardRepository> {
    fn from_ref(app_state: &AppState) -> Self {
        Arc::clone(&app_state.leaderboard_repository)
    }
}

impl FromRef<AppState> for Arc<dyn NotificationRepository> {
    fn from_ref(app_state: &AppState) -> Self {
        Arc::clone(&app_state.notification_repository)
    }
}

impl FromRef<AppState> for Arc<dyn AuthService> {
    fn from_ref(app_state: &AppState) -> Self {
        Arc::clone(&app_state.auth_service)
    }
}

impl FromRef<AppState> for Arc<dyn ProgressService> {
    fn from_ref(app_state: &AppState) -> Self {
        Arc::clone(&app_state.progress_service)
    }
}

impl FromRef<AppState> for Arc<dyn LeaderboardService> {
    fn from_ref(app_state: &AppState) -> Self {
        Arc::clone(&app_state.leaderboard_service)
    }
}

impl FromRef<AppState> for AuthUseCases {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.auth_use_cases.clone()
    }
}

impl FromRef<AppState> for ContentUseCases {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.content_use_cases.clone()
    }
}

impl FromRef<AppState> for ProgressUseCases {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.progress_use_cases.clone()
    }
}

impl FromRef<AppState> for GeminiService {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.gemini_service.clone()
    }
}

impl FromRef<AppState> for EmailService {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.email_service.clone()
    }
}
