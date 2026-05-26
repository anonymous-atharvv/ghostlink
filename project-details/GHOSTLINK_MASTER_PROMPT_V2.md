# GHOSTLINK — MASTER ENGINEERING PROMPT v2.0
## Top-Tier Production Context for AI-Assisted Development
**Classification:** Principal Engineer / CTO Reference  
**Audience:** AI coding agents, senior engineers, system architects  
**Philosophy:** Y Combinator quality bar — build for 10 million users from day one, ship for 100 today.

---

## ═══ HOW TO USE THIS PROMPT ═══

Drop this file verbatim into any new Claude / AI agent conversation. It replaces all other context files. Every code decision, PR, architecture discussion, and design review should be grounded in the principles and constraints defined below.

For implementation tasks, also paste the specific source file(s) you are working in.

**Hierarchy of authority:**
```
FIVE UNBREAKABLE RULES (Section 2) > Security constraints (Section 7)
  > Scalability patterns (Section 5) > Code style (Section 10)
    > Everything else
```

If a decision conflicts with a higher rule, the higher rule wins. Always.

---

## ═══ SECTION 1 — WHO WE ARE ═══

### Product Identity

GhostLink is the **only mobile messaging platform where the server is architecturally incapable of knowing who you are, what you say, or where you are.**

We are not "a more private WhatsApp." We are a different category: a **zero-knowledge communications layer** — a blind relay wrapped in Signal Protocol encryption, deployed on a Rust backend that mathematically cannot betray its users even if subpoenaed, hacked, or acquired.

**Tagline:** *Zero logs. Zero trace. Zero identity.*

### Market Thesis (YC-framing)

The TAM is every human on earth who communicates. The beachhead is 50M+ people globally who have already self-identified their need: activists, journalists, legal professionals, enterprise security teams, privacy-native Gen Z, and the 1B+ users in repressive-regime markets where Signal is blocked or its phone-number requirement creates a surveillance vector.

**The insight that incumbents cannot copy:** Signal requires a phone number. WhatsApp requires a phone number and sells data. Telegram is not E2EE by default and runs centralized servers. We require nothing but a username. That single architectural constraint — no phone, no email, no name — creates a moat that incumbents cannot cross without destroying their own ad revenue or GDPR compliance model.

### Stage & Velocity

- **Backend (Rust):** Architecture verified, 6-crate workspace scaffolded, Phase 1 & 2 core complete, Phase 3 Signal Protocol integration in progress.
- **Android (Kotlin/Compose):** Foundations complete (auth, DB, WS), Phase 3 UI in progress.
- **iOS (Swift/SwiftUI):** Foundations complete (auth, DB, WS), Phase 3 UI in progress.
- **Production target:** 18-week runway to app store submission.

---

## ═══ SECTION 2 — FIVE UNBREAKABLE RULES ═══

These are non-negotiable. Any code, PR, design, or suggestion that violates these rules is automatically wrong, regardless of other merit.

```
RULE 1 — NO REAL IDENTITY
  Never collect, log, store, infer, or transmit: email address, phone number,
  real name, date of birth, government ID, or any field that links a GhostLink
  account to a real-world person. This includes metadata. This includes logs.

RULE 2 — NO PLAINTEXT MESSAGES
  The Signal Protocol (X3DH + Double Ratchet) is MANDATORY for all DMs.
  Sender Key protocol is MANDATORY for all group messages. The server MUST
  receive, store, and route only opaque ciphertext blobs. Any code that
  touches message content in cleartext on the server is a critical bug.

RULE 3 — NO IP LOGGING
  Server logs contain exactly: request_id (UUID), HTTP method, URL path
  (pattern only, not actual IDs), HTTP status code, latency in milliseconds.
  IP addresses are suppressed at the Nginx layer before reaching application
  code. Any tracing span, log line, or metric that includes an IP, username,
  account_id, or any user-supplied value is a critical security bug.

RULE 4 — NO ACCOUNT RECOVERY
  There is no password reset. There is no email verification. There is no
  support ticket path to account access. Losing credentials = losing the
  account. This is not a limitation. It is the privacy guarantee. Never
  build, suggest, or imply a recovery mechanism.

RULE 5 — ZEROIZE ALL SECRETS
  Every struct holding key material (passwords, session keys, identity keys,
  ephemeral keys, AES keys) MUST implement the `zeroize::Zeroize` trait and
  be dropped via `zeroize_on_drop`. Passwords are hashed with Argon2id
  (m=65536, t=3, p=4). No exceptions. No "we'll add it later."
```

---

## ═══ SECTION 3 — TECHNOLOGY STACK ═══

### Backend (Rust — Non-Negotiable Choices)

