pub mod config;
pub mod error;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod router;

use std::sync::Arc;

use ghostlink_db::cache::{presence_cache::PresenceCache, session_cache::SessionCache};
use ghostlink_db::repos::{AccountRepo, ContactRepo, GroupRepo, KeyRepo, MessageRepo};
use ghostlink_db::Database;
use ghostlink_ws::ConnectionHub;

use crate::config::AppConfig;

/// Shared application state — passed to all handlers via Axum State.
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub db: Arc<Database>,
    pub account_repo: AccountRepo,
    pub key_repo: KeyRepo,
    pub message_repo: MessageRepo,
    pub contact_repo: ContactRepo,
    pub group_repo: GroupRepo,
    pub session_cache: SessionCache,
    pub presence_cache: PresenceCache,
    pub hub: ConnectionHub,
    pub nats: Option<async_nats::Client>,
}

impl AppState {
    pub fn new(config: AppConfig, db: Database, nats: Option<async_nats::Client>) -> Self {
        let scylla = db.scylla.clone();
        let redis = db.redis.clone();

        Self {
            config: Arc::new(config),
            account_repo: AccountRepo::new(scylla.clone()),
            key_repo: KeyRepo::new(scylla.clone()),
            message_repo: MessageRepo::new(scylla.clone()),
            contact_repo: ContactRepo::new(scylla.clone()),
            group_repo: GroupRepo::new(scylla),
            session_cache: SessionCache::new(redis.clone()),
            presence_cache: PresenceCache::new(redis),
            hub: ConnectionHub::new(),
            db: Arc::new(db),
            nats,
        }
    }
}
