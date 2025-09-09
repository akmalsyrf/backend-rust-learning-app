pub mod admin_routes;
pub mod advanced_routes;
pub mod api_routes;
pub mod crud_routes;
pub mod health_routes;

use axum::Router;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

use crate::application::state::AppState;

use self::admin_routes::admin_routes;
use self::api_routes::api_routes;
use self::health_routes::health_routes;

pub fn create_app_router(app_state: AppState) -> Router {
    Router::new()
        // Health check routes
        .merge(health_routes())
        // API routes
        .merge(api_routes())
        // Admin routes
        .merge(admin_routes())
        // Middleware layers
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(
                    CorsLayer::new()
                        .allow_origin(Any)
                        .allow_methods(Any)
                        .allow_headers(Any),
                ),
        )
        .with_state(app_state)
}