| Layer | Technology | Why |
|-------|-----------|-----|
| HTTP + WS framework | Axum 0.7 | Tower-native, composable, zero-cost middleware |
| Async runtime | Tokio 1.x | Industry standard; matches Axum's model |
| Primary database | ScyllaDB 5.4 (CQL) | Write-optimized, horizontal scale, native TTL, no ORM overhead |
| Session / presence cache | Redis 7 (deadpool-redis) | Sub-millisecond presence checks, rate limit counters |
| WebSocket cross-pod routing | NATS 2.10 (async-nats) | Sub-millisecond pub/sub, eliminates inter-pod HTTP calls |
| Media storage | AWS S3 / MinIO | Trait-abstracted; swap at infra layer without code change |
| Password hashing | Argon2id (argon2 crate) | OWASP gold standard; GPU-resistant |
| Auth tokens | JWT HS256 (jsonwebtoken) | 30-day expiry; no refresh token by design |
| E2EE | libsignal-client (official Signal library) | Battle-tested; do not roll custom crypto |
| Error handling | thiserror + anyhow | Typed domain errors; never expose internals to clients |
| Input validation | validator crate | Declarative; attach to all request DTOs |
| Logging | tracing + tracing-opentelemetry | Structured; PII-scrubbed at middleware layer |
| Rate limiting | tower-governor | Per-IP and per-account; plugged in as tower middleware |
| Secret management | HashiCorp Vault (prod) / dotenv (dev) | Never hardcode; never commit secrets |
| Memory safety for keys | zeroize crate | Overwrite key bytes on drop |

### Android (Kotlin)

| Layer | Technology |
|-------|-----------|
| UI | Jetpack Compose + Material 3 |
| Architecture | MVVM + Clean Architecture + Hilt DI |
| Local DB | Room + SQLCipher (AES-256, key from Android Keystore) |
| Key storage | Android Keystore System (hardware-backed, StrongBox preferred) |
| Network | Retrofit (REST) + OkHttp (WebSocket) |
| E2EE | libsignal-android |
| Security hardening | FLAG_SECURE, allowBackup=false, RootBeer, ProGuard/R8 |
| Min SDK | 26 (Android 8.0) |

### iOS (Swift)

| Layer | Technology |
|-------|-----------|
| UI | SwiftUI + UIKit AppDelegate |
| Architecture | MVVM + Repository pattern |
| Local DB | GRDB.swift + SQLCipher (key from Keychain / Secure Enclave) |
| Key storage | Keychain with `kSecAttrAccessibleAfterFirstUnlockThisDeviceOnly` |
| Network | URLSession + URLSessionWebSocketTask |
| E2EE | libsignal-ios |
| Security hardening | Background blur, IOSSecuritySuite, ATS enforced |
| Min iOS | 16.0 |

---

## ═══ SECTION 4 — WORKSPACE STRUCTURE ═══

```
ghostlink/
├── ghostlink-server/                 # Rust Cargo workspace
│   ├── Cargo.toml                    # Workspace manifest (shared deps here)
│   ├── .env.example
│   ├── docker-compose.yml
│   ├── Dockerfile                    # Multi-stage: rust:stable → debian:bookworm-slim
│   ├── migrations/                   # CQL migration files (001_*, 002_*, ...)
│   ├── tests/
│   │   ├── integration/              # Full API integration tests (spin up testcontainers)
│   │   └── load/locustfile.py        # 50K concurrent WS load model
│   └── crates/
│       ├── ghostlink-api/            # Axum router, handlers, middleware, DTOs
│       ├── ghostlink-core/           # Domain entities, business rules — ZERO http deps
│       ├── ghostlink-db/             # ScyllaDB repos + Redis cache (no business logic)
│       ├── ghostlink-ws/             # WebSocket hub, NATS bridge, session FSM
│       ├── ghostlink-push/           # APNs + FCM dispatcher (content-free payloads only)
│       └── ghostlink-media/          # S3 upload, TTL cleanup, encrypted blob serving
│
├── ghostlink-android/
│   ├── app/src/main/java/com/ghostlink/app/
│   │   ├── GhostLinkApp.kt
│   │   ├── di/                       # Hilt modules
│   │   ├── data/local/               # Room entities, DAOs, Keystore wrapper
│   │   ├── data/remote/              # Retrofit API, AuthInterceptor, WS client
│   │   ├── data/repository/          # Bridges remote + local
│   │   ├── domain/                   # Models, use cases (pure Kotlin, no Android deps)
│   │   ├── crypto/                   # SignalManager, KeyBundleManager, MediaEncryptor
│   │   └── ui/                       # Compose screens, components, theme, NavGraph
│   └── gradle/libs.versions.toml
│
└── ghostlink-ios/
    ├── GhostLink/
    │   ├── App/                      # GhostLinkApp.swift, AppDelegate
    │   ├── Core/                     # APIClient, WSClient, DatabaseManager, Keychain
    │   ├── Domain/                   # Models, repositories, use cases
    │   ├── Features/                 # SwiftUI feature modules (Onboarding, Chat, Groups...)
    │   └── Crypto/                   # SignalManager, MediaEncryptor
    └── Package.swift
```

**Crate dependency graph (enforce this — no cycles):**
```
ghostlink-api  ──depends on──►  ghostlink-core
ghostlink-api  ──depends on──►  ghostlink-db
ghostlink-api  ──depends on──►  ghostlink-ws
ghostlink-db   ──depends on──►  ghostlink-core
ghostlink-ws   ──depends on──►  ghostlink-core
ghostlink-ws   ──depends on──►  ghostlink-db
ghostlink-push ──depends on──►  ghostlink-core
ghostlink-media──depends on──►  ghostlink-core
```

`ghostlink-core` has **zero** dependencies on Axum, ScyllaDB, Redis, or NATS. It is a pure domain layer. Tests here run instantly without any infrastructure.

---

## ═══ SECTION 5 — SCALABILITY ARCHITECTURE ═══

### Design for 10 million users from day one. Ship for 100 today.

This is the YC principle: your architecture must not require a rewrite to scale, but it must not be over-engineered to ship. Every decision below satisfies both.

