use axum::routing::{get, post};
use axum::Router;

use crate::application::state::AppState;
use crate::presentation::web::ai_features::{
    ai_features_handler, explain_code_form_handler, explain_code_post_handler,
    generate_quiz_form_handler, generate_quiz_post_handler, improve_code_form_handler,
    improve_code_post_handler, validate_code_form_handler, validate_code_post_handler,
};
use crate::presentation::web::analytics::analytics_dashboard_handler;
use crate::presentation::web::audit_logging::audit_logging_handler;
use crate::presentation::web::bulk_operations::{
    bulk_operations_form_handler, bulk_operations_handler,
};
use crate::presentation::web::content_management::{
    content_management_handler, content_versions_handler,
};
use crate::presentation::web::data_validation::data_validation_handler;
use crate::presentation::web::export_import::{
    backup_handler, export_data_handler, export_import_handler, history_handler,
    import_data_handler, restore_handler, stats_handler,
};
use crate::presentation::web::file_upload::{file_upload_handler, upload_file_handler};
use crate::presentation::web::notifications::{
    create_notification_handler, delete_notification_handler, mark_notification_read_handler,
    notifications_handler,
};
use crate::presentation::web::search_filter::{
    search_code_practices_handler, search_lessons_handler, search_questions_handler,
    search_topics_handler, search_users_handler,
};
use crate::presentation::web::user_management::{
    user_management_handler, user_permissions_handler,
};
use crate::presentation::web::user_profile::{edit_user_profile_handler, user_profile_handler};

pub fn advanced_routes() -> Router<AppState> {
    Router::new()
        // AI Features routes
        .route("/admin/ai", get(ai_features_handler))
        .route("/admin/ai/generate-quiz", get(generate_quiz_form_handler).post(generate_quiz_post_handler))
        .route("/admin/ai/validate-code", get(validate_code_form_handler).post(validate_code_post_handler))
        .route("/admin/ai/explain-code", get(explain_code_form_handler).post(explain_code_post_handler))
        .route("/admin/ai/improve-code", get(improve_code_form_handler).post(improve_code_post_handler))

        // Search and Filter routes
        .route("/admin/search/topics", get(search_topics_handler))
        .route("/admin/search/lessons", get(search_lessons_handler))
        .route("/admin/search/questions", get(search_questions_handler))
        .route("/admin/search/code-practices", get(search_code_practices_handler))
        .route("/admin/search/users", get(search_users_handler))

        // Analytics routes
        .route("/admin/analytics", get(analytics_dashboard_handler))

        // Notifications routes
        .route("/admin/notifications", get(notifications_handler))
        .route("/admin/notifications/create", get(create_notification_handler))
        .route("/admin/notifications/:id/mark-read", get(mark_notification_read_handler))
        .route("/admin/notifications/:id/delete", get(delete_notification_handler))

        // Export/Import routes
        .route("/admin/export-import", get(export_import_handler))
        .route("/admin/export-import/export", get(export_data_handler))
        .route("/admin/export-import/import", post(import_data_handler))
        .route("/admin/export-import/backup", get(backup_handler))
        .route("/admin/export-import/restore", get(restore_handler))
        .route("/admin/export-import/stats", get(stats_handler))
        .route("/admin/export-import/history", get(history_handler))

        // File Upload routes
        .route("/admin/file-upload", get(file_upload_handler))
        .route("/admin/file-upload/upload", post(upload_file_handler))

        // User Profile routes
        .route("/admin/users/:id/profile", get(user_profile_handler))
        .route("/admin/users/:id/edit-profile", get(edit_user_profile_handler))

        // Advanced CRUD Operations routes
        .route("/admin/bulk-operations", get(bulk_operations_form_handler))
        .route("/admin/bulk-operations/execute", get(bulk_operations_handler))
        .route("/admin/user-management", get(user_management_handler))
        .route("/admin/users/:id/permissions", get(user_permissions_handler))
        .route("/admin/content-management", get(content_management_handler))
        .route("/admin/content/:id/versions", get(content_versions_handler))
        .route("/admin/data-validation", get(data_validation_handler))
        .route("/admin/audit-logs", get(audit_logging_handler))
}
