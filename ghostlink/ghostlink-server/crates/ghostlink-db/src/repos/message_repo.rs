use scylla::Session;
use std::sync::Arc;
use uuid::Uuid;

/// Repository for message storage and offline queue operations.
#[derive(Clone)]
pub struct MessageRepo {
    session: Arc<Session>,
}

impl MessageRepo {
    pub fn new(session: Arc<Session>) -> Self {
        Self { session }
    }

    /// Store a message in the conversation history.
    pub async fn store_message(
        &self,
        conversation_id: Uuid,
        _message_id: Uuid,
        sender_id: Uuid,
        encrypted_payload: &[u8],
        payload_type: u8,
    ) -> anyhow::Result<()> {
        self.session
            .query(
                "INSERT INTO messages (conversation_id, message_id, sender_id, encrypted_payload, payload_type, status, created_at) \
                 VALUES (?, now(), ?, ?, ?, 0, toTimestamp(now()))",
                (conversation_id, sender_id, encrypted_payload.to_vec(), payload_type as i8),
            )
            .await?;
        Ok(())
    }

    /// Queue a message for an offline recipient (7-day TTL).
    pub async fn enqueue_offline(
        &self,
        recipient_id: Uuid,
        conversation_id: Uuid,
        sender_id: Uuid,
        encrypted_payload: &[u8],
        payload_type: u8,
    ) -> anyhow::Result<()> {
        self.session
            .query(
                "INSERT INTO offline_queue (recipient_id, message_id, conversation_id, sender_id, encrypted_payload, payload_type, created_at) \
                 VALUES (?, now(), ?, ?, ?, ?, toTimestamp(now()))",
                (recipient_id, conversation_id, sender_id, encrypted_payload.to_vec(), payload_type as i8),
            )
            .await?;
        Ok(())
    }

    /// Fetch all offline messages for a recipient.
    pub async fn fetch_offline(
        &self,
        recipient_id: Uuid,
    ) -> anyhow::Result<Vec<ghostlink_core::message::OfflineMessage>> {
        let result = self
            .session
            .query(
                "SELECT recipient_id, message_id, conversation_id, sender_id, encrypted_payload, payload_type, created_at \
                 FROM offline_queue WHERE recipient_id = ?",
                (recipient_id,),
            )
            .await?;

        let mut messages = Vec::new();
        let rows = result.rows_typed::<(Uuid, Uuid, Uuid, Uuid, Vec<u8>, i8, i64)>()?;
        for row_res in rows {
            if let Ok((recipient_id, message_id, conversation_id, sender_id, encrypted_payload, payload_type_val, created_at_ms)) = row_res {
                let payload_type = ghostlink_core::types::PayloadType::from_u8(payload_type_val as u8)
                    .unwrap_or(ghostlink_core::types::PayloadType::Text);
                messages.push(ghostlink_core::message::OfflineMessage {
                    recipient_id,
                    message_id,
                    conversation_id,
                    sender_id,
                    encrypted_payload,
                    payload_type,
                    created_at: chrono::DateTime::from_timestamp_millis(created_at_ms)
                        .unwrap_or_else(chrono::Utc::now),
                });
            }
        }
        Ok(messages)
    }

    /// Clear offline queue after acknowledgment.
    pub async fn clear_offline(&self, recipient_id: Uuid) -> anyhow::Result<()> {
        self.session
            .query(
                "DELETE FROM offline_queue WHERE recipient_id = ?",
                (recipient_id,),
            )
            .await?;
        Ok(())
    }
}