### 5.1 Horizontal Scaling Strategy

**The WebSocket problem:** WS connections are stateful. Alice on Pod 1 cannot receive a message delivered by Pod 3. Solution: NATS pub/sub.

```
Alice sends message to Bob
         │
         ▼
Pod 1 (Alice's WS connection)
  ├── Bob online on THIS pod? → deliver directly via DashMap
  └── Bob not here?
        └── NATS publish: subject="user.{bob_account_id}"
                   │
                   ▼
            NATS Server (all pods subscribed to "user.*")
                   │
                   ▼
            Pod 3 (Bob's WS connection lives here)
              └── Found Bob in local DashMap → deliver
```

Each pod subscribes to `user.*` on startup. No cross-pod HTTP. No distributed lock. NATS handles fan-out to all of Bob's devices (multi-device support) transparently.

**DashMap — never a global RwLock:**
```rust
// Correct: segmented concurrent hashmap — no global lock contention
use dashmap::DashMap;
pub struct ConnectionHub {
    connections: Arc<DashMap<Uuid, Vec<Sender>>>,
}
// WRONG: Arc<RwLock<HashMap<...>>> — single writer blocks ALL readers at scale
```

### 5.2 Database Partitioning Strategy

**ScyllaDB partition key decisions are permanent.** Get them right now.

| Table | Partition Key | Clustering Key | Rationale |
|-------|--------------|----------------|-----------|
| `messages` | `conversation_id` | `message_id TIMEUUID DESC` | All messages for a chat on one shard; DESC for efficient "latest N" |
| `group_messages` | `(group_id, bucket)` where bucket = `epoch_day` | `message_id TIMEUUID DESC` | Prevents hot partition on popular groups; 1 partition per group per day |
| `offline_queue` | `recipient_id` | `message_id TIMEUUID ASC` | FIFO delivery per recipient; scoped to one shard |
| `pre_keys` | `account_id` | `key_id INT` | All keys for a user co-located; consumed atomically via LWT |
| `contacts` | `account_id` | `contact_id UUID` | Contact list per user on one shard |

**Never do a full table scan.** Every read is a primary key lookup or a partition scan with a LIMIT clause. No secondary indexes on hot paths.

**Lightweight Transactions (LWT) for username uniqueness:**
```sql
INSERT INTO username_index (username, account_id)
VALUES (?, ?)
IF NOT EXISTS;
-- Paxos-backed: guarantees uniqueness across all nodes simultaneously
```

### 5.3 Redis Slot Strategy

```
Redis Cluster (3 masters + 3 replicas in production)

Key prefixes and their slot distribution:
  session:{token_hash}        → Auth sessions; TTL 30d
  presence:{account_id}       → Online/offline; TTL 65s (ping refreshes every 60s)
  ratelimit:auth:{ip_hash}    → Auth rate limit; TTL 60s
  ratelimit:api:{account_id}  → API rate limit; TTL 60s
  otp_count:{account_id}      → Pre-key count cache; TTL 300s
```

**Never store IP addresses in Redis keys.** Use a SHA-256 hash of the IP for rate limiting. The hash is one-way — it rate-limits without logging.

### 5.4 Caching Strategy

```
Request arrives
  │
  ├── Redis HIT? ──YES──► Return cached value (< 1ms)
  │
  └── Redis MISS?
        │
        ├── ScyllaDB query (< 5ms on SSD)
        │
        └── Cache result in Redis with appropriate TTL
              Session data   → TTL 30d
              Presence       → TTL 65s
              Username check → TTL 60s
              Pre-key count  → TTL 300s
```

**Never cache message content or key material in Redis.** Only cache lookups and counters.

### 5.5 Performance Targets (SLA)

| Metric | Target | How |
|--------|--------|-----|
| Auth (login/register) | P99 < 800ms | Argon2id is intentionally slow; pre-warm thread pool |
| WS message delivery (online recipient) | P50 < 50ms, P99 < 200ms | DashMap lookup + WS write; no DB on hot path |
| WS message delivery (offline recipient) | P99 < 500ms | ScyllaDB write + NATS push event |
| REST API (non-auth) | P99 < 100ms | Redis cache hit for most reads |
| Media upload (10MB) | < 5s on 4G | Streaming multipart; no buffering in memory |
| WebSocket reconnect | < 3s | Exponential backoff with jitter; offline queue fetch on reconnect |
| Concurrent WS connections per pod | 50,000+ | Tokio async; DashMap; no blocking |
| Message throughput per pod | 1M messages/day | Target at standard 3-pod deployment |

### 5.6 Auto-Scaling Triggers

```yaml
# Kubernetes HPA — scale on WebSocket connection pressure
metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 60   # Scale before saturation, not after
  - type: Pods
    pods:
      metric:
        name: ghostlink_ws_connections_active
      target:
        type: AverageValue
        averageValue: 30000      # Scale out at 30K connections/pod; cap at 50K
minReplicas: 3
maxReplicas: 50
```

---

## ═══ SECTION 6 — API CONTRACTS ═══

**Base URL:** `https://api.ghostlink.app/v1`  
**Auth:** `Authorization: Bearer <jwt_token>` on all endpoints except `/auth/*`  
**JWT:** HS256, 30-day expiry, claims: `{ sub: account_id (UUID), exp, iat }` — NO username in JWT.

### REST Endpoints (18 total)

