use crate::apns::ApnsClient;
use crate::fcm::FcmClient;
use ghostlink_core::types::Platform;
use tracing::info;

/// Routes push notifications to the correct provider (APNs or FCM).
#[derive(Clone)]
pub struct PushDispatcher {
    apns: ApnsClient,
    fcm: FcmClient,
}

impl PushDispatcher {
    pub fn new(apns_team_id: Option<String>, apns_key_id: Option<String>, fcm_server_key: Option<String>) -> Self {
        Self {
            apns: ApnsClient::new(apns_team_id, apns_key_id),
            fcm: FcmClient::new(fcm_server_key),
        }
    }

    /// Dispatch a content-free wakeup notification.
    pub async fn dispatch(&self, platform: Platform, device_token: &str) -> anyhow::Result<()> {
        match platform {
            Platform::Ios => {
                self.apns.send_notification(device_token).await?;
            }
            Platform::Android => {
                self.fcm.send_notification(device_token).await?;
            }
        }
        info!(platform = ?platform, "Push notification dispatched");
        Ok(())
    }
}
