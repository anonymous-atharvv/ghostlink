package com.ghostlink.app.di

import android.content.Context
import androidx.room.Room
import com.ghostlink.app.data.local.db.GhostLinkDatabase
import com.ghostlink.app.data.local.db.dao.*
import com.ghostlink.app.data.local.keystore.SecureKeyStore
import com.ghostlink.app.data.local.keystore.SessionStore
import dagger.Module
import dagger.Provides
import dagger.hilt.InstallIn
import dagger.hilt.android.qualifiers.ApplicationContext
import dagger.hilt.components.SingletonComponent
import net.zetetic.database.sqlcipher.SupportOpenHelperFactory
import javax.inject.Singleton

@Module
@InstallIn(SingletonComponent::class)
object DatabaseModule {

    @Provides
    @Singleton
    fun provideSecureKeyStore(@ApplicationContext context: Context): SecureKeyStore {
        return SecureKeyStore(context)
    }

    @Provides
    @Singleton
    fun provideSessionStore(
        @ApplicationContext context: Context,
        secureKeyStore: SecureKeyStore
    ): SessionStore {
        return SessionStore(context, secureKeyStore)
    }

    @Provides
    @Singleton
    fun provideGhostLinkDatabase(
        @ApplicationContext context: Context,
        sessionStore: SessionStore
    ): GhostLinkDatabase {
        // Enforce strong SQLCipher SQLite encryption with hardware-derived secure passphrase
        val passphrase = sessionStore.getDatabaseKey().toByteArray(Charsets.UTF_8)
        val factory = SupportOpenHelperFactory(passphrase)
        
        return Room.databaseBuilder(
            context,
            GhostLinkDatabase::class.java,
            "ghostlink.db"
        )
            .openHelperFactory(factory)
            .fallbackToDestructiveMigration() // Reset cleanly on migrations during bootstrap
            .build()
    }

    @Provides
    fun provideAccountDao(db: GhostLinkDatabase): AccountDao = db.accountDao()

    @Provides
    fun provideContactDao(db: GhostLinkDatabase): ContactDao = db.contactDao()

    @Provides
    fun provideMessageDao(db: GhostLinkDatabase): MessageDao = db.messageDao()

    @Provides
    fun provideGroupDao(db: GhostLinkDatabase): GroupDao = db.groupDao()
}