| Method | Path | Auth | Purpose |
|--------|------|------|---------|
| POST | /auth/register | No | Create account + upload Signal key bundle |
| POST | /auth/login | No | Authenticate, receive JWT |
| POST | /auth/logout | Yes | Invalidate session in Redis |
| GET | /account/me | Yes | Current account info |
| DELETE | /account/me | Yes | Permanent account deletion (password confirm) |
| GET | /contacts | Yes | List all contacts with status |
| POST | /contacts | Yes | Send contact request by exact username |
| PATCH | /contacts/{account_id} | Yes | Accept / decline / block |
| DELETE | /contacts/{account_id} | Yes | Remove contact |
| GET | /keys/{username}/bundle | Yes | Fetch Signal X3DH key bundle (consumes 1 OTP) |
| PUT | /keys/pre-keys | Yes | Upload new one-time pre-keys |
| GET | /keys/pre-keys/count | Yes | Check remaining OTP count |
| GET | /messages/offline | Yes | Fetch queued offline messages |
| DELETE | /messages/offline | Yes | Acknowledge + clear offline queue |
| POST | /messages/send | Yes | REST fallback when WS unavailable |
| POST | /groups | Yes | Create group |
| GET | /groups/{group_id} | Yes | Group metadata |
| GET | /groups/{group_id}/members | Yes | Member list |
| POST | /groups/{group_id}/members | Yes | Add member (admin) |
| DELETE | /groups/{group_id}/members/{account_id} | Yes | Remove / leave |
| PATCH | /groups/{group_id}/members/{account_id} | Yes | Promote / demote |
| DELETE | /groups/{group_id} | Yes | Delete group (owner) |
| POST | /groups/{group_id}/invite-link | Yes | Generate expiring invite link |
| POST | /media/upload | Yes | Upload encrypted media blob |
| GET | /media/{media_id} | Yes | Download encrypted media blob |

### WebSocket Protocol

**URL:** `wss://api.ghostlink.app/v1/ws/connect`

**Client → Server types:**

| Type | Purpose |
|------|---------|
| `message.send` | Encrypted DM: `{ recipient_id, conversation_id, encrypted_payload, payload_type }` |
| `group_message.send` | Encrypted group msg: `{ group_id, encrypted_payload, payload_type }` |
| `typing.start` / `typing.stop` | Typing indicator: `{ conversation_id }` |
| `message.read` | Read receipt: `{ conversation_id, last_read_message_id }` |
| `ping` | Keepalive (server responds with `pong`) |

**Server → Client types:**

| Type | Purpose |
|------|---------|
| `message.incoming` | Delivered message with full payload |
| `message.ack` | Delivery confirmation with `request_id` correlation |
| `typing.indicator` | `{ conversation_id, account_id, is_typing }` |
| `pong` | Keepalive response |
| `error` | WS-level error with `request_id` + `code` |

### Rate Limits

| Endpoint | Limit | Window |
|----------|-------|--------|
| POST /auth/register | 3 | Per IP per hour |
| POST /auth/login | 10 | Per IP per minute |
| POST /contacts | 30 | Per account per hour |
| POST /media/upload | 50 | Per account per hour |
| WS messages | 60 | Per connection per minute |
| All other REST | 300 | Per account per minute |

**Rate limit headers on every response:**
```
X-RateLimit-Limit: 300
X-RateLimit-Remaining: 297
X-RateLimit-Reset: 1704067200
```

---

## ═══ SECTION 7 — SECURITY CONSTRAINTS ═══

### 7.1 Threat Model Summary

| Adversary | What they can do | Our defense |
|-----------|-----------------|-------------|
| GhostLink employees (full DB access) | Read database rows | E2EE — ciphertext only; we cannot decrypt |
| Law enforcement (subpoena) | Request user data | No identity stored; cannot provide what we don't have |
| Nation-state (full server compromise) | Read server memory + DB | E2EE + no IP logs + private keys never leave devices |
| Network eavesdropper | Traffic analysis | TLS 1.3 minimum; HSTS; certificate pinning on mobile |
| Malicious app (compromised device) | Read decrypted messages | Device-level (out of scope); app hardening where possible |

### 7.2 Signal Protocol Implementation

```
Key Types:
  Identity Key (IK)       → Generated on device, NEVER leaves device
  Signed Pre Key (SPK)    → Public key on server; rotated weekly; signed by IK
  One-Time Pre Key (OPK)  → Public keys on server; consumed once per session
  Ephemeral Key (EK)      → Per-session; lives in memory only; never persisted

X3DH Key Agreement (Alice → Bob first message):
  DH1 = DH(IK_A, SPK_B)
  DH2 = DH(EK_A, IK_B)
  DH3 = DH(EK_A, SPK_B)
  DH4 = DH(EK_A, OPK_B)    [if OPK available]
  master_secret = KDF(DH1 ‖ DH2 ‖ DH3 ‖ DH4)

Double Ratchet (ongoing):
  Every message advances the ratchet → new encryption key per message
  Compromise of one message key does not compromise past or future messages
  Forward secrecy + break-in recovery

Group Messaging (Sender Key):
  Each member generates a SenderKey document
  Member distributes SenderKey to all others via 1:1 E2EE channels
  Group message encrypted once → all recipients decrypt with cached SenderKey
  Member removal → ALL remaining members generate + redistribute new SenderKeys
```

### 7.3 Mandatory Security Checks on Every PR

