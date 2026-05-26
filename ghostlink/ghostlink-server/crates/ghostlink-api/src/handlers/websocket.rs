use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt};
use jsonwebtoken::{decode, DecodingKey, Validation};
use base64::Engine;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::AppState;
use ghostlink_core::account::AuthenticatedAccount;
use ghostlink_ws::WsMessage;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    iat: usize,
}

/// GET /ws/connect
/// Handshake, authenticate JWT, and upgrade to a stateful WebSocket connection.
pub async fn ws_upgrade(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: AppState) {
    let (mut ws_sender, mut ws_receiver) = socket.split();

    // 1. JWT Authentication Handshake
    // Allow a 5-second window to authenticate
    let mut authenticated_user: Option<AuthenticatedAccount> = None;
    let auth_timeout = tokio::time::sleep(tokio::time::Duration::from_secs(5));
    tokio::pin!(auth_timeout);

    loop {
        tokio::select! {
            _ = &mut auth_timeout => {
                let err_msg = WsMessage::Error {
                    request_id: None,
                    code: "AUTH_TIMEOUT".to_string(),
                    message: "Authentication timeout".to_string(),
                };
                if let Ok(text) = serde_json::to_string(&err_msg) {
                    let _ = ws_sender.send(Message::Text(text)).await;
                }
                return;
            }
            msg_opt = ws_receiver.next() => {
                match msg_opt {
                    Some(Ok(Message::Text(text))) => {
                        #[derive(Deserialize)]
                        struct AuthPayload {
                            token: String,
                        }
                        #[derive(Deserialize)]
                        struct AuthMessage {
                            #[serde(rename = "type")]
                            msg_type: String,
                            payload: AuthPayload,
                        }

                        let mut success = false;
                        if let Ok(auth_msg) = serde_json::from_str::<AuthMessage>(&text) {
                            if auth_msg.msg_type == "auth" {
                                if let Ok(token_data) = decode::<Claims>(
                                    &auth_msg.payload.token,
                                    &DecodingKey::from_secret(state.config.jwt_secret.as_bytes()),
                                    &Validation::default(),
                                ) {
                                    if let Ok(account_id) = token_data.claims.sub.parse::<Uuid>() {
                                        if let Ok(Some(account)) = state.account_repo.find_by_id(account_id).await {
                                            authenticated_user = Some(AuthenticatedAccount {
                                                id: account.id,
                                                username: account.username,
                                            });
                                            success = true;
                                        }
                                    }
                                }
                            }
                        }

                        if success {
                            break;
                        } else {
                            let err_msg = WsMessage::Error {
                                request_id: None,
                                code: "AUTH_FAILED".to_string(),
                                message: "Invalid credentials or token format".to_string(),
                            };
                            if let Ok(text) = serde_json::to_string(&err_msg) {
                                let _ = ws_sender.send(Message::Text(text)).await;
                            }
                            return;
                        }
                    }
                    _ => return, // closed or error
                }
            }
        }
    }

    let user = authenticated_user.unwrap();
    let account_id = user.id;

    // 2. Active state: Create unbounded channel for outgoing message deliveries
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<WsMessage>();

    // Register with local connection hub
    state.hub.register(account_id, tx.clone());

    // Update global Redis presence state to online
    let _ = state.presence_cache.set_online(account_id).await;

    // Fetch and drain all pending offline messages on connect
    if let Ok(offline_msgs) = state.message_repo.fetch_offline(account_id).await {
        for m in offline_msgs {
            let ws_msg = WsMessage::MessageIncoming {
                message_id: m.message_id.to_string(),
                conversation_id: m.conversation_id,
                sender_id: m.sender_id,
                encrypted_payload: base64::prelude::BASE64_STANDARD.encode(&m.encrypted_payload),
                payload_type: m.payload_type as u8,
                created_at: m.created_at.to_rfc3339(),
            };
            let _ = tx.send(ws_msg);
        }
        // Purge delivered items from the queue
        let _ = state.message_repo.clear_offline(account_id).await;
    }

    // 3. Drive Concurrent Write/Read Event Loops
    let (mut ws_sink, mut ws_stream) = (ws_sender, ws_receiver);

    let write_loop = async move {
        while let Some(msg) = rx.recv().await {
            if let Ok(serialized) = serde_json::to_string(&msg) {
                if ws_sink.send(Message::Text(serialized)).await.is_err() {
                    break;
                }
            }
        }
    };

    let hub_clone = state.hub.clone();
    let nats_clone = state.nats.clone();
    let message_repo_clone = state.message_repo.clone();
    let presence_cache_clone = state.presence_cache.clone();
    let tx_clone = tx.clone();

    let read_loop = async move {
        while let Some(msg_res) = ws_stream.next().await {
            match msg_res {
                Ok(Message::Text(text)) => {
                    if let Ok(client_msg) = serde_json::from_str::<WsMessage>(&text) {
                        ghostlink_ws::router::WsRouter::handle_message(
                            &hub_clone,
                            &nats_clone,
                            &message_repo_clone,
                            &presence_cache_clone,
                            account_id,
                            client_msg,
                        ).await;
                    }
                }
                Ok(Message::Close(_)) | Err(_) => {
                    break;
                }
                _ => {}
            }
        }
    };

    tokio::join!(write_loop, read_loop);

    // 4. Disconnect Cleanup
    state.hub.unregister(account_id, &tx_clone);
    let _ = state.presence_cache.set_offline(account_id).await;
}
