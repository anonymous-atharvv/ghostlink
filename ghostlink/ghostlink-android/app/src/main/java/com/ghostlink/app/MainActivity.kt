package com.ghostlink.app

import android.os.Bundle
import android.view.WindowManager
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.MaterialTheme
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.navigation.compose.rememberNavController
import com.ghostlink.app.data.local.keystore.SessionStore
import com.ghostlink.app.ui.navigation.GhostLinkNavGraph
import dagger.hilt.android.AndroidEntryPoint
import javax.inject.Inject

@AndroidEntryPoint
class MainActivity : ComponentActivity() {

    @Inject
    lateinit var sessionStore: SessionStore

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        window.setFlags(
            WindowManager.LayoutParams.FLAG_SECURE,
            WindowManager.LayoutParams.FLAG_SECURE
        )

        enableEdgeToEdge()

        val isLoggedIn = sessionStore.getToken() != null

        setContent {
            val darkColors = androidx.compose.material3.darkColorScheme(
                primary = Color(0xFF64B5F6),
                secondary = Color(0xFF81C784),
                background = Color(0xFF0A0E17),
                surface = Color(0xFF161B22),
                error = Color(0xFFEF5350),
                onPrimary = Color(0xFF0D1B2A),
                onBackground = Color(0xFFE2E8F0),
                onSurface = Color(0xFFE2E8F0),
                onError = Color(0xFFFFFFFF)
            )

            MaterialTheme(
                colorScheme = darkColors
            ) {
                val navController = rememberNavController()

                Box(
                    modifier = Modifier
                        .fillMaxSize()
                        .background(MaterialTheme.colorScheme.background)
                ) {
                    GhostLinkNavGraph(
                        navController = navController,
                        isLoggedIn = isLoggedIn
                    )
                }
            }
        }
    }
}
