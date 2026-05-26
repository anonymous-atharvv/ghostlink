use serde::{Deserialize, Serialize};

/// Payload type for messages
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[repr(u8)]
pub enum PayloadType {
    Text = 0,
    Image = 1,
    File = 2,
    Voice = 3,
    Video = 4,
}

impl PayloadType {
    pub fn from_u8(val: u8) -> Option<Self> {
        match val {
            0 => Some(Self::Text),
            1 => Some(Self::Image),
            2 => Some(Self::File),
            3 => Some(Self::Voice),
            4 => Some(Self::Video),
            _ => None,
        }
    }
}

/// Message delivery status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[repr(u8)]
pub enum MessageStatus {
    Sent = 0,
    Delivered = 1,
    Read = 2,
}

impl MessageStatus {
    pub fn from_u8(val: u8) -> Option<Self> {
        match val {
            0 => Some(Self::Sent),
            1 => Some(Self::Delivered),
            2 => Some(Self::Read),
            _ => None,
        }
    }
}

/// Contact relationship status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContactStatus {
    PendingSent,
    PendingReceived,
    Accepted,
    Blocked,
    Declined,
}

/// Group member role
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[repr(u8)]
pub enum GroupRole {
    Member = 0,
    Admin = 1,
    Owner = 2,
}

impl GroupRole {
    pub fn from_u8(val: u8) -> Option<Self> {
        match val {
            0 => Some(Self::Member),
            1 => Some(Self::Admin),
            2 => Some(Self::Owner),
            _ => None,
        }
    }

    /// Check if this role has admin-level permissions
    pub fn is_admin_or_above(&self) -> bool {
        matches!(self, Self::Admin | Self::Owner)
    }
}

/// Platform for push notifications
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[repr(u8)]
pub enum Platform {
    Ios = 0,
    Android = 1,
}

impl Platform {
    pub fn from_u8(val: u8) -> Option<Self> {
        match val {
            0 => Some(Self::Ios),
            1 => Some(Self::Android),
            _ => None,
        }
    }
}

/// Media type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaType {
    Image,
    Video,
    Audio,
    File,
}