```
☐ Zero PII in any log line, trace span, or metric label
☐ All new endpoints have auth middleware applied
☐ All new endpoints have rate limiting applied
☐ All request DTOs use #[derive(Validate)] with explicit constraints
☐ All new key material structs use #[derive(Zeroize, ZeroizeOnDrop)]
☐ Error responses never expose internal error details (AppError::InternalError only)
☐ No hardcoded secrets (CI checks for secret patterns)
☐ cargo audit passes with zero vulnerabilities
☐ cargo clippy -- -D warnings passes
☐ Every public function has at least one unit test
☐ Happy path + failure path tested for all handlers
```

### 7.4 Mobile Security Invariants

**Android:**
- `FLAG_SECURE` set in `MainActivity.onCreate()` — non-negotiable; cannot be removed
- `android:allowBackup="false"` in manifest — non-negotiable
- Database passphrase derived and stored in Android Keystore (hardware-backed, StrongBox if available)
- JWT stored in EncryptedSharedPreferences backed by Keystore — NEVER plain SharedPreferences
- Certificate pinning against `api.ghostlink.app` (SHA-256 hash + backup pin)
- RootBeer root check on startup — show warning, do not block (user autonomy)

**iOS:**
- Keychain items: `kSecAttrAccessibleAfterFirstUnlockThisDeviceOnly` — not synced to iCloud
- Background blur applied in `scenePhase == .inactive` — non-negotiable
- ATS: `NSAllowsArbitraryLoads = false` — no exceptions
- Certificate pinning via URLSessionDelegate
- IOSSecuritySuite jailbreak check on startup — warning only

---

## ═══ SECTION 8 — DATABASE SCHEMA ═══

```sql
-- ScyllaDB keyspace
CREATE KEYSPACE ghostlink
  WITH replication = {
    'class': 'NetworkTopologyStrategy',
    'datacenter1': 3     -- RF=3; survives 1 node failure with no downtime
  }
  AND durable_writes = true;

-- Core accounts (no email, no phone, no real name — ever)
CREATE TABLE accounts (
    account_id UUID PRIMARY KEY,
    username   TEXT,
    password_hash TEXT,     -- Argon2id only
    created_at TIMESTAMP,
    last_seen_at TIMESTAMP  -- Timestamp only; no IP
);

-- Unique username enforcement via LWT INSERT IF NOT EXISTS
CREATE TABLE username_index (
    username   TEXT PRIMARY KEY,
    account_id UUID
);

-- 1:1 and group messages (time-series, 30-day TTL)
CREATE TABLE messages (
    conversation_id UUID,
    message_id      TIMEUUID,
    sender_id       UUID,
    encrypted_payload BLOB,
    payload_type    TINYINT,   -- 0=text 1=image 2=file 3=voice
    status          TINYINT,   -- 0=sent 1=delivered 2=read
    PRIMARY KEY (conversation_id, message_id)
) WITH CLUSTERING ORDER BY (message_id DESC)
  AND default_time_to_live = 2592000;  -- 30 days

-- Offline delivery queue (7-day TTL)
CREATE TABLE offline_queue (
    recipient_id    UUID,
    message_id      TIMEUUID,
    conversation_id UUID,
    sender_id       UUID,
    encrypted_payload BLOB,
    PRIMARY KEY (recipient_id, message_id)
) WITH CLUSTERING ORDER BY (message_id ASC)
  AND default_time_to_live = 604800;   -- 7 days

-- Groups
CREATE TABLE groups (
    group_id   UUID PRIMARY KEY,
    name       TEXT,
    creator_id UUID,
    created_at TIMESTAMP
);

CREATE TABLE group_members (
    group_id  UUID,
    member_id UUID,
    role      TINYINT,   -- 0=member 1=admin 2=owner
    joined_at TIMESTAMP,
    PRIMARY KEY (group_id, member_id)
);

-- Signal Protocol key material (public keys only)
CREATE TABLE identity_keys (
    account_id   UUID PRIMARY KEY,
    identity_key BLOB
);

CREATE TABLE signed_pre_keys (
    account_id UUID,
    key_id     INT,
    public_key BLOB,
    signature  BLOB,
    PRIMARY KEY (account_id, key_id)
);

CREATE TABLE pre_keys (
    account_id UUID,
    key_id     INT,
    public_key BLOB,
    PRIMARY KEY (account_id, key_id)
);

-- Push tokens (anonymous device tokens only — no content in payload ever)
CREATE TABLE push_tokens (
    account_id UUID,
    device_id  UUID,
    platform   TINYINT,  -- 0=ios 1=android
    token      TEXT,
    updated_at TIMESTAMP,
    PRIMARY KEY (account_id, device_id)
);

-- Contacts
CREATE TABLE contacts (
    account_id UUID,
    contact_id UUID,
    status     TINYINT,  -- 0=pending_sent 1=pending_received 2=accepted 3=blocked
    added_at   TIMESTAMP,
    PRIMARY KEY (account_id, contact_id)
);
```

**Redis schema:**
```
session:{token_hash}        TTL: 30d   Value: {account_id, device_id}
presence:{account_id}       TTL: 65s   Value: {device_id, pod_id}
ratelimit:auth:{ip_sha256}  TTL: 60s   Value: count (INCR)
ratelimit:api:{account_id}  TTL: 60s   Value: count (INCR)
otp_count:{account_id}      TTL: 300s  Value: count
```

---

## ═══ SECTION 9 — RUST CODE PATTERNS ═══

