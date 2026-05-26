package com.ghostlink.app.data.local.db.entity

import androidx.room.Entity
import androidx.room.PrimaryKey

@Entity(tableName = "accounts")
data class AccountEntity(
    @PrimaryKey val id: String,
    val username: String,
    val lastSeenAt: Long
)

@Entity(tableName = "messages")
data class MessageEntity(
    @PrimaryKey val messageId: String,
    val conversationId: String, // can be user_id or group_id
    val senderUsername: String,
    val payloadCiphertext: String,
    val status: Int, // 0: Sent, 1: Delivered, 2: Read
    val createdAt: Long,
    val isDisappeared: Boolean,
    val disappearTimerSeconds: Long
)

@Entity(tableName = "contacts")
data class ContactEntity(
    @PrimaryKey val id: String,
    val contactUsername: String,
    val status: Int, // 0: PendingSent, 1: PendingReceived, 2: Accepted, 3: Blocked
    val createdAt: Long
)

@Entity(tableName = "groups")
data class GroupEntity(
    @PrimaryKey val groupId: String,
    val name: String,
    val avatarUrl: String?,
    val role: Int, // 0: Owner, 1: Admin, 2: Member
    val createdAt: Long
)
