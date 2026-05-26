package com.ghostlink.app.ui.navigation

import androidx.compose.runtime.Composable
import androidx.navigation.NavHostController
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import com.ghostlink.app.ui.screens.chat.ChatListScreen
import com.ghostlink.app.ui.screens.chat.ConversationScreen
import com.ghostlink.app.ui.screens.onboarding.LoginScreen
import com.ghostlink.app.ui.screens.onboarding.RegisterScreen
import com.ghostlink.app.ui.screens.onboarding.WelcomeScreen
import com.ghostlink.app.ui.screens.settings.SettingsScreen

object Routes {
    const val WELCOME = "welcome"
    const val LOGIN = "login"
    const val REGISTER = "register"
    const val CHAT_LIST = "chat_list"
    const val CONVERSATION = "conversation/{username}"
    const val SETTINGS = "settings"

    fun conversation(username: String) = "conversation/$username"
}

@Composable
fun GhostLinkNavGraph(
    navController: NavHostController,
    startDestination: String = Routes.WELCOME,
    isLoggedIn: Boolean
) {
    val actualStart = if (isLoggedIn) Routes.CHAT_LIST else startDestination

    NavHost(navController = navController, startDestination = actualStart) {
        composable(Routes.WELCOME) {
            WelcomeScreen(
                onLogin = { navController.navigate(Routes.LOGIN) },
                onRegister = { navController.navigate(Routes.REGISTER) }
            )
        }
        composable(Routes.LOGIN) {
            LoginScreen(
                onSuccess = {
                    navController.navigate(Routes.CHAT_LIST) {
                        popUpTo(Routes.WELCOME) { inclusive = true }
                    }
                }
            )
        }
        composable(Routes.REGISTER) {
            RegisterScreen(
                onSuccess = {
                    navController.navigate(Routes.CHAT_LIST) {
                        popUpTo(Routes.WELCOME) { inclusive = true }
                    }
                }
            )
        }
        composable(Routes.CHAT_LIST) {
            ChatListScreen(
                onConversationClick = { username ->
                    navController.navigate(Routes.conversation(username))
                },
                onSettingsClick = {
                    navController.navigate(Routes.SETTINGS)
                }
            )
        }
        composable(Routes.CONVERSATION) { backStackEntry ->
            val username = backStackEntry.arguments?.getString("username") ?: ""
            ConversationScreen(
                contactUsername = username,
                onBack = { navController.popBackStack() }
            )
        }
        composable(Routes.SETTINGS) {
            SettingsScreen(
                onBack = { navController.popBackStack() },
                onLogout = {
                    navController.navigate(Routes.WELCOME) {
                        popUpTo(0) { inclusive = true }
                    }
                }
            )
        }
    }
}
