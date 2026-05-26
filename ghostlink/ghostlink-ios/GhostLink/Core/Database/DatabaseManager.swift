import Foundation
import GRDB

class DatabaseManager {
    static let shared = DatabaseManager()
    var dbQueue: DatabaseQueue?

    private init() {
        setupDatabase()
    }

    func setupDatabase() {
        do {
            let fileManager = FileManager.default
            let documentsURL = try fileManager.url(for: .documentDirectory, in: .userDomainMask, appropriateFor: nil, create: true)
            let databaseURL = documentsURL.appendingPathComponent("ghostlink.sqlite")

            var config = Configuration()
            let passphrase = KeychainManager.shared.getDatabaseKey()

            config.prepareDatabase = { db in
                try db.usePassphrase(passphrase)
            }

            dbQueue = try DatabaseQueue(path: databaseURL.path, configuration: config)
            try migrate()
        } catch {
            print("Database initialization failed: \(error)")
        }
    }

    private func migrate() throws {
        guard let dbQueue = dbQueue else { return }

        var migrator = DatabaseMigrator()

        migrator.registerMigration("v1_schema") { db in
            try db.create(table: "accounts") { t in
                t.column("id", .text).primaryKey()
                t.column("username", .text).notNull()
                t.column("lastSeenAt", .integer).notNull()
            }

            try db.create(table: "contacts") { t in
                t.column("id", .text).primaryKey()
                t.column("contactUsername", .text).notNull()
                t.column("status", .integer).notNull()
                t.column("createdAt", .integer).notNull()
            }

            try db.create(table: "messages") { t in
                t.column("messageId", .text).primaryKey()
                t.column("conversationId", .text).notNull()
                t.column("senderUsername", .text).notNull()
                t.column("payloadCiphertext", .text).notNull()
                t.column("status", .integer).notNull()
                t.column("createdAt", .integer).notNull()
                t.column("isDisappeared", .boolean).notNull()
                t.column("disappearTimerSeconds", .integer).notNull()
            }
        }

        try migrator.migrate(dbQueue)
    }

    func resetDatabase() {
        dbQueue = nil
        if let path = dbQueue?.path {
            try? FileManager.default.removeItem(atPath: path)
        }
        setupDatabase()
    }
}
