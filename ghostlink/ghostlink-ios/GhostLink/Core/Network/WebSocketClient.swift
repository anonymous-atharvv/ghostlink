import Foundation
import GRDB

class WebSocketClient: NSObject, URLSessionWebSocketDelegate {
    static let shared = WebSocketClient()

    private var webSocketTask: URLSessionWebSocketTask?
    private var urlSession: URLSession!
    private var isConnected = false
    private var reconnectAttempt = 0
    private let maxDelay: TimeInterval = 30
    private let baseDelay: TimeInterval = 1

    private override init() {
        super.init()
        urlSession = URLSession(configuration: .default, delegate: self, delegateQueue: OperationQueue())
    }

    func connect() {
        guard let token = KeychainManager.shared.get(forKey: "com.ghostlink.auth.jwt") else { return }

        var request = URLRequest(url: URL(string: "wss://api.ghostlink.app/v1/ws/connect")!)
        request.setValue("Bearer \(token)", forHTTPHeaderField: "Authorization")

        webSocketTask = urlSession.webSocketTask(with: request)
        webSocketTask?.resume()

        isConnected = true
        reconnectAttempt = 0

        listen()
        sendPing()
    }

    private func listen() {
        webSocketTask?.receive { [weak self] result in
            switch result {
            case .success(let message):
                switch message {
                case .string(let text):
                    self?.handleIncomingMessage(text)
                default:
                    break
                }
                self?.listen()

            case .failure:
                self?.handleDisconnect()
            }
        }
    }

    private func sendPing() {
        guard isConnected else { return }

        webSocketTask?.sendPing { [weak self] error in
            if error != nil {
                self?.handleDisconnect()
            } else {
                DispatchQueue.global().asyncAfter(deadline: .now() + 30) {
                    self?.sendPing()
                }
            }
        }
    }

    func send(jsonPayload: String, completion: @escaping (Bool) -> Void) {
        let message = URLSessionWebSocketMessage.string(jsonPayload)
        webSocketTask?.send(message) { error in
            completion(error == nil)
        }
    }

    private func handleIncomingMessage(_ jsonText: String) {
        guard let data = jsonText.data(using: .utf8) else { return }

        do {
            let decoder = JSONDecoder()
            let wireMsg = try decoder.decode(WsWireMessage.self, from: data)

            switch wireMsg.type {
            case "message.incoming":
                let payloadData = Data(wireMsg.payload.utf8)
                let incoming = try decoder.decode(IncomingMessagePayload.self, from: payloadData)

                let plainText = SignalManager.shared.decrypt(
                    ciphertext: incoming.payload_ciphertext,
                    sender: incoming.sender_username
                )

                try DatabaseManager.shared.dbQueue?.write { db in
                    var message = MessageRow(
                        messageId: incoming.message_id,
                        conversationId: incoming.sender_username,
                        senderUsername: incoming.sender_username,
                        payloadCiphertext: plainText,
                        status: 1,
                        createdAt: incoming.created_at,
                        isDisappeared: false,
                        disappearTimerSeconds: 0
                    )
                    try message.insert(db)
                }

            case "message.ack":
                break

            default:
                break
            }
        } catch {
            print("Failed to decode incoming message: \(error)")
        }
    }

    private func handleDisconnect() {
        isConnected = false
        webSocketTask = nil

        let delay = min(baseDelay * pow(2.0, Double(reconnectAttempt)), maxDelay)
        reconnectAttempt += 1

        DispatchQueue.global().asyncAfter(deadline: .now() + delay) { [weak self] in
            self?.connect()
        }
    }

    func disconnect() {
        isConnected = false
        webSocketTask?.cancel(with: .normalClosure, reason: nil)
        webSocketTask = nil
    }
}

struct WsWireMessage: Codable {
    let type: String
    let payload: String
}

struct IncomingMessagePayload: Codable {
    let message_id: String
    let sender_username: String
    let payload_ciphertext: String
    let created_at: Int64
}

struct MessageRow: Codable, FetchableRecord, PersistableRecord {
    static let databaseTableName = "messages"

    let messageId: String
    let conversationId: String
    let senderUsername: String
    let payloadCiphertext: String
    let status: Int
    let createdAt: Int64
    let isDisappeared: Bool
    let disappearTimerSeconds: Int64
}
