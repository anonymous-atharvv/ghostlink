use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::types::{MessageStatus, PayloadType};

/// Core message entity.
/// The server only ever sees encrypted payloads — never plaintext.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub conversation_id: Uuid,
    pub message_id: Uuid,
    pub sender_id: Uuid,
    /// Signal Protocol encrypted ciphertext — server CANNOT decrypt
    pub encrypted_payload: Vec<u8>,
    pub payload_type: PayloadType,
    pub status: MessageStatus,
    pub created_at: DateTime<Utc>,
}

/// Message queued for an offline recipient.
/// Purged after 7 days TTL or upon acknowledgment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfflineMessage {
    pub recipient_id: Uuid,
    pub message_id: Uuid,
    pub conversation_id: Uuid,
    pub sender_id: Uuid,
    pub encrypted_payload: Vec<u8>,
    pub payload_type: PayloadType,
    pub created_at: DateTime<Utc>,
}

/// Disappearing message configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DisappearingTimer {
    Off,
    OneHour,
    TwentyFourHours,
    SevenDays,
    ThirtyDays,
}

impl DisappearingTimer {
    /// Returns TTL in seconds, None for Off
    pub fn ttl_seconds(&self) -> Option<i64> {
        match self {
            Self::Off => None,
            Self::OneHour => Some(3600),
            Self::TwentyFourHours => Some(86400),
            Self::SevenDays => Some(604800),
            Self::ThirtyDays => Some(2592000),
        }
    }
}
