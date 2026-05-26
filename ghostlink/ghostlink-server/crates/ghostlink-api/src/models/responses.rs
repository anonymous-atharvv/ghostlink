use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub account_id: Uuid,
    pub username: String,
}

#[derive(Debug, Serialize)]
pub struct AccountMeResponse {
    pub account_id: Uuid,
    pub username: String,
    pub created_at: String,
    pub last_seen_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ContactResponse {
    pub contact_id: Uuid,
    pub username: String,
    pub status: ghostlink_core::types::ContactStatus,
    pub added_at: String,
}

#[derive(Debug, Serialize)]
pub struct KeyCountResponse {
    pub count: i64,
}

#[derive(Debug, Serialize)]
pub struct OfflineMessageResponse {
    pub message_id: Uuid,
    pub conversation_id: Uuid,
    pub sender_id: Uuid,
    pub encrypted_payload: String,
    pub payload_type: ghostlink_core::types::PayloadType,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct GroupResponse {
    pub group_id: Uuid,
    pub name: String,
    pub members: Vec<GroupMemberResponse>,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct GroupMemberResponse {
    pub account_id: Uuid,
    pub username: String,
    pub role: String,
}

#[derive(Debug, Serialize)]
pub struct GroupInfoResponse {
    pub group_id: Uuid,
    pub name: String,
    pub member_count: i64,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct InviteLinkResponse {
    pub link: String,
    pub token: String,
    pub expires_at: String,
}
