package com.ghostlink.app.di

import com.ghostlink.app.data.local.keystore.SessionStore
import com.ghostlink.app.data.remote.api.AuthInterceptor
import com.ghostlink.app.data.remote.api.GhostLinkApi
import com.ghostlink.app.data.remote.websocket.WsClient
import com.ghostlink.app.data.remote.websocket.WsMessageHandler
import dagger.Module
import dagger.Provides
import dagger.hilt.InstallIn
import dagger.hilt.components.SingletonComponent
import okhttp3.OkHttpClient
import okhttp3.logging.HttpLoggingInterceptor
import retrofit2.Retrofit
import retrofit2.converter.gson.GsonConverterFactory
import java.util.concurrent.TimeUnit
import javax.inject.Singleton

@Module
@InstallIn(SingletonComponent::class)
object NetworkModule {

    private const val BASE_URL = "https://api.ghostlink.app/v1/" // Production gateway

    @Provides
    @Singleton
    fun provideHttpLoggingInterceptor(): HttpLoggingInterceptor {
        return HttpLoggingInterceptor().apply {
            level = HttpLoggingInterceptor.Level.BASIC // Exclude payload logs to comply with strict zero-PII
        }
    }

    @Provides
    @Singleton
    fun provideAuthInterceptor(sessionStore: SessionStore): AuthInterceptor {
        return AuthInterceptor(sessionStore)
    }

    @Provides
    @Singleton
    fun provideOkHttpClient(
        loggingInterceptor: HttpLoggingInterceptor,
        authInterceptor: AuthInterceptor
    ): OkHttpClient {
        return OkHttpClient.Builder()
            .addInterceptor(authInterceptor)
            .addInterceptor(loggingInterceptor)
            .connectTimeout(15, TimeUnit.SECONDS)
            .readTimeout(15, TimeUnit.SECONDS)
            .writeTimeout(15, TimeUnit.SECONDS)
            .build()
    }

    @Provides
    @Singleton
    fun provideRetrofit(okHttpClient: OkHttpClient): Retrofit {
        return Retrofit.Builder()
            .baseUrl(BASE_URL)
            .client(okHttpClient)
            .addConverterFactory(GsonConverterFactory.create())
            .build()
    }

    @Provides
    @Singleton
    fun provideGhostLinkApi(retrofit: Retrofit): GhostLinkApi {
        return retrofit.create(GhostLinkApi::class.java)
    }

    @Provides
    @Singleton
    fun provideWsClient(
        sessionStore: SessionStore,
        wsMessageHandler: WsMessageHandler
    ): WsClient {
        return WsClient(sessionStore, wsMessageHandler)
    }
}
