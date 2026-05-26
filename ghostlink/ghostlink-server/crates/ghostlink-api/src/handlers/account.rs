use axum::{extract::State, http::StatusCode, Extension, Json};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use serde::{Deserialize, Serialize};
use validator::Validate;
use zeroize::Zeroize;

use crate::error::AppError;
use crate::AppState;
use ghostlink_core::account::AuthenticatedAccount;

// ─── Request DTOs ───

#[derive(Debug, Deserialize, Validate, Zeroize)]
#[zeroize(drop)]
pub struct DeleteAccountRequest {
    #[validate(length(min = 1))]
    pub password: String,
}

// ─── Response DTOs ───

#[derive(Debug, Serialize)]
pub struct AccountMeResponse {
    pub account_id: uuid::Uuid,
    pub username: String,
    pub created_at: String,
    pub last_seen_at: Option<String>,
}

// ─── Handlers ───

/// GET /account/me
/// Retrieve profile details for the authenticated user.
pub async fn me(
    State(state): State<AppState>,
    Extension(auth_account): Extension<AuthenticatedAccount>,
) -> Result<Json<AccountMeResponse>, AppError> {
    let account = state
        .account_repo
        .find_by_id(auth_account.id)
        .await?
        .ok_or(AppError::UserNotFound)?;

    Ok(Json(AccountMeResponse {
        account_id: account.id,
        username: account.username,
        created_at: account.created_at.to_rfc3339(),
        last_seen_at: account.last_seen_at.map(|dt| dt.to_rfc3339()),
    }))
}

/// DELETE /account/me
/// Permanently delete account and username index. Requires password confirmation.
pub async fn delete(
    State(state): State<AppState>,
    Extension(auth_account): Extension<AuthenticatedAccount>,
    Json(mut req): Json<DeleteAccountRequest>,
) -> Result<StatusCode, AppError> {
    req.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    // Find account to verify password
    let account = state
        .account_repo
        .find_by_id(auth_account.id)
        .await?
        .ok_or(AppError::UserNotFound)?;

    let parsed_hash =
        PasswordHash::new(&account.password_hash).map_err(|_| AppError::InternalError)?;

    Argon2::default()
        .verify_password(req.password.as_bytes(), &parsed_hash)
        .map_err(|_| AppError::InvalidCredentials)?;

    // Zeroize password explicit trigger just in case
    req.password.zeroize();

    // Delete record from database
    state.account_repo.delete(account.id, &account.username).await?;

    // Invalidate session cache
    let _ = state.session_cache.invalidate_all(account.id).await;

    Ok(StatusCode::NO_CONTENT)
}
