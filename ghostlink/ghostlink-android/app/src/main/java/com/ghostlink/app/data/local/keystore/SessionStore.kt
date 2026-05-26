package com.ghostlink.app.data.local.keystore

import android.content.Context
import android.content.SharedPreferences

class SessionStore(
    context: Context,
    private val secureKeyStore: SecureKeyStore
) {
    private val prefsName = "ghostlink_secure_prefs"
    private val tokenKey = "enc_auth_token"
    private val usernameKey = "enc_username"
    private val dbKey = "enc_db_key"

    private val sharedPrefs: SharedPreferences = context.getSharedPreferences(prefsName, Context.MODE_PRIVATE)

    fun saveToken(token: String) {
        val encrypted = secureKeyStore.encrypt(token)
        sharedPrefs.edit().putString(tokenKey, encrypted).apply()
    }

    fun getToken(): String? {
        val encrypted = sharedPrefs.getString(tokenKey, null) ?: return null
        return try {
            secureKeyStore.decrypt(encrypted)
        } catch (e: Exception) {
            null
        }
    }

    fun saveUsername(username: String) {
        val encrypted = secureKeyStore.encrypt(username)
        sharedPrefs.edit().putString(usernameKey, encrypted).apply()
    }

    fun getUsername(): String? {
        val encrypted = sharedPrefs.getString(usernameKey, null) ?: return null
        return try {
            secureKeyStore.decrypt(encrypted)
        } catch (e: Exception) {
            null
        }
    }

    fun getDatabaseKey(): String {
        val encrypted = sharedPrefs.getString(dbKey, null)
        if (encrypted != null) {
            return try {
                secureKeyStore.decrypt(encrypted)
            } catch (e: Exception) {
                generateAndSaveNewDbKey()
            }
        }
        return generateAndSaveNewDbKey()
    }

    private fun generateAndSaveNewDbKey(): String {
        // Generate a cryptographically secure 256-bit key for SQLCipher
        val keyBytes = ByteArray(32)
        java.security.SecureRandom().nextBytes(keyBytes)
        val key = android.util.Base64.encodeToString(keyBytes, android.util.Base64.NO_WRAP)
        
        val encrypted = secureKeyStore.encrypt(key)
        sharedPrefs.edit().putString(dbKey, encrypted).apply()
        return key
    }

    fun clear() {
        sharedPrefs.edit().remove(tokenKey).remove(usernameKey).apply()
    }
}
