use scylla::Session;
use std::sync::Arc;
use tracing::info;

/// Run all CQL migrations in order.
/// Idempotent — uses IF NOT EXISTS for safety.
pub async fn run_migrations(session: &Arc<Session>, keyspace: &str) -> anyhow::Result<()> {
    info!("Running database migrations...");

    // Create keyspace if not exists
    session
        .query(
            format!(
                "CREATE KEYSPACE IF NOT EXISTS {} \
                 WITH replication = {{'class': 'SimpleStrategy', 'replication_factor': 1}}",
                keyspace
            ),
            &[],
        )
        .await?;

    session.use_keyspace(keyspace, false).await?;

    // Migration 001: Core schema
    let migration_001 = vec![
        "CREATE TABLE IF NOT EXISTS accounts (
            account_id UUID PRIMARY KEY,
            username TEXT,
            password_hash TEXT,
            created_at TIMESTAMP,
            last_seen_at TIMESTAMP
        )",
        "CREATE TABLE IF NOT EXISTS username_index (
            username TEXT PRIMARY KEY,
            account_id UUID
        )",
        "CREATE TABLE IF NOT EXISTS messages (
            conversation_id UUID,
            message_id TIMEUUID,
            sender_id UUID,
            encrypted_payload BLOB,
            payload_type TINYINT,
            status TINYINT,
            created_at TIMESTAMP,
            PRIMARY KEY (conversation_id, message_id)
        ) WITH CLUSTERING ORDER BY (message_id DESC)
          AND default_time_to_live = 2592000",
        "CREATE TABLE IF NOT EXISTS offline_queue (
            recipient_id UUID,
            message_id TIMEUUID,
            conversation_id UUID,
            encrypted_payload BLOB,
            sender_id UUID,
            payload_type TINYINT,
            created_at TIMESTAMP,
            PRIMARY KEY (recipient_id, message_id)
        ) WITH CLUSTERING ORDER BY (message_id ASC)
          AND default_time_to_live = 604800",
        "CREATE TABLE IF NOT EXISTS contacts (
            owner_id UUID,
            contact_id UUID,
            username TEXT,
            status TINYINT,
            added_at TIMESTAMP,
            PRIMARY KEY (owner_id, contact_id)
        )",
        "CREATE TABLE IF NOT EXISTS identity_keys (
            account_id UUID PRIMARY KEY,
            identity_key BLOB
        )",
        "CREATE TABLE IF NOT EXISTS signed_pre_keys (
            account_id UUID,
            key_id INT,
            public_key BLOB,
            signature BLOB,
            timestamp TIMESTAMP,
            PRIMARY KEY (account_id, key_id)
        )",
        "CREATE TABLE IF NOT EXISTS pre_keys (
            account_id UUID,
            key_id INT,
            public_key BLOB,
            PRIMARY KEY (account_id, key_id)
        )",
    ];

    for cql in &migration_001 {
        session.query(cql.to_string(), &[]).await?;
    }
    info!("Migration 001 (core schema) — applied");

    // Migration 002: Groups
    let migration_002 = vec![
        "CREATE TABLE IF NOT EXISTS groups (
            group_id UUID PRIMARY KEY,
            name TEXT,
            description TEXT,
            creator_id UUID,
            created_at TIMESTAMP,
            encrypted_avatar_key TEXT
        )",
        "CREATE TABLE IF NOT EXISTS group_members (
            group_id UUID,
            member_id UUID,
            username TEXT,
            role TINYINT,
            joined_at TIMESTAMP,
            PRIMARY KEY (group_id, member_id)
        )",
        "CREATE TABLE IF NOT EXISTS group_invites (
            token TEXT PRIMARY KEY,
            group_id UUID,
            created_by UUID,
            expires_at TIMESTAMP
        ) WITH default_time_to_live = 86400",
    ];

    for cql in &migration_002 {
        session.query(cql.to_string(), &[]).await?;
    }
    info!("Migration 002 (groups) — applied");

    // Migration 003: Media + push tokens
    let migration_003 = vec![
        "CREATE TABLE IF NOT EXISTS media (
            media_id UUID PRIMARY KEY,
            account_id UUID,
            media_type TINYINT,
            s3_key TEXT,
            encrypted_key TEXT,
            size_bytes BIGINT,
            created_at TIMESTAMP
        ) WITH default_time_to_live = 2592000",
        "CREATE TABLE IF NOT EXISTS push_tokens (
            account_id UUID,
            device_id UUID,
            platform TINYINT,
            token TEXT,
            updated_at TIMESTAMP,
            PRIMARY KEY (account_id, device_id)
        )",
    ];

    for cql in &migration_003 {
        session.query(cql.to_string(), &[]).await?;
    }
    info!("Migration 003 (media + push) — applied");

    info!("All migrations completed successfully");
    Ok(())
}
