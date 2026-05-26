package com.ghostlink.app.data.remote.websocket

import kotlinx.serialization.Serializable

@Serializable
data class WsWireMessage(
    val type: String,
    val payload: String
)

@Serializable
data class IncomingMessagePayload(
    val message_id: String,
    val sender_username: String,
    val payload_ciphertext: String,
    val created_at: Long
)

@Serializable
data class MessageAckPayload(
    val message_id: String,
    val status: Int
)

@Serializable
data class OutboundMessagePayload(
    val message_id: String,
    val recipient_username: String,
    val payload_ciphertext: String,
    val created_at: Long
)
