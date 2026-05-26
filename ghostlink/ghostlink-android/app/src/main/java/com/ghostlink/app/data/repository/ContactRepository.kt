package com.ghostlink.app.data.repository

import com.ghostlink.app.data.local.db.dao.ContactDao
import com.ghostlink.app.data.local.db.entity.ContactEntity
import com.ghostlink.app.data.remote.api.GhostLinkApi
import com.ghostlink.app.data.remote.dto.AddContactRequest
import com.ghostlink.app.data.remote.dto.UpdateContactRequest
import kotlinx.coroutines.flow.Flow
import javax.inject.Inject
import javax.inject.Singleton

@Singleton
class ContactRepository @Inject constructor(
    private val api: GhostLinkApi,
    private val contactDao: ContactDao
) {
    fun getLocalContacts(): Flow<List<ContactEntity>> = contactDao.getContacts()

    suspend fun syncContactsFromServer(): Result<Unit> {
        return try {
            val contacts = api.getContacts()
            val entities = contacts.map { dto ->
                ContactEntity(
                    id = dto.id,
                    contactUsername = dto.contact_username,
                    status = dto.status,
                    createdAt = dto.created_at
                )
            }
            contactDao.insertContacts(entities)
            Result.success(Unit)
        } catch (e: Exception) {
            Result.failure(e)
        }
    }

    suspend fun addContact(username: String): Result<Unit> {
        return try {
            val dto = api.addContact(AddContactRequest(username))
            val entity = ContactEntity(
                id = dto.id,
                contactUsername = dto.contact_username,
                status = dto.status,
                createdAt = dto.created_at
            )
            contactDao.insertContact(entity)
            Result.success(Unit)
        } catch (e: Exception) {
            Result.failure(e)
        }
    }

    suspend fun acceptContactRequest(contactId: String): Result<Unit> {
        return try {
            val dto = api.updateContactStatus(contactId, UpdateContactRequest(status = 2)) // 2: Accept
            val entity = ContactEntity(
                id = dto.id,
                contactUsername = dto.contact_username,
                status = dto.status,
                createdAt = dto.created_at
            )
            contactDao.insertContact(entity)
            Result.success(Unit)
        } catch (e: Exception) {
            Result.failure(e)
        }
    }

    suspend fun blockContact(contactId: String): Result<Unit> {
        return try {
            val dto = api.updateContactStatus(contactId, UpdateContactRequest(status = 3)) // 3: Block
            val entity = ContactEntity(
                id = dto.id,
                contactUsername = dto.contact_username,
                status = dto.status,
                createdAt = dto.created_at
            )
            contactDao.insertContact(entity)
            Result.success(Unit)
        } catch (e: Exception) {
            Result.failure(e)
        }
    }

    suspend fun deleteContact(contactId: String): Result<Unit> {
        return try {
            api.deleteContact(contactId)
            contactDao.deleteContact(contactId)
            Result.success(Unit)
        } catch (e: Exception) {
            Result.failure(e)
        }
    }
}
