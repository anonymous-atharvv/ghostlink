use futures::StreamExt;

use crate::hub::ConnectionHub;
use crate::protocol::WsMessage;

/// NATS bridge for cross-pod WebSocket message routing.
/// When a message recipient is on a different pod, the message is
/// published to NATS on the subject `user.{account_id}`.
/// All pods subscribe and check their local ConnectionHub.
pub struct NatsBridge {
    client: async_nats::Client,
    hub: ConnectionHub,
}

impl NatsBridge {
    pub async fn new(nats_url: &str, hub: ConnectionHub) -> anyhow::Result<Self> {
        let client = async_nats::connect(nats_url).await?;
        tracing::info!("NATS bridge connected to {}", nats_url);
        Ok(Self { client, hub })
    }

    /// Publish a message for cross-pod delivery.
    pub async fn publish_for_user(
        &self,
        account_id: uuid::Uuid,
        msg: &WsMessage,
    ) -> anyhow::Result<()> {
        let subject = format!("user.{}", account_id);
        let payload = serde_json::to_vec(msg)?;
        self.client
            .publish(subject, payload.into())
            .await?;
        Ok(())
    }

    /// Subscribe to user messages and deliver via local hub.
    /// Runs as a background task — call this once on startup.
    pub async fn start_subscriber(self) -> anyhow::Result<()> {
        let mut subscriber = self.client.subscribe("user.*").await?;
        tracing::info!("NATS subscriber started on user.*");

        while let Some(msg) = subscriber.next().await {
            // Extract account_id from subject "user.{uuid}"
            let account_id_str = msg
                .subject
                .as_str()
                .strip_prefix("user.")
                .unwrap_or_default();

            if let Ok(account_id) = account_id_str.parse::<uuid::Uuid>() {
                if let Ok(ws_msg) = serde_json::from_slice::<WsMessage>(&msg.payload) {
                    self.hub.send_to_account(account_id, ws_msg);
                }
            }
        }

        Ok(())
    }
}
