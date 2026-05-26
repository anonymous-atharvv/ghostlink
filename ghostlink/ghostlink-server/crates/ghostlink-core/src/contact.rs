use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::types::ContactStatus;

/// Contact relationship between two accounts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub owner_id: Uuid,
    pub contact_id: Uuid,
    pub username: String,
    pub status: ContactStatus,
    pub added_at: DateTime<Utc>,
}

/// Action that can be performed on a contact request
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContactAction {
    Accept,
    Decline,
    Block,
}
