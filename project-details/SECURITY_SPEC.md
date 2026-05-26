# Security Specification
## GhostLink — Zero-Knowledge Architecture
**Version:** 1.0.0  
**Classification:** Internal Engineering

---

## 1. Threat Model

### 1.1 Adversaries

| Adversary | Capability | Our Defense |
|-----------|-----------|-------------|
| GhostLink employees | Full database access | E2EE — we cannot read messages |
| Law enforcement (legal order) | Subpoena for user data | No identity data exists; we cannot provide what we don't have |
| Nation-state (server compromise) | Full server access | E2EE + no logs + key material never on server |
| Network eavesdropper | Traffic analysis | TLS 1.3; minimal metadata in transit |
| Malicious user | CSRF, injection, DoS | Input validation, CSRF protection, rate limiting |
| Malicious app (compromised device) | Screen/keyboard capture | Device-level (out of scope); app hardening where possible |

### 1.2 What We Protect

**High sensitivity (must protect at all costs):**
- Message content
- User's real identity (no real identity stored)
- Contact relationships
- User's IP address (not logged)

**Medium sensitivity (protect with reasonable effort):**
- Usernames (pseudonymous, user-chosen)
- Timestamps of messages (stored with TTL)
- Group membership

**Out of scope:**
- Device-level attacks (rooted/jailbroken devices)
- Compromise of the user's own device
- Rubber-hose cryptanalysis (coercion of user)

---

## 2. End-to-End Encryption

### 2.1 Signal Protocol Implementation

GhostLink implements the Signal Protocol as specified at:
https://signal.org/docs/

**Components used:**
- **X3DH** (Extended Triple Diffie-Hellman) — Initial key agreement
- **Double Ratchet** — Ongoing message encryption
- **Curve25519** — Elliptic curve DH
- **AES-256-CBC** — Symmetric message encryption
- **HMAC-SHA256** — Message authentication
- **SHA-512** — Key derivation

