use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::error::AppError;
use crate::AppState;
use ghostlink_core::account::AuthenticatedAccount;
use ghostlink_core::crypto::{OneTimePreKey, KeyBundle};

// ─── Request DTOs ───

#[derive(Debug, Deserialize, Validate)]
pub struct UploadPreKeysRequest {
    #[validate(length(min = 1, max = 100))]
    pub one_time_pre_keys: Vec<OneTimePreKey>,
}

// ─── Response DTOs ───

#[derive(Debug, Serialize)]
pub struct KeyCountResponse {
    pub count: i64,
}

// ─── Handlers ───

/// GET /keys/{username}/bundle
/// Retrieve the Signal Protocol X3DH key bundle for a user.
/// Consumes one One-Time Pre-Key (OPK) atomically from ScyllaDB.
pub async fn get_bundle(
    State(state): State<AppState>,
    Extension(_auth_account): Extension<AuthenticatedAccount>,
    Path(username): Path<String>,
) -> Result<Json<KeyBundle>, AppError> {
    let normalized = username.to_lowercase();

    // Look up user account
    let target = state
        .account_repo
        .find_by_username(&normalized)
        .await?
        .ok_or(AppError::UserNotFound)?;

    // Fetch and atomically consume the bundle
    let bundle = state
        .key_repo
        .get_key_bundle(target.id)
        .await?
        .ok_or(AppError::InternalError)?;

    // Update presence OTP count in Redis to avoid stale lookups
    let remaining_count = state.key_repo.count_pre_keys(target.id).await.unwrap_or(0);
    let _ = state.presence_cache.set_pre_key_count(target.id, remaining_count).await;

    Ok(Json(bundle))
}

/// PUT /keys/pre-keys
/// Upload new One-Time Pre-Keys (OPKs) to refill the pool.
pub async fn upload_pre_keys(
    State(state): State<AppState>,
    Extension(auth_account): Extension<AuthenticatedAccount>,
    Json(req): Json<UploadPreKeysRequest>,
) -> Result<StatusCode, AppError> {
    req.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    // Perform bulk upload
    state
        .key_repo
        .upload_pre_keys(auth_account.id, &req.one_time_pre_keys)
        .await?;

    // Cache the new pre-key count in Redis presence cache
    let count = state.key_repo.count_pre_keys(auth_account.id).await?;
    let _ = state.presence_cache.set_pre_key_count(auth_account.id, count).await;

    Ok(StatusCode::NO_CONTENT)
}

/// GET /keys/pre-keys/count
/// Check the remaining number of One-Time Pre-Keys in the pool.
pub async fn count_pre_keys(
    State(state): State<AppState>,
    Extension(auth_account): Extension<AuthenticatedAccount>,
) -> Result<Json<KeyCountResponse>, AppError> {
    // Attempt cache hit first from Redis via presence cache
    let cached_count = state.presence_cache.get_pre_key_count(auth_account.id).await.ok();

    let count = match cached_count {
        Some(Some(c)) => c,
        _ => {
            // Miss: query ScyllaDB and hydrate cache
            let count = state.key_repo.count_pre_keys(auth_account.id).await?;
            let _ = state.presence_cache.set_pre_key_count(auth_account.id, count).await;
            count
        }
    };

    Ok(Json(KeyCountResponse { count }))
}