### 9.1 Handler Pattern (always follow this shape)

```rust
pub async fn handler_name(
    State(state): State<AppState>,
    Extension(account): Extension<AuthenticatedAccount>,   // injected by auth middleware
    Json(req): Json<RequestType>,
) -> Result<(StatusCode, Json<ResponseType>), AppError> {

    // 1. Validate (ALWAYS first — reject before any DB or business logic)
    req.validate().map_err(|e| AppError::Validation(e.to_string()))?;

    // 2. Business logic (delegate to service or repo layer — no SQL in handlers)
    let result = state.service.do_thing(account.id, &req).await?;

    // 3. Return typed response (never return raw DB types to clients)
    Ok((StatusCode::OK, Json(ResponseType::from(result))))
}
```

### 9.2 AppError (the only error type that reaches clients)

```rust
#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Username already taken")]
    UsernameConflict,
    #[error("User not found")]
    UserNotFound,
    #[error("Group not found")]
    GroupNotFound,
    #[error("Forbidden")]
    Forbidden,
    #[error("Rate limited")]
    RateLimited,
    #[error("Validation failed: {0}")]
    Validation(String),
    #[error("Internal error")]
    InternalError,   // ← swallow ALL internal detail here; log internally via tracing
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code) = match &self {
            AppError::InvalidCredentials  => (StatusCode::UNAUTHORIZED, "INVALID_CREDENTIALS"),
            AppError::UsernameConflict    => (StatusCode::CONFLICT, "USERNAME_CONFLICT"),
            AppError::UserNotFound        => (StatusCode::NOT_FOUND, "USER_NOT_FOUND"),
            AppError::GroupNotFound       => (StatusCode::NOT_FOUND, "GROUP_NOT_FOUND"),
            AppError::Forbidden           => (StatusCode::FORBIDDEN, "FORBIDDEN"),
            AppError::RateLimited         => (StatusCode::TOO_MANY_REQUESTS, "RATE_LIMITED"),
            AppError::Validation(_)       => (StatusCode::BAD_REQUEST, "VALIDATION_ERROR"),
            AppError::InternalError       => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR"),
        };
        Json(json!({ "error": code, "message": self.to_string() })).into_response()
    }
}

// DB/cache errors convert to AppError — never expose ScyllaDB errors to clients
impl From<DbError> for AppError {
    fn from(e: DbError) -> Self {
        tracing::error!(error = %e, "database error");  // log internally
        AppError::InternalError                         // return opaque error to client
    }
}
```

### 9.3 Request DTO Pattern

```rust
#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 32), regex(path = "*USERNAME_REGEX"))]
    pub username: String,

    #[validate(length(min = 8, max = 1024))]
    pub password: String,   // validated here but NEVER logged anywhere

    #[validate(length(min = 1))]
    pub identity_key: String,

    pub signed_pre_key: SignedPreKey,

    #[validate(length(min = 1, max = 100))]
    pub one_time_pre_keys: Vec<OneTimePreKey>,
}

// Ensure password is zeroized when the request struct is dropped
impl Drop for RegisterRequest {
    fn drop(&mut self) {
        self.password.zeroize();
    }
}
```

### 9.4 Repository Pattern

```rust
// In ghostlink-db — pure data access, zero business logic
impl AccountRepo {
    pub async fn find_by_username(
        &self,
        username: &str,
    ) -> Result<Option<Account>, DbError> {
        let rows = self.session
            .query("SELECT account_id, username, password_hash, created_at
                    FROM username_index JOIN accounts USING (account_id)
                    WHERE username = ?", (username,))
            .await?;
        // map rows → Option<Account>
    }

    pub async fn create(&self, account: &Account) -> Result<(), DbError> {
        // Use LWT to enforce uniqueness:
        let result = self.session
            .query("INSERT INTO username_index (username, account_id)
                    VALUES (?, ?) IF NOT EXISTS",
                   (&account.username, account.id))
            .await?;
        if !result.rows_num()? > 0 {
            return Err(DbError::UsernameConflict);
        }
        // then insert into accounts table
    }
}
```

### 9.5 Logging Rules

```rust
// CORRECT — structured, no PII
tracing::info!(
    request_id = %request_id,
    method     = %method,
    path       = %sanitized_path,   // pattern only: /contacts/{id} not /contacts/550e8400-...
    status     = %status_code,
    latency_ms = %latency,
);

// WRONG — PII in log
tracing::info!(username = %req.username, "login attempt");      // username is PII
tracing::info!(account_id = %account.id, "registered");         // account_id is PII
tracing::error!(ip = %peer_addr, "rate limited");               // IP is PII
tracing::debug!(payload = %req.encrypted_payload, "message");   // NEVER log payloads
```

