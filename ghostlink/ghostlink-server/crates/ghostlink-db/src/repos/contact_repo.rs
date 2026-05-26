use scylla::Session;
use std::sync::Arc;
use uuid::Uuid;

/// Repository for contact relationship management.
#[derive(Clone)]
pub struct ContactRepo {
    session: Arc<Session>,
}

impl ContactRepo {
    pub fn new(session: Arc<Session>) -> Self {
        Self { session }
    }

    /// Create a contact request (pending_sent for owner, pending_received for target).
    pub async fn create_request(
        &self,
        owner_id: Uuid,
        contact_id: Uuid,
        contact_username: &str,
        owner_username: &str,
    ) -> anyhow::Result<()> {
        // Owner side: pending_sent (status = 0)
        self.session
            .query(
                "INSERT INTO contacts (owner_id, contact_id, username, status, added_at) VALUES (?, ?, ?, 0, toTimestamp(now()))",
                (owner_id, contact_id, contact_username),
            )
            .await?;

        // Target side: pending_received (status = 1)
        self.session
            .query(
                "INSERT INTO contacts (owner_id, contact_id, username, status, added_at) VALUES (?, ?, ?, 1, toTimestamp(now()))",
                (contact_id, owner_id, owner_username),
            )
            .await?;

        Ok(())
    }

    /// Accept a contact request — update both sides to accepted (status = 2).
    pub async fn accept(&self, owner_id: Uuid, contact_id: Uuid) -> anyhow::Result<()> {
        self.session
            .query(
                "UPDATE contacts SET status = 2 WHERE owner_id = ? AND contact_id = ?",
                (owner_id, contact_id),
            )
            .await?;
        self.session
            .query(
                "UPDATE contacts SET status = 2 WHERE owner_id = ? AND contact_id = ?",
                (contact_id, owner_id),
            )
            .await?;
        Ok(())
    }

    /// Block a contact (status = 4).
    pub async fn block(&self, owner_id: Uuid, contact_id: Uuid) -> anyhow::Result<()> {
        self.session
            .query(
                "UPDATE contacts SET status = 4 WHERE owner_id = ? AND contact_id = ?",
                (owner_id, contact_id),
            )
            .await?;
        Ok(())
    }

    /// Remove a contact relationship.
    pub async fn remove(&self, owner_id: Uuid, contact_id: Uuid) -> anyhow::Result<()> {
        self.session
            .query(
                "DELETE FROM contacts WHERE owner_id = ? AND contact_id = ?",
                (owner_id, contact_id),
            )
            .await?;
        Ok(())
    }

    /// Check if two users are accepted contacts.
    pub async fn are_contacts(&self, user_a: Uuid, user_b: Uuid) -> anyhow::Result<bool> {
        let result = self
            .session
            .query(
                "SELECT status FROM contacts WHERE owner_id = ? AND contact_id = ?",
                (user_a, user_b),
            )
            .await?;

        match result.rows_typed::<(i8,)>()?.next() {
            Some(Ok((status,))) => Ok(status == 2), // accepted
            _ => Ok(false),
        }
    }

    /// List all contacts for an account.
    pub async fn list(&self, owner_id: Uuid) -> anyhow::Result<Vec<ghostlink_core::contact::Contact>> {
        let result = self
            .session
            .query(
                "SELECT contact_id, username, status, added_at FROM contacts WHERE owner_id = ?",
                (owner_id,),
            )
            .await?;

        let mut contacts = Vec::new();
        
        let rows = result.rows_typed::<(Uuid, String, i8, i64)>()?;
        for row_result in rows {
            if let Ok((contact_id, username, status_val, added_at_ms)) = row_result {
                let status = match status_val {
                    0 => ghostlink_core::types::ContactStatus::PendingSent,
                    1 => ghostlink_core::types::ContactStatus::PendingReceived,
                    2 => ghostlink_core::types::ContactStatus::Accepted,
                    3 => ghostlink_core::types::ContactStatus::Blocked,
                    4 => ghostlink_core::types::ContactStatus::Blocked,
                    _ => ghostlink_core::types::ContactStatus::Declined,
                };
                contacts.push(ghostlink_core::contact::Contact {
                    owner_id,
                    contact_id,
                    username,
                    status,
                    added_at: chrono::DateTime::from_timestamp_millis(added_at_ms)
                        .unwrap_or_else(chrono::Utc::now),
                });
            }
        }
        
        Ok(contacts)
    }
}
