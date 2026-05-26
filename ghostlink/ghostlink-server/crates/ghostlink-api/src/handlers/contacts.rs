use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::error::AppError;
use crate::AppState;
use ghostlink_core::account::AuthenticatedAccount;
use ghostlink_core::contact::ContactAction;

// ─── Request DTOs ───

#[derive(Debug, Deserialize, Validate)]
pub struct AddContactRequest {
    #[validate(length(min = 3, max = 32))]
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct RespondContactRequest {
    pub action: ContactAction,
}

// ─── Response DTOs ───

#[derive(Debug, Serialize)]
pub struct ContactResponse {
    pub contact_id: Uuid,
    pub username: String,
    pub status: ghostlink_core::types::ContactStatus,
    pub added_at: String,
}

// ─── Handlers ───

/// GET /contacts
/// Retrieve the list of all contact relationships.
pub async fn list(
    State(state): State<AppState>,
    Extension(auth_account): Extension<AuthenticatedAccount>,
) -> Result<Json<Vec<ContactResponse>>, AppError> {
    let contacts = state.contact_repo.list(auth_account.id).await?;
    
    let response = contacts
        .into_iter()
        .map(|c| ContactResponse {
            contact_id: c.contact_id,
            username: c.username,
            status: c.status,
            added_at: c.added_at.to_rfc3339(),
        })
        .collect();

    Ok(Json(response))
}

/// POST /contacts
/// Send a contact request to an exact username.
pub async fn add(
    State(state): State<AppState>,
    Extension(auth_account): Extension<AuthenticatedAccount>,
    Json(req): Json<AddContactRequest>,
) -> Result<StatusCode, AppError> {
    req.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let normalized_target = req.username.to_lowercase();

    // Cannot add yourself
    if normalized_target == auth_account.username.to_lowercase() {
        return Err(AppError::Validation("Cannot add yourself as a contact".to_string()));
    }

    // Find the target account
    let target = state
        .account_repo
        .find_by_username(&normalized_target)
        .await?
        .ok_or(AppError::UserNotFound)?;

    // Check if a relationship already exists
    let existing = state.contact_repo.list(auth_account.id).await?;
    if existing.iter().any(|c| c.contact_id == target.id) {
        return Err(AppError::ContactExists);
    }

    // Create the dual request
    state
        .contact_repo
        .create_request(
            auth_account.id,
            target.id,
            &target.username,
            &auth_account.username,
        )
        .await?;

    Ok(StatusCode::CREATED)
}

/// PATCH /contacts/{contact_id}
/// Accept, decline, or block a contact request.
pub async fn respond(
    State(state): State<AppState>,
    Extension(auth_account): Extension<AuthenticatedAccount>,
    Path(contact_id): Path<Uuid>,
    Json(req): Json<RespondContactRequest>,
) -> Result<StatusCode, AppError> {
    // Verify target exists
    let existing = state.contact_repo.list(auth_account.id).await?;
    let relationship = existing
        .iter()
        .find(|c| c.contact_id == contact_id)
        .ok_or(AppError::UserNotFound)?;

    match req.action {
        ContactAction::Accept => {
            // Must be pending_received to accept
            if relationship.status != ghostlink_core::types::ContactStatus::PendingReceived {
                return Err(AppError::Validation(
                    "Can only accept incoming pending contact requests".to_string(),
                ));
            }
            state.contact_repo.accept(auth_account.id, contact_id).await?;
        }
        ContactAction::Decline => {
            state.contact_repo.remove(auth_account.id, contact_id).await?;
            state.contact_repo.remove(contact_id, auth_account.id).await?;
        }
        ContactAction::Block => {
            state.contact_repo.block(auth_account.id, contact_id).await?;
        }
    }

    Ok(StatusCode::OK)
}

/// DELETE /contacts/{contact_id}
/// Remove a contact relationship.
pub async fn delete(
    State(state): State<AppState>,
    Extension(auth_account): Extension<AuthenticatedAccount>,
    Path(contact_id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    // Delete mutual relationships
    state.contact_repo.remove(auth_account.id, contact_id).await?;
    state.contact_repo.remove(contact_id, auth_account.id).await?;

    Ok(StatusCode::NO_CONTENT)
}