### 9.6 Test Structure (every public function gets tests)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // Happy path
    #[tokio::test]
    async fn test_register_success() {
        let state = test_app_state().await;
        let req = valid_register_request();
        let response = register(State(state), Json(req)).await;
        assert!(response.is_ok());
        let (status, Json(body)) = response.unwrap();
        assert_eq!(status, StatusCode::CREATED);
        assert!(!body.token.is_empty());
    }

    // All failure modes
    #[tokio::test]
    async fn test_register_duplicate_username_returns_409() { ... }

    #[tokio::test]
    async fn test_register_short_username_returns_400() { ... }

    #[tokio::test]
    async fn test_register_short_password_returns_400() { ... }

    #[tokio::test]
    async fn test_login_wrong_password_returns_401() { ... }

    #[tokio::test]
    async fn test_login_nonexistent_user_returns_401() { ... }
    // Both wrong-user and wrong-password must return IDENTICAL responses
    // to prevent username enumeration attacks
}
```

---

## ═══ SECTION 10 — CODE QUALITY STANDARDS ═══

### Non-Negotiable Quality Gates (CI enforces all of these)

```bash
cargo test --all             # All tests pass
cargo clippy --all -- -D warnings  # Zero warnings (treated as errors)
cargo fmt --all -- --check   # Consistent formatting
cargo audit                  # Zero known vulnerabilities in dependencies
```

### Code Review Checklist

When reviewing any PR, check in this order:

1. **Security (any violation = instant reject)**
   - PII in logs?
   - Unvalidated user input?
   - Missing auth middleware on new endpoint?
   - Missing rate limit on new endpoint?
   - Key material not zeroized?
   - Internal error details exposed to client?

2. **Correctness**
   - Does the happy path work?
   - Are all error cases handled?
   - Are concurrent access patterns safe (no data races)?

3. **Scalability**
   - Does this add a full-table scan? (Reject if yes)
   - Does this create a hot partition? (Justify if yes)
   - Does this break the NATS fan-out pattern? (Reject if yes)

4. **Rust idioms**
   - Using `?` operator properly?
   - Avoiding unnecessary clones?
   - Using `Arc<>` instead of `Mutex<>` where possible?

5. **Tests**
   - Does every new public function have a test?
   - Are failure modes tested (not just happy path)?

---

## ═══ SECTION 11 — FEATURE SCOPE ═══

### v1.0 — In Scope (build these)

- Account creation (username + password; no real identity)
- Login / logout / permanent account deletion
- Contact system (add by exact username, accept/decline/block, contact requests)
- 1:1 Direct Messaging with full Signal Protocol E2EE
- Group Chats (up to 256 members, Sender Key encryption)
- Media sharing: images (10MB), files (50MB), voice notes (5min) — all encrypted before upload
- Delivery receipts (✓ sent, ✓✓ delivered)
- Read receipts (✓✓ blue, toggleable per-user)
- Typing indicators
- Disappearing messages (off / 1h / 24h / 7d / 30d)
- Push notifications — content-free (device wake only)
- App lock (PIN + biometric re-auth)
- Screenshot prevention (FLAG_SECURE / blur overlay)
- Group invite links (expiring)
- Reply to message (quote)
- Delete for me / delete for everyone (within 1 hour)
- Mute conversation
- @mention in groups
- Group admin controls (promote, demote, remove, delete group)

### v2.0 — Planned (do not build these yet)

- Voice calls (WebRTC, DTLS-SRTP)
- Video calls
- Status updates (E2EE, disappearing)
- Multi-device sync (proper session fan-out)
- Web client (PWA)
- Duress PIN (alternate PIN wipes all local data)
- Anonymous crypto payments (Monero subscriptions)

### Explicitly Never (do not suggest, do not build)

- Account recovery of any kind
- Email / phone collection
- User directory or search
- Public profiles
- Read receipt toggle forced on (always user-controlled)
- Ad integration
- Analytics that identify individual users

---

## ═══ SECTION 12 — OBSERVABILITY (PRIVACY-PRESERVING) ═══

### Prometheus Metrics We Collect

```
ghostlink_http_requests_total{method, path_pattern, status}
ghostlink_http_request_duration_seconds{method, path_pattern, quantile}
ghostlink_ws_connections_active
ghostlink_ws_messages_sent_total{type}
ghostlink_messages_queued_total
ghostlink_offline_queue_depth
ghostlink_media_uploads_total{media_type}
ghostlink_push_notifications_sent_total{platform, status}
ghostlink_db_query_duration_seconds{operation}
ghostlink_cache_hit_ratio{cache}
ghostlink_pre_key_count_low_alerts_total   // trigger key replenishment alerts
```

### Prometheus Metrics We NEVER Collect

```
Per-user message counts      → reveals usage patterns per identity
Per-conversation metrics     → correlates accounts
Message size histograms      → reveals content types
Connection duration per user → behavioral fingerprinting
Geographic distribution      → location inference
```

### OpenTelemetry Span Attributes

```
ALLOWED:   http.method, http.route (pattern), http.status_code,
           db.system, db.operation, rpc.service, error.type

FORBIDDEN: user.id, account.id, username, http.url (has IDs in path),
           request body content, message content, IP address
```

### Alerts

```yaml
# Alert if OTP pre-key pool is depleted (clients can't establish new sessions)
- alert: PreKeyPoolExhausted
  expr: ghostlink_pre_key_count_low_alerts_total > 0
  severity: warning

# Alert on error rate spike (possible attack or regression)
- alert: ErrorRateHigh
  expr: rate(ghostlink_http_requests_total{status=~"5.."}[5m]) > 0.01
  severity: critical

# Alert on WS connection count approaching pod capacity
- alert: WsConnectionsHigh
  expr: ghostlink_ws_connections_active > 45000
  severity: warning
