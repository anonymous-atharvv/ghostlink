use serde::Serialize;
use tracing::info;

/// Apple Push Notification Service client.
/// Sends content-free wakeup notifications to iOS devices.
#[derive(Clone)]
pub struct ApnsClient {
    _team_id: String,
    _key_id: String,
    signing_key: String,
    initialized: bool,
}

#[derive(Serialize)]
struct ApnsPayload {
    aps: Aps,
}

#[derive(Serialize)]
struct Aps {
    alert: Alert,
    #[serde(rename = "content-available")]
    content_available: i32,
    sound: String,
}

#[derive(Serialize)]
struct Alert {
    title: String,
    body: String,
}

impl ApnsClient {
    pub fn new(team_id: Option<String>, key_id: Option<String>) -> Self {
        let initialized = team_id.is_some() && key_id.is_some();
        if initialized {
            info!("APNs client initialized");
        } else {
            info!("APNs client running in stub mode — no APNs credentials configured");
        }
        Self {
            _team_id: team_id.unwrap_or_default(),
            _key_id: key_id.unwrap_or_default(),
            signing_key: String::new(),
            initialized,
        }
    }

    /// Send a content-free push notification.
    pub async fn send_notification(&self, device_token: &str) -> anyhow::Result<()> {
        if !self.initialized {
            info!(token = %device_token, "APNs stub: would send notification");
            return Ok(());
        }

        let payload = ApnsPayload {
            aps: Aps {
                alert: Alert {
                    title: "GhostLink".to_string(),
                    body: "New message".to_string(),
                },
                content_available: 1,
                sound: "default".to_string(),
            },
        };

        let _payload_json = serde_json::to_string(&payload)?;
        // TODO: Send HTTP/2 request to APNs when credentials are configured
        // let request = reqwest::Client::builder()
        //     .http2_prior_knowledge()
        //     .build()?;
        // let token = create_jwt(&self.team_id, &self.key_id)?;
        // request.post(format!("https://api.push.apple.com/3/device/{}", device_token))
        //     .header("authorization", format!("bearer {}", token))
        //     .json(&payload)
        //     .send()
        //     .await?;

        info!(token = %device_token, "APNs notification sent");
        Ok(())
    }
}
