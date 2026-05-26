package com.ghostlink.app.data.remote.dto

import kotlinx.serialization.Serializable

@Serializable
data class RegisterRequest(
    val username: String,
    val password: String,
    val identity_key: String,
    val signed_pre_key: SignedPreKeyDto,
    val one_time_pre_keys: List<OneTimePreKeyDto>
)

@Serializable
data class SignedPreKeyDto(
    val key_id: Int,
    val public_key: String,
    val signature: String
)

@Serializable
data class OneTimePreKeyDto(
    val key_id: Int,
    val public_key: String
)

@Serializable
data class LoginRequest(
    val username: String,
    val password: String
)

@Serializable
data class AuthResponse(
    val token: String,
    val account_id: String,
    val username: String
)

@Serializable
data class ContactDto(
    val contact_id: String,
    val username: String,
    val status: String,
    val added_at: String
)

@Serializable
data class AddContactRequest(
    val username: String
)

@Serializable
data class UpdateContactRequest(
    val action: String
)

@Serializable
data class PreKeyUploadRequest(
    val one_time_pre_keys: List<OneTimePreKeyDto>
)

@Serializable
data class KeyBundleResponse(
    val account_id: String,
    val identity_key: String,
    val signed_pre_key: SignedPreKeyDto,
    val one_time_pre_key: OneTimePreKeyDto?
)

@Serializable
data class OfflineMessageDto(
    val message_id: String,
    val conversation_id: String,
    val sender_id: String,
    val encrypted_payload: String,
    val payload_type: Int,
    val created_at: String
)
