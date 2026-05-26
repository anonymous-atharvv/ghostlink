use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

/// Application error type.
/// Maps domain errors to HTTP status codes.
/// NEVER exposes internal error details to clients.
#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Username already taken")]
    UsernameConflict,

    #[error("User not found")]
    UserNotFound,

    #[error("Group not found")]
    GroupNotFound,

    #[error("Forbidden")]
    Forbidden,

    #[error("Rate limited")]
    RateLimited,

    #[error("Token expired")]
    TokenExpired,

    #[error("Token invalid")]
    TokenInvalid,

    #[error("Contact already exists")]
    ContactExists,

    #[error("Validation failed: {0}")]
    Validation(String),

    #[error("Internal error")]
    InternalError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = match &self {
            AppError::InvalidCredentials => (
                StatusCode::UNAUTHORIZED,
                "INVALID_CREDENTIALS",
                "Invalid username or password",
            ),
            AppError::UsernameConflict => (
                StatusCode::CONFLICT,
                "USERNAME_CONFLICT",
                "Username is already taken",
            ),
            AppError::UserNotFound => (
                StatusCode::NOT_FOUND,
                "USER_NOT_FOUND",
                "User not found",
            ),
            AppError::GroupNotFound => (
                StatusCode::NOT_FOUND,
                "GROUP_NOT_FOUND",
                "Group not found",
            ),
            AppError::Forbidden => (
                StatusCode::FORBIDDEN,
                "FORBIDDEN",
                "Insufficient permissions",
            ),
            AppError::RateLimited => (
                StatusCode::TOO_MANY_REQUESTS,
                "RATE_LIMITED",
                "Too many requests",
            ),
            AppError::TokenExpired => (
                StatusCode::UNAUTHORIZED,
                "TOKEN_EXPIRED",
                "Token has expired",
            ),
            AppError::TokenInvalid => (
                StatusCode::UNAUTHORIZED,
                "TOKEN_INVALID",
                "Invalid token",
            ),
            AppError::ContactExists => (
                StatusCode::CONFLICT,
                "CONTACT_EXISTS",
                "Contact already exists or request pending",
            ),
            AppError::Validation(msg) => (
                StatusCode::BAD_REQUEST,
                "VALIDATION_ERROR",
                msg.as_str(),
            ),
            AppError::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_ERROR",
                "An internal error occurred",
            ),
        };

        let body = json!({
            "error": code,
            "message": message,
        });

        (status, Json(body)).into_response()
    }
}

/// Convert anyhow errors to AppError::InternalError.
/// Logs the actual error internally but never exposes it to clients.
impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        // Log internally — NO PII, only error message
        tracing::error!(error = %err, "Internal error occurred");
        AppError::InternalError
    }
}
