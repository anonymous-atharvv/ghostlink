use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::protocol::WsMessage;

/// Per-connection sender handle
pub type WsSender = mpsc::UnboundedSender<WsMessage>;

/// Global WebSocket connection registry.
/// Uses DashMap for lock-free concurrent access across tokio tasks.
/// Supports multi-device: one account can have multiple active connections.
#[derive(Clone)]
pub struct ConnectionHub {
    /// account_id → list of device senders
    connections: Arc<DashMap<Uuid, Vec<WsSender>>>,
}

impl ConnectionHub {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(DashMap::new()),
        }
    }

    /// Register a new WebSocket connection for an account.
    pub fn register(&self, account_id: Uuid, tx: WsSender) {
        self.connections
            .entry(account_id)
            .or_default()
            .push(tx);
        tracing::debug!("WS registered for account (device count: {})",
            self.connections.get(&account_id).map(|v| v.len()).unwrap_or(0));
    }

    /// Unregister a WebSocket connection.
    pub fn unregister(&self, account_id: Uuid, tx: &WsSender) {
        if let Some(mut senders) = self.connections.get_mut(&account_id) {
            senders.retain(|s| !s.same_channel(tx));
            if senders.is_empty() {
                drop(senders);
                self.connections.remove(&account_id);
            }
        }
    }

    /// Send a message to all devices of an account.
    /// Returns true if the account had at least one active connection.
    pub fn send_to_account(&self, account_id: Uuid, msg: WsMessage) -> bool {
        if let Some(senders) = self.connections.get(&account_id) {
            let mut sent = false;
            for sender in senders.iter() {
                if sender.send(msg.clone()).is_ok() {
                    sent = true;
                }
            }
            sent
        } else {
            false
        }
    }

    /// Check if an account has any active connections on this pod.
    pub fn is_online_local(&self, account_id: Uuid) -> bool {
        self.connections.contains_key(&account_id)
    }

    /// Get total active connection count (for metrics).
    pub fn connection_count(&self) -> usize {
        self.connections
            .iter()
            .map(|entry| entry.value().len())
            .sum()
    }
}

impl Default for ConnectionHub {
    fn default() -> Self {
        Self::new()
    }
}
