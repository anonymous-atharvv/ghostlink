package com.ghostlink.app.di

import android.content.Context
import com.ghostlink.app.crypto.KeyBundleManager
import com.ghostlink.app.crypto.MediaEncryptor
import com.ghostlink.app.crypto.SignalManager
import dagger.Module
import dagger.Provides
import dagger.hilt.InstallIn
import dagger.hilt.android.qualifiers.ApplicationContext
import dagger.hilt.components.SingletonComponent
import javax.inject.Singleton

@Module
@InstallIn(SingletonComponent::class)
object CryptoModule {

    @Provides
    @Singleton
    fun provideSignalManager(@ApplicationContext context: Context): SignalManager {
        return SignalManager(context)
    }

    @Provides
    @Singleton
    fun provideKeyBundleManager(signalManager: SignalManager): KeyBundleManager {
        return KeyBundleManager(signalManager)
    }

    @Provides
    @Singleton
    fun provideMediaEncryptor(): MediaEncryptor {
        return MediaEncryptor()
    }
}
