package com.ghostlink.app.data.repository

import com.ghostlink.app.crypto.SignalManager
import com.ghostlink.app.data.local.db.dao.MessageDao
import com.ghostlink.app.data.local.db.entity.MessageEntity
import com.ghostlink.app.data.local.keystore.SessionStore
import com.ghostlink.app.data.remote.websocket.WsClient
import com.ghostlink.app.data.remote.websocket.WsWireMessage
import com.ghostlink.app.data.remote.websocket.IncomingMessagePayload
import kotlinx.coroutines.flow.Flow
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json
import java.util.UUID
import javax.inject.Inject
import javax.inject.Singleton

@Singleton
class ChatRepository @Inject constructor(
    private val messageDao: MessageDao,
    private val wsClient: WsClient,
    private val signalManager: SignalManager,
    private val sessionStore: SessionStore
) {
    private val json = Json { ignoreUnknownKeys = true }

    fun getMessagesForConversation(conversationId: String): Flow<List<MessageEntity>> {
        return messageDao.getMessagesForConversation(conversationId)
    }

    suspend fun sendMessage(recipientUsername: String, textContent: String): Result<Unit> {
        return try {
            val myUsername = sessionStore.getUsername() ?: throw IllegalStateException("User not authenticated")
            val messageId = UUID.randomUUID().toString()

            val cipherText = signalManager.encryptPayload(textContent, recipientUsername)

            val payload = json.encodeToString(
                OutboundMessagePayload.serializer(),
                OutboundMessagePayload(
                    message_id = messageId,
                    recipient_username = recipientUsername,
                    payload_ciphertext = cipherText,
                    created_at = System.currentTimeMillis()
                )
            )

            val wireMsg = json.encodeToString(
                WsWireMessage.serializer(),
                WsWireMessage(type = "message.send", payload = payload)
            )

            val success = wsClient.sendMessage(wireMsg)

            val localEntity = MessageEntity(
                messageId = messageId,
                conversationId = recipientUsername,
                senderUsername = myUsername,
                payloadCiphertext = textContent,
                status = if (success) 0 else -1,
                createdAt = System.currentTimeMillis(),
                isDisappeared = false,
                disappearTimerSeconds = 0
            )
            messageDao.insertMessage(localEntity)

            if (success) {
                Result.success(Unit)
            } else {
                Result.failure(Exception("WebSocket delivery offline. Queued locally."))
            }
        } catch (e: Exception) {
            Result.failure(e)
        }
    }
}
