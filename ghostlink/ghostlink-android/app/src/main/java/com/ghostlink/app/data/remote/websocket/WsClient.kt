package com.ghostlink.app.data.remote.websocket

import com.ghostlink.app.data.local.keystore.SessionStore
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.MutableSharedFlow
import kotlinx.coroutines.flow.SharedFlow
import kotlinx.coroutines.launch
import okhttp3.*
import java.util.concurrent.TimeUnit
import javax.inject.Inject
import javax.inject.Singleton

@Singleton
class WsClient @Inject constructor(
    private val sessionStore: SessionStore,
    private val wsMessageHandler: WsMessageHandler
) {
    private var client: OkHttpClient? = null
    private var webSocket: WebSocket? = null
    private val reconnectManager = ReconnectManager { connect() }

    private val _connectionState = MutableSharedFlow<Boolean>(replay = 1)
    val connectionState: SharedFlow<Boolean> = _connectionState

    private val scope = CoroutineScope(Dispatchers.IO)

    fun connect() {
        val token = sessionStore.getToken() ?: return

        val request = Request.Builder()
            .url("wss://api.ghostlink.app/v1/ws/connect")
            .header("Authorization", "Bearer $token")
            .build()

        client = OkHttpClient.Builder()
            .pingInterval(30, TimeUnit.SECONDS)
            .readTimeout(0, TimeUnit.MILLISECONDS)
            .build()

        webSocket = client?.newWebSocket(request, object : WebSocketListener() {
            override fun onOpen(webSocket: WebSocket, response: Response) {
                reconnectManager.reset()
                scope.launch { _connectionState.emit(true) }
            }

            override fun onMessage(webSocket: WebSocket, text: String) {
                wsMessageHandler.handleIncomingJsonMessage(text)
            }

            override fun onClosing(webSocket: WebSocket, code: Int, reason: String) {
                webSocket.close(1000, null)
                scope.launch { _connectionState.emit(false) }
            }

            override fun onFailure(webSocket: WebSocket, t: Throwable, response: Response?) {
                scope.launch { _connectionState.emit(false) }
                reconnectManager.scheduleReconnect()
            }
        })
    }

    fun sendMessage(jsonPayload: String): Boolean {
        return webSocket?.send(jsonPayload) ?: false
    }

    fun disconnect() {
        reconnectManager.reset()
        webSocket?.close(1000, "User disconnected")
        scope.launch { _connectionState.emit(false) }
    }
}
