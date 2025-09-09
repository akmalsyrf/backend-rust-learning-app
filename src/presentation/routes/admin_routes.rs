use axum::routing::get;
use axum::Router;

use crate::application::state::AppState;
use crate::presentation::web::dashboard::dashboard_handler;

use super::advanced_routes::advanced_routes;
use super::crud_routes::crud_routes;

pub fn admin_routes() -> Router<AppState> {
    Router::new()
        // Dashboard
        .route("/admin", get(dashboard_handler))
        // Merge CRUD routes
        .merge(crud_routes())
        // Merge advanced features routes
        .merge(advanced_routes())
}
