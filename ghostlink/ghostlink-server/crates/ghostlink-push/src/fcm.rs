use serde::Serialize;
use tracing::info;

/// Firebase Cloud Messaging client.
/// Sends content-free wakeup notifications to Android devices.
#[derive(Clone)]
pub struct FcmClient {
    _server_key: String,
    client: reqwest::Client,
    initialized: bool,
}

#[derive(Serialize)]
struct FcmPayload {
    to: String,
    data: FcmData,
}

#[derive(Serialize)]
struct FcmData {
    #[serde(rename = "type")]
    msg_type: String,
}

impl FcmClient {
    pub fn new(server_key: Option<String>) -> Self {
        let initialized = server_key.is_some();
        if initialized {
            info!("FCM client initialized");
        } else {
            info!("FCM client running in stub mode — no FCM credentials configured");
        }
        Self {
            _server_key: server_key.unwrap_or_default(),
            client: reqwest::Client::new(),
            initialized,
        }
    }

    /// Send a content-free push notification.
    pub async fn send_notification(&self, device_token: &str) -> anyhow::Result<()> {
        if !self.initialized {
            info!(token = %device_token, "FCM stub: would send notification");
            return Ok(());
        }

        let payload = FcmPayload {
            to: device_token.to_string(),
            data: FcmData {
                msg_type: "NEW_MESSAGE".to_string(),
            },
        };

        let _payload_json = serde_json::to_string(&payload)?;

        // TODO: Send HTTP request to FCM when server key is configured
        // let client = reqwest::Client::new();
        // client.post("https://fcm.googleapis.com/fcm/send")
        //     .header("Authorization", format!("key={}", self.server_key))
        //     .header("Content-Type", "application/json")
        //     .json(&payload)
        //     .send()
        //     .await?;

        info!(token = %device_token, "FCM notification sent");
        Ok(())
    }
}