**Rust Library:** `libsignal-client` (Signal's official Rust library)

### 2.2 Key Types and Lifecycle

| Key Type | Lives On | Rotated | Notes |
|----------|---------|---------|-------|
| Identity Key (IK) | Device (never leaves) | Never (account lifetime) | Long-term key pair |
| Signed Pre Key (SPK) | Server (public only) | Weekly | Signed by IK |
| One-Time Pre Key (OPK) | Server (public only) | Consumed once | Batch uploaded |
| Ephemeral Key (EK) | Memory only | Per session | Never stored |
| Session Keys | Device secure storage | Per Double Ratchet step | |

### 2.3 Group Encryption Details

Group messages use Signal's **Sender Key** protocol:

1. Each member generates a `SenderKey` document (chain key + signature key)
2. Member distributes their SenderKey to all other members via individual 1:1 E2EE channels
3. When sending a group message:
   - Encrypt with personal chain key (AES-256-CBC + HMAC-SHA256)
   - Single encryption per message regardless of group size
4. Recipients decrypt with cached SenderKey from sender

**Key Ratcheting in Groups:**
- SenderKey advances after each message (forward secrecy)
- When a member is removed, remaining members generate new SenderKeys and redistribute

---

## 3. Authentication Security

### 3.1 Password Hashing

**Algorithm:** Argon2id  
**Parameters (production):**
- Memory: 65,536 KB (64 MB)
- Iterations: 3
- Parallelism: 4
- Output length: 32 bytes
- Salt: 16 bytes random (per-password)

**Rationale:** Argon2id is the current OWASP recommendation. The parameters make brute-force attacks computationally expensive even with GPU clusters.

### 3.2 JWT Security

```
Algorithm: HS256 (HMAC-SHA256)
Secret: 256-bit random (from /dev/urandom, stored in Vault)
Expiry: 30 days
Claims: { sub: account_id, exp, iat }  -- NO username, NO email in JWT
```

**JWT Validation Checklist (on every request):**
- [ ] Valid signature
- [ ] Not expired (exp check)
- [ ] Subject (sub) is a valid UUID format
- [ ] Account still exists in DB

### 3.3 Brute Force Prevention

```
Login endpoint:
  - 10 attempts per IP per minute → 429 + 5-min cooldown
  - 50 attempts per IP per hour → 429 + 1-hour block
  - Progressive backoff: 1s, 2s, 4s, 8s per attempt

Register endpoint:
  - 3 attempts per IP per hour
  - CAPTCHA (hCaptcha, privacy-respecting) after 2nd attempt
```

---

## 4. Data Minimization

### 4.1 What We Store

| Data | Storage | Rationale |
|------|---------|-----------|
| Username | Plaintext | Required for lookup |
| Password | Argon2id hash only | Authentication |
| Public keys (Signal) | Plaintext | Required for E2EE setup |
| Encrypted messages | Ciphertext only | 30-day TTL, then deleted |
| Push token | Plaintext | Required for notifications |
| Last seen timestamp | Timestamp only (no IP) | Optional feature |
| Group membership | Account IDs only | Required for routing |

### 4.2 What We DO NOT Store

| Data | Notes |
|------|-------|
| IP addresses | Never logged |
| Device fingerprint | Never collected |
| User agent | Not stored |
| Message content (plaintext) | Technically impossible — E2EE |
| Location | Never requested |
| Email / phone | Not collected |
| Real name | Not collected |
| Browsing behavior | Not tracked |
| Analytics events | Minimal (crash reports only, no user ID) |

### 4.3 Log Sanitization Policy

Server logs contain:
```
[INFO] request_id=abc123 method=POST path=/auth/login status=200 latency_ms=45
```

Server logs **never** contain:
- Username
- Account ID
- IP address
- Request body
- Any user-supplied data

Implementation in Rust:
```rust
// Middleware strips all PII before logging
async fn log_middleware(req: Request, next: Next) -> Response {
    let request_id = Uuid::new_v4().to_string();
    let method = req.method().clone();
    let path = req.uri().path().to_owned();
    
    let start = Instant::now();
    let response = next.run(req).await;
    let latency = start.elapsed().as_millis();
    
    // ONLY log these fields — never IP, never body, never identity
    tracing::info!(
        request_id = %request_id,
        method = %method,
        path = %path,
        status = %response.status().as_u16(),
        latency_ms = %latency,
    );
    
    response
}
```

---

## 5. Transport Security

### 5.1 TLS Configuration

```nginx
# Nginx TLS config
ssl_protocols TLSv1.3;
ssl_ciphers TLS_AES_256_GCM_SHA384:TLS_CHACHA20_POLY1305_SHA256;
ssl_prefer_server_ciphers off;
ssl_session_timeout 1d;
ssl_session_cache shared:SSL:50m;
ssl_session_tickets off;
ssl_stapling on;
ssl_stapling_verify on;

# HSTS
add_header Strict-Transport-Security "max-age=63072000; includeSubDomains; preload" always;
```

### 5.2 Certificate Pinning (Mobile)

Both mobile apps implement certificate pinning:

**Android:**
```xml
<!-- res/xml/network_security_config.xml -->
<network-security-config>
    <domain-config>
        <domain includeSubdomains="true">api.ghostlink.app</domain>
        <pin-set expiration="2026-01-01">
            <pin digest="SHA-256">CERTIFICATE_HASH_BASE64</pin>
            <pin digest="SHA-256">BACKUP_CERTIFICATE_HASH_BASE64</pin>
        </pin-set>
    </domain-config>
</network-security-config>
```

**iOS (URLSession):**
```swift
class PinningDelegate: NSObject, URLSessionDelegate {
    func urlSession(_ session: URLSession,
                    didReceive challenge: URLAuthenticationChallenge,
                    completionHandler: @escaping (URLSession.AuthChallengeDisposition, URLCredential?) -> Void) {
        guard challenge.protectionSpace.authenticationMethod == NSURLAuthenticationMethodServerTrust,
              let serverTrust = challenge.protectionSpace.serverTrust else {
            completionHandler(.cancelAuthenticationChallenge, nil)
            return
        }
        // Validate against pinned certificate hash
        if validateCertificate(serverTrust) {
            completionHandler(.useCredential, URLCredential(trust: serverTrust))
        } else {
            completionHandler(.cancelAuthenticationChallenge, nil)
        }
    }
}
```

---

## 6. Mobile App Security

### 6.1 Android

| Mechanism | Implementation |
|-----------|---------------|
| Key storage | Android Keystore System (hardware-backed) |
| SQLite encryption | SQLCipher |
| Screenshot prevention | `FLAG_SECURE` on all Activities |
| Root detection | RootBeer library |
| Debuggable flag | `false` in release build |
| Exported components | All set to `false` |
| Backup prevention | `allowBackup="false"` in Manifest |
| ProGuard/R8 | Enabled with aggressive obfuscation |

### 6.2 iOS

| Mechanism | Implementation |
|-----------|---------------|
| Key storage | Secure Enclave via Keychain |
| SQLite encryption | SQLCipher |
| Screenshot prevention | Overlay on app resign active |
| Jailbreak detection | `IOSSecuritySuite` library |
| ATS (App Transport Security) | NSAllowsArbitraryLoads = false |
| App backgrounding | Blur overlay on background |
| Keychain access group | `com.ghostlink.app` only |

### 6.3 Secure Storage Key Hierarchy

```
Device Hardware (Secure Enclave / Keystore)
    └── Master Key (never leaves hardware)
          ├── Database Encryption Key (encrypted with Master Key)
          │     └── SQLCipher-encrypted SQLite database
          ├── Signal Protocol Keys (encrypted with Master Key)
          │     ├── Identity Key
          │     ├── Session Keys  
          │     └── Sender Keys
          └── App Lock PIN Hash (encrypted with Master Key)
```

---

## 7. Push Notification Privacy

Push notifications must NOT leak message content.

**FCM (Android) payload:**
```json
{
  "to": "device_push_token",
  "data": {
    "type": "NEW_MESSAGE"
  }
}
```

**APNs (iOS) payload:**
```json
{
  "aps": {
    "alert": {
      "title": "GhostLink",
      "body": "New message"
    },
    "content-available": 1,
    "sound": "default"
  }
}
```

**What is NOT in the payload:**
- Sender username
- Message preview
- Conversation ID (to prevent traffic analysis)
- Group name

The app wakes up on push, connects to WebSocket, and fetches actual messages via encrypted channel.

---

## 8. Abuse Prevention (Anonymous Context)

### 8.1 The Challenge

Traditional abuse prevention relies on identity (phone number, email). GhostLink has no identity. Alternative approaches:

### 8.2 CSAM Detection

Client-side scanning using **PhotoDNA hashes** (perceptual hashing):
- Images are hashed locally before encryption
- Hash is compared against NCMEC hash database locally
- If match found: upload is blocked client-side; account flagged server-side
- Server never sees image content
- False positive rate: ~0.000001%

```rust
// Server-side: store hash of media, not content
struct MediaRecord {
    media_id: Uuid,
    account_id: Uuid,
    phash: Option<[u8; 16]>,  // Perceptual hash for CSAM check
    encrypted_blob: Vec<u8>,  // Server cannot decrypt this
}
```

### 8.3 Spam Prevention

- Rate limiting on message sending (60/min per WebSocket connection)
- Contact request system: User must accept before receiving messages
- Block feature: Permanent block requires no account to re-block
- Proof-of-work on account creation (client-side puzzle, prevents mass account creation)

### 8.4 Account Banning

Without identity, bans are device-level:
- Device fingerprint hash (non-reversible) stored on ban
- New accounts from same device are blocked
- Users can circumvent by using a different device (accepted trade-off for anonymity)

---

## 9. Incident Response

### 9.1 Security Contacts

- **Security email:** security@ghostlink.app (PGP encrypted submissions welcomed)
- **Bug bounty:** Yes — responsible disclosure rewarded
- **Response SLA:** Critical vulnerabilities acknowledged within 24 hours

### 9.2 Breach Protocol

In the event of a server compromise:

1. **Immediate:** Take affected services offline
2. **Assessment:** Determine what was exposed (cannot be message content due to E2EE)
3. **Notification:** Notify users via in-app message within 72 hours
4. **Remediation:** Rotate all server-side secrets; force re-key of Signal sessions
5. **Post-mortem:** Public incident report within 30 days

**What a breach cannot expose:**
- Message content (E2EE — mathematically impossible without device compromise)
- Passwords (Argon2id hash — computationally infeasible to reverse)
- Real identity (we don't store it)

**What a breach could expose:**
- Usernames (pseudonymous only)
- Timestamps of messages
- Group membership lists

---

## 10. Penetration Testing

**Schedule:** Quarterly external pentest  
**Scope:** API, mobile apps, infrastructure  
**Testing methodology:** OWASP Mobile Top 10, OWASP API Top 10  
**Report availability:** Executive summary published publicly after remediation  

---

*End of Security Specification v1.0*
