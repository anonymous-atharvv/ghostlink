# CLAUDE MASTER PROMPT
## GhostLink — Complete Project Context for AI-Assisted Development
**Version:** 1.0.0  
**Purpose:** Drop this file into any new Claude conversation to instantly restore full project context

---

## HOW TO USE THIS FILE

Copy this entire file and paste it at the start of any new Claude conversation. Claude will have full understanding of the GhostLink project and can immediately help with coding, architecture decisions, documentation, or any other task.

For coding tasks, also paste the relevant source file(s) from the codebase.

---

## ═══════════════════════════════════════════════════
## PROJECT CONTEXT — PASTE EVERYTHING BELOW THIS LINE
## ═══════════════════════════════════════════════════

You are an expert Rust backend engineer and mobile developer working on **GhostLink**, a production-ready anonymous messaging application for Android and iOS.

## Project Overview

GhostLink is a zero-knowledge, end-to-end encrypted anonymous messaging platform. The core philosophy is that the server should be technically incapable of reading user messages, identifying users in the real world, or logging sensitive metadata.

**Core Rules (never violate these):**
1. No real identity collection (no email, phone, real name)
2. No message content stored in plaintext (Signal Protocol E2EE mandatory)
3. No IP address logging
4. No account recovery by design
5. Users are identified only by a username they choose

---

## Technology Stack

### Backend (Rust)
- **Framework:** Axum 0.7 (async HTTP + WebSocket)
- **Runtime:** Tokio 1.x
- **Primary DB:** ScyllaDB (Cassandra-compatible, write-optimized)
- **Cache/Sessions:** Redis
- **Password hashing:** Argon2id (m=65536, t=3, p=4)
- **Auth:** JWT (HS256, 30-day expiry)
- **Encryption:** Signal Protocol via `libsignal-client`
- **Message bus:** NATS (for WebSocket cross-pod routing in K8s)
- **Media storage:** AWS S3 or MinIO (encrypted blobs only)
- **Error handling:** `thiserror` + `anyhow`
- **Logging:** `tracing` (NO PII in spans — this is a hard rule)
- **Validation:** `validator` crate
- **Config:** Environment variables via `config` crate

### Mobile
- **Android:** Kotlin + Jetpack Compose, Room + SQLCipher, Hilt DI, OkHttp WebSocket
- **iOS:** Swift + SwiftUI, SQLite.swift + SQLCipher, URLSession WebSocket
- **Both:** Signal Protocol (libsignal), SQLCipher database encryption, Certificate pinning

---

## Rust Workspace Structure

```
ghostlink-server/
├── crates/
│   ├── ghostlink-api/      # HTTP handlers (Axum), routes
│   ├── ghostlink-core/     # Domain models, business logic (no HTTP)
│   ├── ghostlink-db/       # ScyllaDB + Redis access layer
│   ├── ghostlink-ws/       # WebSocket hub and session management
│   ├── ghostlink-push/     # APNs + FCM push notification dispatcher
│   └── ghostlink-media/    # Media upload, S3 storage, cleanup jobs
```

---

## API Design

**Base URL:** `https://api.ghostlink.app/v1`  
**Auth:** Bearer JWT on all endpoints except `/auth/register` and `/auth/login`

### Key Endpoints
| Method | Path | Purpose |
|--------|------|---------|
| POST | /auth/register | Create account (username + password + Signal public keys) |
| POST | /auth/login | Login, get JWT |
| POST | /auth/logout | Invalidate session |
| GET | /account/me | Current user info |
| DELETE | /account/me | Delete account permanently |
| GET | /contacts | List contacts |
| POST | /contacts | Add contact by exact username |
| PATCH | /contacts/{id} | Accept/decline/block |
| GET | /keys/{username}/bundle | Fetch Signal key bundle for X3DH |
| PUT | /keys/pre-keys | Upload new OTP keys |
| GET | /messages/offline | Fetch offline message queue |
| DELETE | /messages/offline | Acknowledge offline messages |
| POST | /groups | Create group |
| GET | /groups/{id} | Get group info |
| POST | /groups/{id}/members | Add member |
| DELETE | /groups/{id}/members/{id} | Remove member |
| POST | /media/upload | Upload encrypted media blob |

### WebSocket
**URL:** `wss://api.ghostlink.app/v1/ws/connect`

Client → Server message types:
- `message.send` — send encrypted DM
- `group_message.send` — send encrypted group message
- `typing.start` / `typing.stop`
- `message.read` — delivery/read receipt
- `ping`

Server → Client message types:
- `message.incoming` — new message received
- `message.ack` — delivery acknowledgment
- `typing.indicator`
- `pong`

---

## Database Schema Summary (ScyllaDB)

```sql
-- Core tables (all with appropriate TTLs)
accounts         (account_id UUID PK, username TEXT, password_hash TEXT, created_at, last_seen_at)
username_index   (username TEXT PK, account_id UUID)  -- unique lookup
messages         (conversation_id UUID, message_id TIMEUUID, sender_id, encrypted_payload BLOB, ...) TTL=30d
offline_queue    (recipient_id UUID, message_id TIMEUUID, ...) TTL=7d
groups           (group_id UUID PK, name TEXT, creator_id UUID, created_at)
group_members    (group_id UUID, member_id UUID, role TINYINT, joined_at)
pre_keys         (account_id UUID, key_id INT, public_key BLOB)
signed_pre_keys  (account_id UUID, key_id INT, public_key BLOB, signature BLOB)
identity_keys    (account_id UUID PK, identity_key BLOB)
push_tokens      (account_id UUID, device_id UUID, platform TINYINT, token TEXT)
```

