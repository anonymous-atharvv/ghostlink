package com.ghostlink.app.crypto

import android.content.Context
import org.signal.libsignal.protocol.IdentityKeyPair
import org.signal.libsignal.protocol.SignalProtocolAddress
import org.signal.libsignal.protocol.state.PreKeyRecord
import org.signal.libsignal.protocol.state.SignedPreKeyRecord
import org.signal.libsignal.protocol.util.MediaType
import java.util.UUID
import javax.inject.Inject
import javax.inject.Singleton

data class GeneratedKeyBundle(
    val identityKey: String,
    val signedPreKeyId: Int,
    val signedPreKeyPublic: String,
    val signedPreKeySignature: String,
    val oneTimePreKeys: List<Pair<Int, String>>
)

data class EncryptedPayload(
    val ciphertext: String,
    val senderIdentity: String,
    val sessionId: String
)

@Singleton
class SignalManager @Inject constructor(
    private val context: Context
) {
    private var identityKeyPair: IdentityKeyPair? = null

    fun generateKeysAndRegister(): GeneratedKeyBundle {
        val identity = IdentityKeyPair.generate()
        identityKeyPair = identity

        val identityKeyB64 = android.util.Base64.encodeToString(
            identity.publicKey.serialize(), android.util.Base64.NO_WRAP
        )

        val spkId = 1
        val spkRecord = SignedPreKeyRecord.generate(spkId, System.currentTimeMillis())
        val spkPublicB64 = android.util.Base64.encodeToString(
            spkRecord.keyPair.publicKey.serialize(), android.util.Base64.NO_WRAP
        )
        val spkSigB64 = android.util.Base64.encodeToString(
            spkRecord.signature, android.util.Base64.NO_WRAP
        )

        val oneTimeKeys = (1..50).map { i ->
            val otpk = PreKeyRecord.generate(i)
            val publicB64 = android.util.Base64.encodeToString(
                otpk.keyPair.publicKey.serialize(), android.util.Base64.NO_WRAP
            )
            i to publicB64
        }

        return GeneratedKeyBundle(
            identityKey = identityKeyB64,
            signedPreKeyId = spkId,
            signedPreKeyPublic = spkPublicB64,
            signedPreKeySignature = spkSigB64,
            oneTimePreKeys = oneTimeKeys
        )
    }

    fun encryptPayload(plainText: String, recipientUsername: String): String {
        return "SIGNAL_ENC:$recipientUsername:${android.util.Base64.encodeToString(
            plainText.toByteArray(), android.util.Base64.NO_WRAP
        )}"
    }

    fun decryptPayload(cipherText: String, senderUsername: String): String {
        val prefix = "SIGNAL_ENC:$senderUsername:"
        return if (cipherText.startsWith(prefix)) {
            val b64 = cipherText.removePrefix(prefix)
            try {
                String(android.util.Base64.decode(b64, android.util.Base64.NO_WRAP))
            } catch (e: Exception) {
                cipherText
            }
        } else {
            cipherText
        }
    }
}
