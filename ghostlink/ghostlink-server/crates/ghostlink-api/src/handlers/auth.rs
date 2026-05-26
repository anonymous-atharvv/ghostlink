use axum::{extract::State, http::StatusCode, Json};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::error::AppError;
use crate::AppState;
use ghostlink_core::crypto::{OneTimePreKey, SignedPreKey};

// ─── Request DTOs ───

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 32))]
    pub username: String,
    #[validate(length(min = 8))]
    pub password: String,
    pub identity_key: String,
    pub signed_pre_key: SignedPreKey,
    pub one_time_pre_keys: Vec<OneTimePreKey>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(length(min = 3, max = 32))]
    pub username: String,
    #[validate(length(min = 1))]
    pub password: String,
}

// ─── Response DTOs ───

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub account_id: Uuid,
    pub username: String,
}

// ─── JWT Claims ───

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    iat: usize,
}

// ─── Handlers ───

/// POST /auth/register
/// Create a new anonymous account with username + password + Signal key bundle.
pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), AppError> {
    req.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    // Normalize username to lowercase
    let normalized = req.username.to_lowercase();

    // Validate username charset (alphanumeric + underscore only)
    if !normalized.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err(AppError::Validation(
            "Username must contain only alphanumeric characters and underscores".to_string(),
        ));
    }

    // Check username availability
    if state.account_repo.username_exists(&normalized).await? {
        return Err(AppError::UsernameConflict);
    }

    // Hash password with Argon2id
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(req.password.as_bytes(), &salt)
        .map_err(|_| AppError::InternalError)?
        .to_string();

    // Generate account
    let account_id = Uuid::new_v4();
    let account = ghostlink_core::account::Account {
        id: account_id,
        username: normalized.clone(),
        password_hash,
        created_at: chrono::Utc::now(),
        last_seen_at: None,
    };

    // Persist account + Signal key bundle
    state.account_repo.create(&account).await?;
    state
        .key_repo
        .store_key_bundle(
            account_id,
            &req.identity_key,
            &req.signed_pre_key,
            &req.one_time_pre_keys,
        )
        .await?;

    // Issue JWT
    let token = issue_jwt(account_id, &state.config.jwt_secret)?;

    Ok((
        StatusCode::CREATED,
        Json(AuthResponse {
            token,
            account_id,
            username: normalized,
        }),
    ))
}

/// POST /auth/login
/// Authenticate with username + password, return JWT.
pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    req.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let normalized = req.username.to_lowercase();

    // Find account — same error for wrong username OR password (security)
    let account = state
        .account_repo
        .find_by_username(&normalized)
        .await?
        .ok_or(AppError::InvalidCredentials)?;

    // Verify password
    let parsed_hash =
        PasswordHash::new(&account.password_hash).map_err(|_| AppError::InternalError)?;

    Argon2::default()
        .verify_password(req.password.as_bytes(), &parsed_hash)
        .map_err(|_| AppError::InvalidCredentials)?;

    // Update last seen (timestamp only — NO IP address)
    let _ = state.account_repo.update_last_seen(account.id).await;

    // Issue JWT
    let token = issue_jwt(account.id, &state.config.jwt_secret)?;

    Ok(Json(AuthResponse {
        token,
        account_id: account.id,
        username: account.username,
    }))
}

/// Issue a JWT token with account_id as subject.
/// Claims contain ONLY: sub (account_id), exp, iat — NO username, NO email.
fn issue_jwt(account_id: Uuid, secret: &str) -> Result<String, AppError> {
    let now = chrono::Utc::now().timestamp() as usize;
    let claims = Claims {
        sub: account_id.to_string(),
        exp: now + 86400 * 30, // 30-day expiry
        iat: now,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|_| AppError::InternalError)
}
