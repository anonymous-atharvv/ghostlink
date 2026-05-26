use ghostlink_core::crypto::{OneTimePreKey, SignedPreKey};
use serde::Deserialize;
use validator::Validate;

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

#[derive(Debug, Deserialize, Validate)]
pub struct AddContactRequest {
    #[validate(length(min = 3, max = 32))]
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct RespondContactRequest {
    pub action: ghostlink_core::contact::ContactAction,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UploadPreKeysRequest {
    #[validate(length(min = 1, max = 100))]
    pub one_time_pre_keys: Vec<OneTimePreKey>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateGroupRequest {
    #[validate(length(min = 1, max = 64))]
    pub name: String,
    pub member_usernames: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct AddGroupMemberRequest {
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMemberRoleRequest {
    pub role: String,
}

#[derive(Debug, Deserialize)]
pub struct InviteLinkRequest {
    pub expires_hours: u32,
}
