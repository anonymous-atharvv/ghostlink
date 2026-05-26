use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::types::GroupRole;

/// Group entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub creator_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub encrypted_avatar_key: Option<String>,
}

/// Group membership record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupMember {
    pub group_id: Uuid,
    pub member_id: Uuid,
    pub username: String,
    pub role: GroupRole,
    pub joined_at: DateTime<Utc>,
}

/// Group invite link
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupInvite {
    pub group_id: Uuid,
    pub token: String,
    pub created_by: Uuid,
    pub expires_at: DateTime<Utc>,
}

/// Maximum members per group
pub const MAX_GROUP_MEMBERS: usize = 256;
