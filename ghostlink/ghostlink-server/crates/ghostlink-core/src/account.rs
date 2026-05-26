use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use zeroize::Zeroize;

/// Core account entity.
/// Represents a pseudonymous user — no email, phone, or real name.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: Uuid,
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub last_seen_at: Option<DateTime<Utc>>,
}

/// Authenticated account extracted from JWT middleware.
/// Carried through request extensions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticatedAccount {
    pub id: Uuid,
    pub username: String,
}

/// Temporary struct for password handling — zeroized on drop.
#[derive(Debug, Zeroize)]
#[zeroize(drop)]
pub struct SensitivePassword {
    pub value: String,
}

impl SensitivePassword {
    pub fn new(password: String) -> Self {
        Self { value: password }
    }
}

/// Account creation parameters
#[derive(Debug, Clone)]
pub struct NewAccount {
    pub username: String,
    pub password_hash: String,
}
