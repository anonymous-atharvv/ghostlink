package com.ghostlink.app.data.repository

import com.ghostlink.app.crypto.GeneratedKeyBundle
import com.ghostlink.app.crypto.SignalManager
import com.ghostlink.app.data.local.db.dao.AccountDao
import com.ghostlink.app.data.local.db.entity.AccountEntity
import com.ghostlink.app.data.local.keystore.SessionStore
import com.ghostlink.app.data.remote.api.GhostLinkApi
import com.ghostlink.app.data.remote.dto.*
import kotlinx.coroutines.flow.Flow
import javax.inject.Inject
import javax.inject.Singleton

@Singleton
class AuthRepository @Inject constructor(
    private val api: GhostLinkApi,
    private val sessionStore: SessionStore,
    private val signalManager: SignalManager,
    private val accountDao: AccountDao
) {
    fun getMyAccount(): Flow<AccountEntity?> = accountDao.getMyAccount()

    fun isLoggedIn(): Boolean = sessionStore.getToken() != null

    suspend fun register(username: String, passwordRaw: String): Result<Unit> {
        return try {
            val normalizedUsername = username.lowercase().trim()
            val keyBundle: GeneratedKeyBundle = signalManager.generateKeysAndRegister()

            val request = RegisterRequest(
                username = normalizedUsername,
                password = passwordRaw,
                identity_key = keyBundle.identityKey,
                signed_pre_key = SignedPreKeyDto(
                    key_id = keyBundle.signedPreKeyId,
                    public_key = keyBundle.signedPreKeyPublic,
                    signature = keyBundle.signedPreKeySignature
                ),
                one_time_pre_keys = keyBundle.oneTimePreKeys.map { (id, pub) ->
                    OneTimePreKeyDto(key_id = id, public_key = pub)
                }
            )

            val response = api.register(request)

            sessionStore.saveToken(response.token)
            sessionStore.saveUsername(normalizedUsername)

            val localAccount = AccountEntity(
                id = normalizedUsername,
                username = normalizedUsername,
                lastSeenAt = System.currentTimeMillis()
            )
            accountDao.insertAccount(localAccount)

            Result.success(Unit)
        } catch (e: Exception) {
            Result.failure(e)
        }
    }

    suspend fun login(username: String, passwordRaw: String): Result<Unit> {
        return try {
            val normalizedUsername = username.lowercase().trim()

            val request = LoginRequest(
                username = normalizedUsername,
                password = passwordRaw
            )

            val response = api.login(request)

            sessionStore.saveToken(response.token)
            sessionStore.saveUsername(normalizedUsername)

            val localAccount = AccountEntity(
                id = normalizedUsername,
                username = normalizedUsername,
                lastSeenAt = System.currentTimeMillis()
            )
            accountDao.insertAccount(localAccount)

            Result.success(Unit)
        } catch (e: Exception) {
            Result.failure(e)
        }
    }

    suspend fun logout() {
        sessionStore.clear()
        accountDao.clear()
    }
}
