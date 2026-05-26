use uuid::Uuid;

/// Per-connection session state machine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionState {
    /// Connection established, awaiting authentication
    Connected,
    /// JWT validated, user authenticated
    Authenticated { account_id: Uuid, username: String },
    /// Session terminated
    Disconnected,
}

/// Represents a single WebSocket session.
#[derive(Debug)]
pub struct WsSession {
    pub state: SessionState,
    pub connected_at: chrono::DateTime<chrono::Utc>,
}

impl WsSession {
    pub fn new() -> Self {
        Self {
            state: SessionState::Connected,
            connected_at: chrono::Utc::now(),
        }
    }

    pub fn authenticate(&mut self, account_id: Uuid, username: String) {
        self.state = SessionState::Authenticated {
            account_id,
            username,
        };
    }

    pub fn disconnect(&mut self) {
        self.state = SessionState::Disconnected;
    }

    pub fn account_id(&self) -> Option<Uuid> {
        match &self.state {
            SessionState::Authenticated { account_id, .. } => Some(*account_id),
            _ => None,
        }
    }

    pub fn is_authenticated(&self) -> bool {
        matches!(self.state, SessionState::Authenticated { .. })
    }
}

impl Default for WsSession {
    fn default() -> Self {
        Self::new()
    }
}
