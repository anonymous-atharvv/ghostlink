use uuid::Uuid;
use crate::hub::ConnectionHub;
use crate::protocol::WsMessage;
use ghostlink_db::repos::MessageRepo;
use ghostlink_db::cache::presence_cache::PresenceCache;

/// Routes incoming WebSocket messages to appropriate handlers.
pub struct WsRouter;

impl WsRouter {
    /// Dispatch a client message to the appropriate handler.
    pub async fn handle_message(
        hub: &ConnectionHub,
        nats_client: &Option<async_nats::Client>,
        message_repo: &MessageRepo,
        presence_cache: &PresenceCache,
        sender_id: Uuid,
        msg: WsMessage,
    ) {
        match msg {
            WsMessage::MessageSend {
                request_id,
                recipient_id,
                conversation_id,
                encrypted_payload,
                payload_type,
            } => {
                let message_id = Uuid::new_v4();
                let created_at_str = chrono::Utc::now().to_rfc3339();

                let incoming_msg = WsMessage::MessageIncoming {
                    message_id: message_id.to_string(),
                    conversation_id,
                    sender_id,
                    encrypted_payload: encrypted_payload.clone(),
                    payload_type,
                    created_at: created_at_str.clone(),
                };

                // 1. Try local delivery first
                let delivered = hub.send_to_account(recipient_id, incoming_msg.clone());

                // 2. Check if we need cross-pod or offline routing
                if delivered {
                    // Send ACK back to sender immediately
                    hub.send_to_account(
                        sender_id,
                        WsMessage::MessageAck {
                            request_id,
                            message_id: message_id.to_string(),
                            status: "delivered".to_string(),
                        },
                    );
                } else {
                    // Not connected locally. Check global online status in Redis presence cache
                    let globally_online = presence_cache.is_online(recipient_id).await.unwrap_or(false);

                    if globally_online {
                        // User is on another pod. Publish to NATS for cross-pod routing
                        if let Some(client) = nats_client {
                            let subject = format!("user.{}", recipient_id);
                            if let Ok(payload_bytes) = serde_json::to_vec(&incoming_msg) {
                                let _ = client.publish(subject, payload_bytes.into()).await;
                            }
                        }

                        // Send sent ACK back to sender (pending delivery check by recipient pod)
                        hub.send_to_account(
                            sender_id,
                            WsMessage::MessageAck {
                                request_id,
                                message_id: message_id.to_string(),
                                status: "sent".to_string(),
                            },
                        );
                    } else {
                        // Recipient is offline. Enqueue to offline_queue in ScyllaDB
                        if let Ok(payload_bytes) = base64::Engine::decode(
                            &base64::prelude::BASE64_STANDARD,
                            &encrypted_payload,
                        ) {
                            let _ = message_repo.enqueue_offline(
                                recipient_id,
                                conversation_id,
                                sender_id,
                                &payload_bytes,
                                payload_type,
                            ).await;
                        }

                        // Trigger push notification wakeup via NATS push.wakeup subject
                        if let Some(client) = nats_client {
                            let push_event = serde_json::json!({
                                "account_id": recipient_id.to_string(),
                            });
                            if let Ok(payload_bytes) = serde_json::to_vec(&push_event) {
                                let _ = client.publish("push.wakeup".to_string(), payload_bytes.into()).await;
                            }
                        }

                        // Send sent ACK back to sender
                        hub.send_to_account(
                            sender_id,
                            WsMessage::MessageAck {
                                request_id,
                                message_id: message_id.to_string(),
                                status: "sent".to_string(),
                            },
                        );
                    }
                }
            }

            WsMessage::TypingStart { conversation_id } => {
                // Route typing indicator to conversation partner
                tracing::debug!(
                    conversation = %conversation_id,
                    "Typing start indicator"
                );
            }

            WsMessage::TypingStop { conversation_id } => {
                tracing::debug!(
                    conversation = %conversation_id,
                    "Typing stop indicator"
                );
            }

            WsMessage::MessageRead {
                conversation_id,
                last_read_message_id,
            } => {
                tracing::debug!(
                    conversation = %conversation_id,
                    last_read = %last_read_message_id,
                    "Read receipt"
                );
            }

            WsMessage::Ping => {
                hub.send_to_account(sender_id, WsMessage::Pong);
            }

            _ => {
                tracing::warn!("Received unexpected server→client message from client");
            }
        }
    }
}
