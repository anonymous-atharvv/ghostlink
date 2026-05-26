use deadpool_redis::Pool;
use redis::AsyncCommands;
use uuid::Uuid;

/// Redis-backed session cache for JWT token management.
#[derive(Clone)]
pub struct SessionCache {
    pool: Pool,
}

impl SessionCache {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    /// Store a session token with 30-day TTL.
    pub async fn store_session(
        &self,
        token_hash: &str,
        account_id: Uuid,
        device_id: Uuid,
    ) -> anyhow::Result<()> {
        let mut conn = self.pool.get().await?;
        let key = format!("session:{}", token_hash);
        let value = serde_json::json!({
            "account_id": account_id.to_string(),
            "device_id": device_id.to_string(),
            "created_at": chrono::Utc::now().to_rfc3339(),
        })
        .to_string();

        conn.set_ex::<_, _, ()>(&key, value, 86400 * 30).await?;
        Ok(())
    }

    /// Invalidate a session token.
    pub async fn invalidate(&self, token_hash: &str) -> anyhow::Result<()> {
        let mut conn = self.pool.get().await?;
        conn.del::<_, ()>(format!("session:{}", token_hash)).await?;
        Ok(())
    }

    /// Check if a session token exists (is valid).
    pub async fn exists(&self, token_hash: &str) -> anyhow::Result<bool> {
        let mut conn = self.pool.get().await?;
        let exists: bool = conn.exists(format!("session:{}", token_hash)).await?;
        Ok(exists)
    }

    /// Invalidate all sessions for an account (used on account deletion).
    pub async fn invalidate_all(&self, account_id: Uuid) -> anyhow::Result<()> {
        let mut conn = self.pool.get().await?;
        let _: () = redis::cmd("DEL")
            .arg(format!("session:{}:*", account_id))
            .query_async(&mut conn)
            .await?;
        Ok(())
    }
}
