package com.ghostlink.app.data.local.db

import androidx.room.Database
import androidx.room.RoomDatabase
import com.ghostlink.app.data.local.db.dao.*
import com.ghostlink.app.data.local.db.entity.*

@Database(
    entities = [
        AccountEntity::class,
        MessageEntity::class,
        ContactEntity::class,
        GroupEntity::class
    ],
    version = 1,
    exportSchema = false
)
abstract class GhostLinkDatabase : RoomDatabase() {
    abstract fun accountDao(): AccountDao
    abstract fun contactDao(): ContactDao
    abstract fun messageDao(): MessageDao
    abstract fun groupDao(): GroupDao
}
