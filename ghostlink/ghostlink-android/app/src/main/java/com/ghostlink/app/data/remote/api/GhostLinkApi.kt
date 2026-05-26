package com.ghostlink.app.data.remote.api

import com.ghostlink.app.data.remote.dto.*
import retrofit2.http.*

interface GhostLinkApi {

    @POST("auth/register")
    suspend fun register(@Body request: RegisterRequest): AuthResponse

    @POST("auth/login")
    suspend fun login(@Body request: LoginRequest): AuthResponse

    @GET("contacts")
    suspend fun getContacts(): List<ContactDto>

    @POST("contacts")
    suspend fun addContact(@Body request: AddContactRequest): ContactDto

    @PATCH("contacts/{id}")
    suspend fun updateContactStatus(
        @Path("id") id: String,
        @Body request: UpdateContactRequest
    ): ContactDto

    @DELETE("contacts/{id}")
    suspend fun deleteContact(@Path("id") id: String)

    @PUT("keys/pre-keys")
    suspend fun uploadPreKeys(@Body request: PreKeyUploadRequest)

    @GET("keys/{username}/bundle")
    suspend fun getKeyBundle(@Path("username") username: String): KeyBundleResponse

    @GET("messages/offline")
    suspend fun getOfflineMessages(): List<OfflineMessageDto>

    @DELETE("messages/offline")
    suspend fun ackOfflineMessages()
}
