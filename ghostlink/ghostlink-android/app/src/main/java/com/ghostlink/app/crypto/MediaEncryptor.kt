package com.ghostlink.app.crypto

import java.io.InputStream
import java.io.OutputStream
import javax.crypto.Cipher
import javax.crypto.spec.IvParameterSpec
import javax.crypto.spec.SecretKeySpec
import javax.inject.Inject
import javax.inject.Singleton

@Singleton
class MediaEncryptor @Inject constructor() {

    private val transformation = "AES/CBC/PKCS5Padding"

    fun encryptStream(input: InputStream, output: OutputStream, key: ByteArray, iv: ByteArray) {
        val cipher = Cipher.getInstance(transformation)
        val secretKeySpec = SecretKeySpec(key, "AES")
        val ivParameterSpec = IvParameterSpec(iv)
        cipher.init(Cipher.ENCRYPT_MODE, secretKeySpec, ivParameterSpec)
        
        val buffer = ByteArray(4096)
        var bytesRead: Int
        while (input.read(buffer).also { bytesRead = it } != -1) {
            val encrypted = cipher.update(buffer, 0, bytesRead)
            if (encrypted != null) {
                output.write(encrypted)
            }
        }
        val finalBytes = cipher.doFinal()
        if (finalBytes != null) {
            output.write(finalBytes)
        }
    }

    fun decryptStream(input: InputStream, output: OutputStream, key: ByteArray, iv: ByteArray) {
        val cipher = Cipher.getInstance(transformation)
        val secretKeySpec = SecretKeySpec(key, "AES")
        val ivParameterSpec = IvParameterSpec(iv)
        cipher.init(Cipher.DECRYPT_MODE, secretKeySpec, ivParameterSpec)
        
        val buffer = ByteArray(4096)
        var bytesRead: Int
        while (input.read(buffer).also { bytesRead = it } != -1) {
            val decrypted = cipher.update(buffer, 0, bytesRead)
            if (decrypted != null) {
                output.write(decrypted)
            }
        }
        val finalBytes = cipher.doFinal()
        if (finalBytes != null) {
            output.write(finalBytes)
        }
    }
}
