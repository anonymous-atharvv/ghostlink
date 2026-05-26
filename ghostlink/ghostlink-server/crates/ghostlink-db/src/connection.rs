use std::sync::Arc;

use deadpool_redis::{Config as RedisConfig, Pool as RedisPool, Runtime};
use scylla::transport::session::PoolSize;
use scylla::{Session, SessionBuilder};
use tracing::info;

/// Database configuration
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub scylla_nodes: Vec<String>,
    pub scylla_keyspace: String,
    pub scylla_username: Option<String>,
    pub scylla_password: Option<String>,
    pub redis_url: String,
}

/// Unified database handle — holds ScyllaDB session and Redis pool.
#[derive(Clone)]
pub struct Database {
    pub scylla: Arc<Session>,
    pub redis: RedisPool,
}

impl Database {
    /// Connect to ScyllaDB and Redis, run migrations.
    pub async fn connect(config: &DatabaseConfig) -> anyhow::Result<Self> {
        // ScyllaDB connection
        info!(
            nodes = ?config.scylla_nodes,
            keyspace = %config.scylla_keyspace,
            "Connecting to ScyllaDB"
        );

        let mut builder = SessionBuilder::new()
            .known_nodes(&config.scylla_nodes)
            .pool_size(PoolSize::PerHost(std::num::NonZeroUsize::new(3).unwrap()));

        if let (Some(user), Some(pass)) =
            (&config.scylla_username, &config.scylla_password)
        {
            builder = builder.user(user, pass);
        }

        let session = builder.build().await?;

        // Use the keyspace
        session
            .use_keyspace(&config.scylla_keyspace, false)
            .await
            .unwrap_or_else(|_| {
                tracing::warn!(
                    "Keyspace '{}' not found — migrations will create it",
                    config.scylla_keyspace
                );
            });

        info!("ScyllaDB connected successfully");

        // Redis connection pool
        info!(url = %config.redis_url, "Connecting to Redis");
        let redis_config = RedisConfig::from_url(&config.redis_url);
        let redis_pool = redis_config.create_pool(Some(Runtime::Tokio1))?;

        // Verify Redis connectivity
        let mut conn = redis_pool.get().await?;
        let _: String = redis::cmd("PING")
            .query_async(&mut conn)
            .await?;
        info!("Redis connected successfully");

        Ok(Self {
            scylla: Arc::new(session),
            redis: redis_pool,
        })
    }
}
