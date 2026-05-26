use std::sync::Arc;

use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use tower_http::limit::RequestBodyLimitLayer;

use crate::handlers;
use crate::middleware::{auth::auth_middleware, rate_limit::RateLimitConfig, request_id::request_id_middleware};
use crate::AppState;

/// Build the complete Axum router with all routes and middleware.
pub fn create_router(state: AppState) -> Router {
    let config = Arc::clone(&state.config);

    // Public routes (no auth required) with auth rate limiting
    let public_routes = Router::new()
        .route("/auth/register", post(handlers::auth::register))
        .route("/auth/login", post(handlers::auth::login))
        .layer(RateLimitConfig::auth_layer(&config));

    // Protected routes (auth middleware applied)
    let protected_routes = Router::new()
        // Account
        .route("/account/me", get(handlers::account::me).delete(handlers::account::delete))
        // Contacts
        .route("/contacts", get(handlers::contacts::list).post(handlers::contacts::add))
        .route("/contacts/:contact_id", axum::routing::patch(handlers::contacts::respond).delete(handlers::contacts::delete))
        // Keys
        .route("/keys/:username/bundle", get(handlers::keys::get_bundle))
        .route("/keys/pre-keys", axum::routing::put(handlers::keys::upload_pre_keys))
        .route("/keys/pre-keys/count", get(handlers::keys::count_pre_keys))
        // Messages
        .route("/messages/offline", get(handlers::messages::fetch_offline).delete(handlers::messages::ack_offline))
        // WebSocket
        .route("/ws/connect", get(handlers::websocket::ws_upgrade))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .layer(RateLimitConfig::api_layer(&config));

    // Health endpoints (always public)
    let health_routes = Router::new()
        .route("/health", get(handlers::health::health))
        .route("/health/ready", get(handlers::health::ready));

    // Assemble the full router
    Router::new()
        .nest("/v1", public_routes.merge(protected_routes))
        .merge(health_routes)
        .layer(middleware::from_fn(request_id_middleware))
        .layer(RequestBodyLimitLayer::new(
            config.max_media_size_bytes.max(config.max_message_size_bytes),
        ))
        .with_state(state)
}
