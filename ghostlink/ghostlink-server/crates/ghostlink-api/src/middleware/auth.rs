use axum::{
    extract::{Request, State},
    http::header::AUTHORIZATION,
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::AppError;
use crate::AppState;
use ghostlink_core::account::AuthenticatedAccount;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    iat: usize,
}

/// JWT authentication middleware.
/// Extracts and validates the Bearer token, then injects AuthenticatedAccount
/// into request extensions for downstream handlers.
pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let auth_header = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .ok_or(AppError::TokenInvalid)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(AppError::TokenInvalid)?;

    // Decode and validate JWT
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(state.config.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|e| match e.kind() {
        jsonwebtoken::errors::ErrorKind::ExpiredSignature => AppError::TokenExpired,
        _ => AppError::TokenInvalid,
    })?;

    // Parse account ID from claims
    let account_id: Uuid = token_data
        .claims
        .sub
        .parse()
        .map_err(|_| AppError::TokenInvalid)?;

    // Verify account still exists
    let account = state
        .account_repo
        .find_by_id(account_id)
        .await
        .map_err(|_| AppError::InternalError)?
        .ok_or(AppError::TokenInvalid)?;

    // Inject authenticated account into request extensions
    req.extensions_mut().insert(AuthenticatedAccount {
        id: account.id,
        username: account.username,
    });

    Ok(next.run(req).await)
}
