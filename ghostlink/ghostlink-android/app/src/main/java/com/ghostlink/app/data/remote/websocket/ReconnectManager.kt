package com.ghostlink.app.data.remote.websocket

import android.os.Handler
import android.os.Looper
import kotlin.math.pow

class ReconnectManager(private val onReconnect: () -> Unit) {

    private val handler = Handler(Looper.getMainLooper())
    private var attempt = 0
    private val maxDelayMs = 30000L // Cap reconnect interval at 30 seconds
    private val baseDelayMs = 1000L
    private var isReconnecting = false

    fun scheduleReconnect() {
        if (isReconnecting) return
        isReconnecting = true
        
        // Exponential backoff with jitter
        val delay = (baseDelayMs * 2.0.pow(attempt.toDouble())).toLong().coerceAtMost(maxDelayMs)
        val jitter = (0..200).random()
        val totalDelay = delay + jitter

        handler.postDelayed({
            attempt++
            isReconnecting = false
            onReconnect()
        }, totalDelay)
    }

    fun reset() {
        attempt = 0
        isReconnecting = false
        handler.removeCallbacksAndMessages(null)
    }
}