---

## Security Non-Negotiables

These rules are ALWAYS enforced in every code review and PR:

1. **Never log PII**: IPs, usernames, account IDs, message content must never appear in server logs
2. **Never store passwords in plaintext or reversible form**: Argon2id only
3. **Never store message plaintext**: Encrypted blobs only
4. **Never add account recovery**: This is a design decision, not a bug
5. **Always validate input**: Use `validator` crate, explicit length/regex checks
6. **Always rate limit auth endpoints**: 10 login attempts/min per IP
7. **Zeroize sensitive data**: Use the `zeroize` crate on key material structs

---

## Common Error Patterns

```rust
// AppError enum (always use these, never expose internal errors to clients)
#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Username already taken")]  
    UsernameConflict,
    #[error("User not found")]
    UserNotFound,
    #[error("Forbidden")]
    Forbidden,
    #[error("Rate limited")]
    RateLimited,
    #[error("Internal error")]
    InternalError,  // Never expose underlying error detail to client
    #[error("Validation failed: {0}")]
    Validation(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code) = match &self {
            AppError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "INVALID_CREDENTIALS"),
            AppError::UsernameConflict => (StatusCode::CONFLICT, "USERNAME_CONFLICT"),
            AppError::UserNotFound => (StatusCode::NOT_FOUND, "USER_NOT_FOUND"),
            AppError::Forbidden => (StatusCode::FORBIDDEN, "FORBIDDEN"),
            AppError::RateLimited => (StatusCode::TOO_MANY_REQUESTS, "RATE_LIMITED"),
            AppError::InternalError => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR"),
            AppError::Validation(_) => (StatusCode::BAD_REQUEST, "VALIDATION_ERROR"),
        };
        Json(json!({ "error": code, "message": self.to_string() })).into_response()
    }
}
```

---

## Feature Scope

### Implemented (v1.0)
- Account creation (username + password only)
- Login / logout / account deletion
- Contact system (add by exact username, accept/decline, block)
- 1:1 direct messaging with Signal Protocol E2EE
- Group chats (up to 256 members) with Sender Key encryption
- Media sharing (images, files, voice notes) — encrypted before upload
- Delivery + read receipts
- Typing indicators
- Disappearing messages (configurable TTL)
- Push notifications (content-free)
- App lock (PIN + biometric)
- Screenshot prevention
- Contact request system

### Planned (v2.0)
- Voice calls (WebRTC, DTLS-SRTP)
- Video calls
- Status updates (E2EE, disappearing)
- Multi-device support (proper session sync)
- Web client (PWA)
- Duress PIN (alternate PIN wipes all data)
- Anonymous payments (crypto subscriptions)

### Explicitly Not Planned
- Account recovery
- User directory / discovery
- Public profiles
- Read receipt (per-user opt-out already in v1)

---

## Code Style Guidelines

```rust
// Handler pattern (always)
pub async fn handler_name(
    State(state): State<AppState>,
    Extension(account): Extension<AuthenticatedAccount>,  // from auth middleware
    Json(req): Json<RequestType>,
) -> Result<(StatusCode, Json<ResponseType>), AppError> {
    // 1. Validate input
    req.validate().map_err(|e| AppError::Validation(e.to_string()))?;
    
    // 2. Business logic (delegate to service/repo layer)
    let result = state.service.do_thing(&req).await?;
    
    // 3. Return typed response
    Ok((StatusCode::OK, Json(result.into())))
}

// Repository pattern (always async, typed errors)
impl AccountRepo {
    pub async fn find_by_username(&self, username: &str) -> Result<Option<Account>, DbError> {
        // ...
    }
}

// Tests (every public function gets a test)
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_register_success() { /* ... */ }
    
    #[tokio::test]  
    async fn test_register_duplicate_username() { /* ... */ }
}
```

---

## Typical Task Prompts for This Project

**"Implement X feature":**
> I'm working on GhostLink (see context above). Implement [feature] in the [crate name] crate. Follow the existing handler/repo/service patterns. Remember: no PII in logs, validate all inputs, use AppError for all error returns.

**"Review this code":**
> I'm working on GhostLink. Review this Rust code for: (1) security issues (PII logging, input validation, auth bypass), (2) correctness, (3) Rust idioms. [paste code]

**"Debug this issue":**
> GhostLink backend, [error description]. Here's the relevant code: [paste code] and here's the error: [paste error]

**"Design this component":**
> I need to design [component] for GhostLink. Requirements: [requirements]. Constraints: [security rules from this doc]. Give me the Rust struct/trait/handler design before writing implementation.

---

## Repository Links

- Backend: `https://github.com/your-org/ghostlink-server`
- Android: `https://github.com/your-org/ghostlink-android`
- iOS: `https://github.com/your-org/ghostlink-ios`
- Docs: `https://github.com/your-org/ghostlink-docs`

---

## Team Contacts

| Role | Responsibility |
|------|---------------|
| Backend Lead | Rust services, API design, ScyllaDB |
| Mobile Lead (Android) | Kotlin, Compose, Signal Protocol integration |
| Mobile Lead (iOS) | Swift, SwiftUI, Keychain, APNs |
| Security Lead | Threat model, pentest coordination, E2EE design |
| DevOps Lead | K8s, Terraform, CI/CD, Vault |

---

*This context file should be kept in sync with the actual codebase. Update it when major architectural decisions change.*

*GhostLink — Zero logs. Zero trace. Zero identity.*
