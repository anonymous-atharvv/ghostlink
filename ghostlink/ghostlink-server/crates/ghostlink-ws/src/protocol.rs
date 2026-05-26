use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// All WebSocket message types (wire protocol).
/// Both client→server and server→client messages share this enum.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
#[serde(rename_all = "snake_case")]
pub enum WsMessage {
    // ── Client → Server ──

    /// Send an encrypted DM
    #[serde(rename = "message.send")]
    MessageSend {
        request_id: String,
        recipient_id: Uuid,
        conversation_id: Uuid,
        encrypted_payload: String, // Base64
        payload_type: u8,
    },

    /// Send an encrypted group message
    #[serde(rename = "group_message.send")]
    GroupMessageSend {
        request_id: String,
        group_id: Uuid,
        encrypted_payload: String, // Base64
        payload_type: u8,
    },

    /// Typing started
    #[serde(rename = "typing.start")]
    TypingStart { conversation_id: Uuid },

    /// Typing stopped
    #[serde(rename = "typing.stop")]
    TypingStop { conversation_id: Uuid },

    /// Mark messages as read
    #[serde(rename = "message.read")]
    MessageRead {
        conversation_id: Uuid,
        last_read_message_id: String,
    },

    /// Client keepalive ping
    #[serde(rename = "ping")]
    Ping,

    // ── Server → Client ──

    /// Incoming message notification
    #[serde(rename = "message.incoming")]
    MessageIncoming {
        message_id: String,
        conversation_id: Uuid,
        sender_id: Uuid,
        encrypted_payload: String,
        payload_type: u8,
        created_at: String,
    },

    /// Delivery acknowledgment
    #[serde(rename = "message.ack")]
    MessageAck {
        request_id: String,
        message_id: String,
        status: String, // "sent" | "delivered"
    },

    /// Typing indicator
    #[serde(rename = "typing.indicator")]
    TypingIndicator {
        conversation_id: Uuid,
        account_id: Uuid,
        is_typing: bool,
    },

    /// Server keepalive pong
    #[serde(rename = "pong")]
    Pong,

    /// Error response
    #[serde(rename = "error")]
    Error {
        request_id: Option<String>,
        code: String,
        message: String,
    },
}
