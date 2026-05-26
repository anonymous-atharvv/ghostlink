import Foundation

enum APIError: Error {
    case invalidURL
    case noData
    case decodeFailed
    case serverError(String)
}

class APIClient {
    static let shared = APIClient()
    private let baseURL = "https://api.ghostlink.app/v1"
    private let decoder: JSONDecoder = {
        let d = JSONDecoder()
        return d
    }()

    private init() {}

    private func makeRequest(path: String, method: String = "GET", body: Encodable? = nil) -> URLRequest? {
        guard let url = URL(string: "\(baseURL)\(path)") else { return nil }
        var req = URLRequest(url: url)
        req.httpMethod = method
        req.setValue("application/json", forHTTPHeaderField: "Content-Type")
        req.setValue("application/json", forHTTPHeaderField: "Accept")

        if let token = KeychainManager.shared.get(forKey: "com.ghostlink.auth.jwt") {
            req.setValue("Bearer \(token)", forHTTPHeaderField: "Authorization")
        }

        if let body = body {
            req.httpBody = try? JSONEncoder().encode(body)
        }

        return req
    }

    func request<T: Decodable>(_ path: String, method: String = "GET", body: Encodable? = nil) async throws -> T {
        guard let req = makeRequest(path: path, method: method, body: body) else {
            throw APIError.invalidURL
        }

        let (data, response) = try await URLSession.shared.data(for: req)
        guard let httpResponse = response as? HTTPURLResponse else {
            throw APIError.serverError("Invalid response")
        }

        guard (200...299).contains(httpResponse.statusCode) else {
            throw APIError.serverError("HTTP \(httpResponse.statusCode)")
        }

        guard let decoded = try? decoder.decode(T.self, from: data) else {
            throw APIError.decodeFailed
        }
        return decoded
    }
}

// MARK: - DTOs

struct RegisterRequest: Codable {
    let username: String
    let password: String
    let identity_key: String
    let signed_pre_key: SignedPreKeyDTO
    let one_time_pre_keys: [OneTimePreKeyDTO]
}

struct SignedPreKeyDTO: Codable {
    let key_id: Int
    let public_key: String
    let signature: String
}

struct OneTimePreKeyDTO: Codable {
    let key_id: Int
    let public_key: String
}

struct LoginRequest: Codable {
    let username: String
    let password: String
}

struct AuthResponse: Codable {
    let token: String
    let account_id: String
    let username: String
}