```

---

## ═══ SECTION 13 — DEPLOYMENT & INFRASTRUCTURE ═══

### Kubernetes Topology (production minimum)

```
ghostlink-api     → 3 pods, autoscale to 50 on CPU > 60%
ghostlink-ws      → 5 pods (most resource-intensive), autoscale on WS connection count
ghostlink-media   → 3 pods
ghostlink-push    → 2 pods
ScyllaDB          → 3-node cluster, RF=3, NVMe SSDs required
Redis             → 3-master + 3-replica cluster
NATS              → 3-node cluster
```

### Docker Build (multi-stage — final image is minimal)

```dockerfile
FROM rust:1.78-slim AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN useradd -r -s /bin/false ghostlink
COPY --from=builder /app/target/release/ghostlink-server /usr/local/bin/
USER ghostlink
EXPOSE 8080
CMD ["ghostlink-server"]
```

### Secret Management (never in code, never in git)

```
Production:  HashiCorp Vault (Vault Agent sidecar injects as env vars)
Staging:     Kubernetes Secrets (KMS-encrypted at rest)
Development: .env file (gitignored; template in .env.example)

Secrets that exist:
  JWT_SECRET              — 256-bit random; rotate quarterly
  ARGON2_MEMORY_KB etc.   — config, not secret but not committed
  APNS_PRIVATE_KEY        — file reference; never inline
  FCM_SERVER_KEY          — Vault only
  SCYLLA_PASSWORD         — Vault only; rotated weekly
  AWS_SECRET_ACCESS_KEY   — Vault only; use IAM roles in production
```

---

## ═══ SECTION 14 — AGENT TASK EXECUTION PROTOCOL ═══

When any AI agent receives a task for GhostLink, execute in this order:

```
Step 1: Read this file (or confirm it's in context)
Step 2: Identify which crate / module / platform the task touches
Step 3: Check: does this task touch any of the 5 Unbreakable Rules?
         If yes → design the implementation so the rule is satisfied first
Step 4: Write the implementation
Step 5: Write or update tests
Step 6: Verify: would cargo clippy -D warnings pass?
Step 7: Verify: are there any PII in log lines?
Step 8: Deliver with clear commit message format:
         feat(crate-name): description [scope]
         fix(crate-name): description [scope]
         sec(crate-name): description — security fix
```

### Standard Task Prompts

**Implement a feature:**
> GhostLink backend: implement [feature] in the [crate] crate. Follow the handler/repo/service patterns in Section 9. Validate all inputs (Section 9.3). No PII in logs (Rule 3). Use AppError for all error returns (Section 9.2). Write tests for all cases (Section 9.6).

**Security review:**
> Review this GhostLink code for: (1) PII in logs, (2) missing input validation, (3) missing auth, (4) unzeroized key material, (5) internal error details exposed to clients, (6) race conditions. Return a prioritized finding list.

**Architecture decision:**
> GhostLink design decision: [describe problem]. Constraints: no PII collection (Rule 1), no plaintext messages (Rule 2), must scale to 10M users (Section 5). Evaluate [Option A] vs [Option B] against these constraints and recommend with justification.

**Mobile task:**
> GhostLink [Android/iOS]: implement [feature]. All Signal key material must be stored in [Keystore/Keychain] (Section 7.4). No plaintext persisted outside SQLCipher. No JWT in insecure storage. Write unit + UI tests.

---

## ═══ SECTION 15 — YC-LEVEL ENGINEERING PRINCIPLES ═══

These are the principles that separate a top-1% team from everyone else. Internalize them.

### 1. Make the server architecturally incapable of evil

The strongest privacy guarantee is not a policy — it is an architectural impossibility. Every design decision should push toward "we cannot do X even if compelled." This is the moat. This is why GhostLink wins.

### 2. Do things that don't scale, to learn what must scale

The NATS + DashMap WS architecture scales to 10M users. That does not mean we deploy 50 pods on day one. Deploy 3. Measure. Scale only what is actually hot. Premature scaling is waste; late scaling is death. Know which is which by measuring.

### 3. Simplicity is a security property

Every line of code is an attack surface. Every abstraction is a complexity tax. Prefer the simpler solution when security properties are equivalent. ScyllaDB LWT for username uniqueness beats a distributed lock manager — fewer moving parts.

### 4. The latency budget is a contract, not a suggestion

P99 < 100ms for REST, P50 < 50ms for WS delivery. If a PR breaks this, it does not ship. Cache what needs caching. Batch what needs batching. Never block async tasks.

### 5. Test failure modes, not just success

Every endpoint has at least one test for the failure case. The system's behavior under adversarial conditions is more important than its behavior under ideal conditions. Write tests for: wrong credentials, duplicate usernames, rate limit exhaustion, malformed payloads, missing auth headers.

### 6. Privacy is a feature, market it as one

The no-phone-number architecture is not a technical limitation — it is the lead product differentiator. Every user-facing string, every onboarding screen, every App Store description should lead with this. "Create an account in 10 seconds. No phone. No email. No real name." That converts.

### 7. Audit everything. Log nothing about users.

The tracing infrastructure should tell you everything about system health (latency, error rate, throughput, queue depth) and nothing about who is using it. If you can reconstruct a user's behavior from logs, you have a privacy bug.

---

*GhostLink — Zero logs. Zero trace. Zero identity.*  
*Built by a team that ships. Designed for a world that watches.*

---

**Version:** 2.0.0 — Generated from full project analysis (all 14 reference documents)  
**Authority:** Supersedes MASTER_PROMPT.md v1.0.0 and CLAUDE_AGENT_CONTEXT.md  
**Update policy:** Update this file when architecture decisions change. NEVER let it drift from the codebase.
