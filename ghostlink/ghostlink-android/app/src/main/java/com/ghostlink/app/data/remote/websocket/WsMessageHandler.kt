package com.ghostlink.app.data.remote.websocket

import com.ghostlink.app.crypto.SignalManager
import com.ghostlink.app.data.local.db.dao.MessageDao
import com.ghostlink.app.data.local.db.entity.MessageEntity
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import kotlinx.serialization.json.Json
import javax.inject.Inject
import javax.inject.Singleton

@Singleton
class WsMessageHandler @Inject constructor(
    private val messageDao: MessageDao,
    private val signalManager: SignalManager
) {
    private val scope = CoroutineScope(Dispatchers.IO)
    private val json = Json { ignoreUnknownKeys = true }

    fun handleIncomingJsonMessage(jsonText: String) {
        try {
            val wireMsg = json.decodeFromString<WsWireMessage>(jsonText)

            when (wireMsg.type) {
                "message.incoming" -> {
                    val incomingData = json.decodeFromString<IncomingMessagePayload>(wireMsg.payload)

                    val plainText = signalManager.decryptPayload(
                        incomingData.payload_ciphertext,
                        incomingData.sender_username
                    )

                    val entity = MessageEntity(
                        messageId = incomingData.message_id,
                        conversationId = incomingData.sender_username,
                        senderUsername = incomingData.sender_username,
                        payloadCiphertext = plainText,
                        status = 1,
                        createdAt = incomingData.created_at,
                        isDisappeared = false,
                        disappearTimerSeconds = 0
                    )

                    scope.launch {
                        messageDao.insertMessage(entity)
                    }
                }
                "message.ack" -> {
                    val ack = json.decodeFromString<MessageAckPayload>(wireMsg.payload)
                    scope.launch {
                        messageDao.updateMessageStatus(ack.message_id, ack.status)
                    }
                }
            }
        } catch (e: Exception) {
            e.printStackTrace()
        }
    }
}
