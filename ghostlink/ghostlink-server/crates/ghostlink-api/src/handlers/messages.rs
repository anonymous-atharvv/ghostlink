use axum::{extract::State, http::StatusCode, Extension, Json};
use serde::Serialize;
use uuid::Uuid;
use base64::prelude::*;

use crate::error::AppError;
use crate::AppState;
use ghostlink_core::account::AuthenticatedAccount;

// ─── Response DTOs ───

#[derive(Debug, Serialize)]
pub struct OfflineMessageResponse {
    pub message_id: Uuid,
    pub conversation_id: Uuid,
    pub sender_id: Uuid,
    pub encrypted_payload: String, // Base64
    pub payload_type: ghostlink_core::types::PayloadType,
    pub created_at: String,
}

// ─── Handlers ───

/// GET /messages/offline
/// Retrieve all pending offline messages for the authenticated user.
pub async fn fetch_offline(
    State(state): State<AppState>,
    Extension(auth_account): Extension<AuthenticatedAccount>,
) -> Result<Json<Vec<OfflineMessageResponse>>, AppError> {
    let messages = state
        .message_repo
        .fetch_offline(auth_account.id)
        .await?;

    let response = messages
        .into_iter()
        .map(|m| OfflineMessageResponse {
            message_id: m.message_id,
            conversation_id: m.conversation_id,
            sender_id: m.sender_id,
            encrypted_payload: BASE64_STANDARD.encode(&m.encrypted_payload),
            payload_type: m.payload_type,
            created_at: m.created_at.to_rfc3339(),
        })
        .collect();

    Ok(Json(response))
}

/// DELETE /messages/offline
/// Acknowledge and clear the offline message queue for the authenticated user.
pub async fn ack_offline(
    State(state): State<AppState>,
    Extension(auth_account): Extension<AuthenticatedAccount>,
) -> Result<StatusCode, AppError> {
    state
        .message_repo
        .clear_offline(auth_account.id)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
