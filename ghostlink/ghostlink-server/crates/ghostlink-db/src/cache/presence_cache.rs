use deadpool_redis::Pool;
use redis::AsyncCommands;
use uuid::Uuid;

/// Redis-backed presence tracker for online/offline status.
/// Keys auto-expire after 65 seconds; clients refresh every 60s.
#[derive(Clone)]
pub struct PresenceCache {
    pool: Pool,
}

impl PresenceCache {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    /// Set user as online (65-second TTL, refreshed on heartbeat).
    pub async fn set_online(&self, account_id: Uuid) -> anyhow::Result<()> {
        let mut conn = self.pool.get().await?;
        let key = format!("presence:{}", account_id);
        conn.set_ex::<_, _, ()>(&key, "online", 65).await?;
        Ok(())
    }

    /// Set user as offline (remove presence key).
    pub async fn set_offline(&self, account_id: Uuid) -> anyhow::Result<()> {
        let mut conn = self.pool.get().await?;
        conn.del::<_, ()>(format!("presence:{}", account_id)).await?;
        Ok(())
    }

    /// Check if a user is currently online.
    pub async fn is_online(&self, account_id: Uuid) -> anyhow::Result<bool> {
        let mut conn = self.pool.get().await?;
        let exists: bool = conn.exists(format!("presence:{}", account_id)).await?;
        Ok(exists)
    }

    /// Cache the remaining pre-key count for an account.
    pub async fn set_pre_key_count(&self, account_id: Uuid, count: i64) -> anyhow::Result<()> {
        let mut conn = self.pool.get().await?;
        let key = format!("prekey_count:{}", account_id);
        conn.set_ex::<_, _, ()>(&key, count, 300).await?;
        Ok(())
    }

    /// Get cached pre-key count for an account.
    pub async fn get_pre_key_count(&self, account_id: Uuid) -> anyhow::Result<Option<i64>> {
        let mut conn = self.pool.get().await?;
        let key = format!("prekey_count:{}", account_id);
        let val: Option<i64> = conn.get(&key).await?;
        Ok(val)
    }
}
