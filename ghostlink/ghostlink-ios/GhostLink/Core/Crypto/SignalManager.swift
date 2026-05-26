import Foundation
import LibSignalClient

struct GeneratedKeys {
    let identityKey: String
    let signedPreKey: SignedPreKeyDTO
    let oneTimeKeys: [OneTimePreKeyDTO]
}

class SignalManager {
    static let shared = SignalManager()
    private init() {}

    func generateKeys() -> GeneratedKeys {
        let identityKey = "IK_" + UUID().uuidString
        let spk = SignedPreKeyDTO(
            key_id: 1,
            public_key: "SPK_" + UUID().uuidString,
            signature: "SIG_" + UUID().uuidString
        )
        let otpks = (1...50).map { i in
            OneTimePreKeyDTO(key_id: i, public_key: "OPK_\(i)_" + UUID().uuidString)
        }

        return GeneratedKeys(
            identityKey: identityKey,
            signedPreKey: spk,
            oneTimeKeys: otpks
        )
    }

    func encrypt(plaintext: String, recipient: String) -> String {
        return "SIGNAL_ENC:\(recipient):\(Data(plaintext.utf8).base64EncodedString())"
    }

    func decrypt(ciphertext: String, sender: String) -> String {
        let prefix = "SIGNAL_ENC:\(sender):"
        guard ciphertext.hasPrefix(prefix) else { return ciphertext }
        let b64 = String(ciphertext.dropFirst(prefix.count))
        guard let data = Data(base64Encoded: b64) else { return ciphertext }
        return String(data: data, encoding: .utf8) ?? ciphertext
    }
}
