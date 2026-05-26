package com.ghostlink.app

import android.app.Application
import dagger.hilt.android.HiltAndroidApp
import net.zetetic.database.sqlcipher.RequerySQLiteOpenHelperFactory

@HiltAndroidApp
class GhostLinkApp : Application() {
    override fun onCreate() {
        super.onCreate()
        
        // Initialize SQLCipher database library for secure local SQLite storage
        System.loadLibrary("sqlcipher")
    }
}
