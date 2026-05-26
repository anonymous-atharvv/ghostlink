use chrono::Utc;
use scylla::Session;
use std::sync::Arc;
use uuid::Uuid;

use ghostlink_core::account::Account;

/// Repository for account CRUD operations.
#[derive(Clone)]
pub struct AccountRepo {
    session: Arc<Session>,
}

impl AccountRepo {
    pub fn new(session: Arc<Session>) -> Self {
        Self { session }
    }

    /// Check if a username is already taken.
    pub async fn username_exists(&self, username: &str) -> anyhow::Result<bool> {
        let result = self
            .session
            .query(
                "SELECT account_id FROM username_index WHERE username = ?",
                (username,),
            )
            .await?;
        Ok(result.rows_num()? > 0)
    }

    /// Create a new account (insert into both accounts and username_index).
    pub async fn create(&self, account: &Account) -> anyhow::Result<()> {
        // Insert into username_index (acts as unique constraint)
        // Use IF NOT EXISTS for atomicity
        let applied = self
            .session
            .query(
                "INSERT INTO username_index (username, account_id) VALUES (?, ?) IF NOT EXISTS",
                (&account.username, account.id),
            )
            .await?;

        // Check if the LWT was applied
        if let Some(rows) = applied.rows {
            if let Some(row) = rows.into_iter().next() {
                let was_applied: bool = row.into_typed::<(bool,)>()?.0;
                if !was_applied {
                    return Err(anyhow::anyhow!("Username already taken"));
                }
            }
        }

        // Insert account record
        self.session
            .query(
                "INSERT INTO accounts (account_id, username, password_hash, created_at) VALUES (?, ?, ?, ?)",
                (account.id, &account.username, &account.password_hash, account.created_at.timestamp_millis()),
            )
            .await?;

        Ok(())
    }

    /// Find account by username.
    pub async fn find_by_username(&self, username: &str) -> anyhow::Result<Option<Account>> {
        // First look up account_id from username_index
        let id_result = self
            .session
            .query(
                "SELECT account_id FROM username_index WHERE username = ?",
                (username,),
            )
            .await?;

        let account_id: Uuid = match id_result.rows_typed::<(Uuid,)>()?.next() {
            Some(Ok((id,))) => id,
            _ => return Ok(None),
        };

        self.find_by_id(account_id).await
    }

    /// Find account by ID.
    pub async fn find_by_id(&self, account_id: Uuid) -> anyhow::Result<Option<Account>> {
        let result = self
            .session
            .query(
                "SELECT account_id, username, password_hash, created_at, last_seen_at FROM accounts WHERE account_id = ?",
                (account_id,),
            )
            .await?;

        match result
            .rows_typed::<(Uuid, String, String, i64, Option<i64>)>()?
            .next()
        {
            Some(Ok((id, username, password_hash, created_at_ms, last_seen_ms))) => {
                Ok(Some(Account {
                    id,
                    username,
                    password_hash,
                    created_at: chrono::DateTime::from_timestamp_millis(created_at_ms)
                        .unwrap_or_else(Utc::now),
                    last_seen_at: last_seen_ms
                        .and_then(chrono::DateTime::from_timestamp_millis),
                }))
            }
            _ => Ok(None),
        }
    }

    /// Update last_seen timestamp (no IP stored — privacy requirement).
    pub async fn update_last_seen(&self, account_id: Uuid) -> anyhow::Result<()> {
        let now = Utc::now().timestamp_millis();
        self.session
            .query(
                "UPDATE accounts SET last_seen_at = ? WHERE account_id = ?",
                (now, account_id),
            )
            .await?;
        Ok(())
    }

    /// Permanently delete account and username index entry.
    pub async fn delete(&self, account_id: Uuid, username: &str) -> anyhow::Result<()> {
        self.session
            .query(
                "DELETE FROM accounts WHERE account_id = ?",
                (account_id,),
            )
            .await?;
        self.session
            .query(
                "DELETE FROM username_index WHERE username = ?",
                (username,),
            )
            .await?;
        Ok(())
    }
}
