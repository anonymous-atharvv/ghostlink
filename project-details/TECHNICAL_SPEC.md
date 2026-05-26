# Technical Specification
## GhostLink — Rust Backend + Mobile Architecture
**Version:** 1.0.0  
**Status:** Engineering Reference  

---

## 1. Architecture Overview

```
┌──────────────────────────────────────────────────────────────────────┐
│                          CLIENT LAYER                                │
│    Android (Kotlin/Jetpack Compose)   iOS (Swift/SwiftUI)           │
└──────────────────────────┬───────────────────────────────────────────┘
                           │ HTTPS + WSS (TLS 1.3)
┌──────────────────────────▼───────────────────────────────────────────┐
│                        EDGE / GATEWAY                                │
│              Nginx (TLS termination, rate limiting)                  │
│              Cloudflare (DDoS, IP anonymization)                     │
└──────────────────────────┬───────────────────────────────────────────┘
                           │
┌──────────────────────────▼───────────────────────────────────────────┐
│                       SERVICE LAYER (Rust)                           │
│  ┌─────────────┐  ┌──────────────┐  ┌────────────┐  ┌────────────┐  │
│  │  Auth Svc   │  │  Chat Svc    │  │  Media Svc │  │  Push Svc  │  │
│  │  (Axum)     │  │  (Axum+WS)   │  │  (Axum)    │  │  (Axum)    │  │
│  └─────────────┘  └──────────────┘  └────────────┘  └────────────┘  │
└──────────────────────────┬───────────────────────────────────────────┘
                           │
┌──────────────────────────▼───────────────────────────────────────────┐
│                        DATA LAYER                                    │
│   ScyllaDB (messages, accounts)    Redis (sessions, presence)        │
│   MinIO / S3 (encrypted media)     NATS (internal message bus)       │
└──────────────────────────────────────────────────────────────────────┘
```

---

## 2. Rust Backend Specification

### 2.1 Technology Choices

| Component | Library | Rationale |
|-----------|---------|-----------|
| HTTP/WS framework | `axum` 0.7+ | Ergonomic, tower-compatible, async-first |
| Async runtime | `tokio` 1.x | Industry standard async runtime |
| Database driver | `scylla` (scylladb/scylla-rust-driver) | Native ScyllaDB driver |
| Redis client | `deadpool-redis` | Async pool |
| Serialization | `serde` + `serde_json` | Standard |
| Auth / JWT | `jsonwebtoken` | JWT HS256 for API tokens |
| Password hashing | `argon2` | Argon2id — memory-hard, current best practice |
| Encryption (Signal) | `libsignal-protocol` (forked from signal-protocol-rust) | E2EE |
| UUID | `uuid` v4 | Account + message IDs |
| Tracing | `tracing` + `tracing-opentelemetry` | No PII in spans |
| Config | `config` crate | Environment-based config |
| Error handling | `thiserror` + `anyhow` | Typed errors |
| Validation | `validator` | Input validation |
| Rate limiting | `tower-governor` | Per-IP, per-account limits |
| Message bus | `async-nats` | Internal service communication |
| Media streaming | `tokio-util` | Streaming uploads |

---

### 2.2 Project Structure

```
ghostlink-server/
├── Cargo.toml                     # Workspace manifest
├── Cargo.lock
├── docker-compose.yml
├── .env.example
│
├── crates/
│   ├── ghostlink-api/             # HTTP/WebSocket handlers
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   ├── lib.rs
│   │   │   ├── config.rs
│   │   │   ├── router.rs
│   │   │   ├── middleware/
│   │   │   │   ├── auth.rs        # JWT extraction middleware
│   │   │   │   ├── rate_limit.rs
│   │   │   │   └── request_id.rs
│   │   │   ├── handlers/
│   │   │   │   ├── auth.rs        # POST /auth/register, /auth/login
│   │   │   │   ├── contacts.rs    # GET/POST /contacts
│   │   │   │   ├── messages.rs    # POST /messages (offline delivery)
│   │   │   │   ├── groups.rs      # CRUD /groups
│   │   │   │   ├── media.rs       # POST /media/upload
│   │   │   │   ├── websocket.rs   # WS /ws/connect
│   │   │   │   └── health.rs      # GET /health
│   │   │   ├── models/
│   │   │   │   ├── requests.rs    # Incoming DTO structs
│   │   │   │   └── responses.rs   # Outgoing DTO structs
│   │   │   └── error.rs           # AppError enum → HTTP status
│   │   └── Cargo.toml
│   │
│   ├── ghostlink-core/            # Domain logic (no HTTP)
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── account.rs         # Account entity + business rules
│   │   │   ├── message.rs         # Message entity
│   │   │   ├── group.rs           # Group entity
│   │   │   ├── contact.rs         # Contact entity
│   │   │   └── crypto.rs          # Key generation helpers
│   │   └── Cargo.toml
│   │
│   ├── ghostlink-db/              # Database access layer
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── connection.rs      # ScyllaDB pool setup
│   │   │   ├── migrations.rs
│   │   │   ├── repos/
│   │   │   │   ├── account_repo.rs
│   │   │   │   ├── message_repo.rs
│   │   │   │   ├── group_repo.rs
│   │   │   │   └── contact_repo.rs
│   │   │   └── cache/
│   │   │       ├── session_cache.rs  # Redis session store
│   │   │       └── presence_cache.rs # Online presence
│   │   └── Cargo.toml
│   │
│   ├── ghostlink-ws/              # WebSocket engine
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── hub.rs             # Connection registry (DashMap)
│   │   │   ├── session.rs         # Per-connection state machine
│   │   │   ├── router.rs          # WS message routing
│   │   │   └── protocol.rs        # Wire protocol definitions
│   │   └── Cargo.toml
│   │
│   ├── ghostlink-push/            # Push notification dispatcher
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── apns.rs            # Apple Push Notification Service
│   │   │   ├── fcm.rs             # Firebase Cloud Messaging
│   │   │   └── dispatcher.rs      # Route to correct provider
│   │   └── Cargo.toml
│   │
│   └── ghostlink-media/           # Media handling
│       ├── src/
│       │   ├── lib.rs
│       │   ├── upload.rs          # Chunked upload handler
│       │   ├── storage.rs         # S3/MinIO abstraction
│       │   └── cleanup.rs         # TTL-based purge job
│       └── Cargo.toml
│
├── migrations/                    # ScyllaDB CQL migration files
│   ├── 001_initial_schema.cql
│   ├── 002_add_groups.cql
│   └── 003_add_media.cql
│
└── tests/
    ├── integration/
    │   ├── auth_test.rs
    │   ├── message_test.rs
    │   └── group_test.rs
    └── load/
        └── locustfile.py          # Load testing
```

---

### 2.3 Core Rust: Auth Handler

```rust
// crates/ghostlink-api/src/handlers/auth.rs

use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{rand_core::OsRng, SaltString};
use jsonwebtoken::{encode, Header, EncodingKey};

use crate::{AppState, error::AppError};
use ghostlink_core::account::Account;

#[derive(Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 32), regex = "USERNAME_REGEX")]
    pub username: String,
    #[validate(length(min = 8))]
    pub password: String,
    /// Client-generated public key bundle for Signal Protocol key exchange
    pub identity_key: String,
    pub signed_pre_key: SignedPreKey,
    pub one_time_pre_keys: Vec<OneTimePreKey>,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub account_id: Uuid,
    pub username: String,
}

pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), AppError> {
    req.validate()?;

    // Check username availability (case-insensitive)
    let normalized = req.username.to_lowercase();
    if state.db.account_repo.username_exists(&normalized).await? {
        return Err(AppError::UsernameConflict);
    }

    // Hash password with Argon2id
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(req.password.as_bytes(), &salt)
        .map_err(|_| AppError::InternalError)?
        .to_string();

    // Generate account
    let account_id = Uuid::new_v4();
    let account = Account {
        id: account_id,
        username: normalized.clone(),
        password_hash,
        created_at: chrono::Utc::now(),
    };

    // Persist account + upload key bundle atomically
    state.db.account_repo.create(&account).await?;
    state.db.key_repo.store_key_bundle(
        account_id,
        &req.identity_key,
        &req.signed_pre_key,
        &req.one_time_pre_keys,
    ).await?;

    // Issue JWT (no sensitive data in claims)
    let token = issue_jwt(account_id, &state.config.jwt_secret)?;

    Ok((StatusCode::CREATED, Json(AuthResponse {
        token,
        account_id,
        username: normalized,
    })))
}

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let account = state.db.account_repo
        .find_by_username(&req.username.to_lowercase()).await?
        .ok_or(AppError::InvalidCredentials)?;

    let parsed_hash = PasswordHash::new(&account.password_hash)
        .map_err(|_| AppError::InternalError)?;

    Argon2::default()
        .verify_password(req.password.as_bytes(), &parsed_hash)
        .map_err(|_| AppError::InvalidCredentials)?;

    // Rotate session on login
    let token = issue_jwt(account.id, &state.config.jwt_secret)?;

    // Update last seen (timestamp only, no IP)
    state.db.account_repo.update_last_seen(account.id).await?;

    Ok(Json(AuthResponse {
        token,
        account_id: account.id,
        username: account.username,
    }))
}

fn issue_jwt(account_id: Uuid, secret: &str) -> Result<String, AppError> {
    #[derive(Serialize)]
    struct Claims {
        sub: String,
        exp: usize,
        iat: usize,
    }

    let now = chrono::Utc::now().timestamp() as usize;
    let claims = Claims {
        sub: account_id.to_string(),
        exp: now + 86400 * 30, // 30-day token
        iat: now,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    ).map_err(|_| AppError::InternalError)
}
```

---

### 2.4 WebSocket Engine

```rust
// crates/ghostlink-ws/src/hub.rs

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use uuid::Uuid;

/// Per-connection sender handle
pub type Sender = mpsc::UnboundedSender<WsMessage>;

/// Global connection registry
#[derive(Clone)]
pub struct ConnectionHub {
    /// account_id → vec of device senders (multi-device support)
    connections: Arc<RwLock<HashMap<Uuid, Vec<Sender>>>>,
}

impl ConnectionHub {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register(&self, account_id: Uuid, tx: Sender) {
        let mut map = self.connections.write().await;
        map.entry(account_id).or_default().push(tx);
    }

    pub async fn unregister(&self, account_id: Uuid, tx: &Sender) {
        let mut map = self.connections.write().await;
        if let Some(senders) = map.get_mut(&account_id) {
            senders.retain(|s| !s.same_channel(tx));
            if senders.is_empty() {
                map.remove(&account_id);
            }
        }
    }

    pub async fn send_to_account(&self, account_id: Uuid, msg: WsMessage) {
        let map = self.connections.read().await;
        if let Some(senders) = map.get(&account_id) {
            for sender in senders {
                let _ = sender.send(msg.clone());
            }
        }
    }

    pub async fn is_online(&self, account_id: Uuid) -> bool {
        self.connections.read().await.contains_key(&account_id)
    }
}
```

---

### 2.5 Environment Configuration

```toml
# .env.example

# Server
SERVER_HOST=0.0.0.0
SERVER_PORT=8080

# Database
SCYLLA_NODES=localhost:9042
SCYLLA_KEYSPACE=ghostlink
SCYLLA_USERNAME=
SCYLLA_PASSWORD=

# Redis
REDIS_URL=redis://localhost:6379

# Security
JWT_SECRET=change-this-to-a-256-bit-random-string
ARGON2_MEMORY_KB=65536
ARGON2_ITERATIONS=3
ARGON2_PARALLELISM=4

# Media Storage
STORAGE_BACKEND=s3   # s3 | minio | local
S3_BUCKET=ghostlink-media
S3_REGION=us-east-1
AWS_ACCESS_KEY_ID=
AWS_SECRET_ACCESS_KEY=

# Push Notifications
APNS_KEY_ID=
APNS_TEAM_ID=
APNS_PRIVATE_KEY_PATH=/secrets/apns.p8
FCM_SERVER_KEY=

# NATS
NATS_URL=nats://localhost:4222

# Observability
OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317
LOG_LEVEL=info

# Limits
MAX_MESSAGE_SIZE_BYTES=65536
MAX_MEDIA_SIZE_BYTES=52428800
MAX_GROUP_MEMBERS=256
RATE_LIMIT_AUTH_PER_MIN=10
RATE_LIMIT_API_PER_MIN=300
```

---

### 2.6 Cargo.toml (Root Workspace)

```toml
[workspace]
members = [
    "crates/ghostlink-api",
    "crates/ghostlink-core",
    "crates/ghostlink-db",
    "crates/ghostlink-ws",
    "crates/ghostlink-push",
    "crates/ghostlink-media",
]
resolver = "2"

[workspace.dependencies]
axum = { version = "0.7", features = ["ws", "multipart"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
uuid = { version = "1", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
thiserror = "1"
anyhow = "1"
argon2 = "0.5"
jsonwebtoken = "9"
validator = { version = "0.18", features = ["derive"] }
scylla = "0.13"
deadpool-redis = "0.15"
async-nats = "0.35"
tower = "0.4"
tower-http = { version = "0.5", features = ["cors", "trace", "request-id"] }
```

---

## 3. Signal Protocol E2EE Design

### 3.1 Key Exchange Flow

```
                    Server (blind relay)
                          │
Alice registers ──────────┤──────── Bob registers
  uploads:                │           uploads:
  - IdentityKey (IK_A)    │           - IdentityKey (IK_B)
  - SignedPreKey (SPK_A)  │           - SignedPreKey (SPK_B)  
  - OneTimePreKeys[N]     │           - OneTimePreKeys[N]

Alice wants to message Bob:
  1. Alice fetches Bob's key bundle from server
  2. Alice performs X3DH:
       DH1 = DH(IK_A, SPK_B)
       DH2 = DH(EK_A, IK_B)  [EK = ephemeral key]
       DH3 = DH(EK_A, SPK_B)
       DH4 = DH(EK_A, OPK_B) [if available]
       master_secret = KDF(DH1 || DH2 || DH3 || DH4)
  3. Alice encrypts first message with master_secret
  4. Server routes encrypted blob to Bob
  5. Bob performs same X3DH to derive master_secret
  6. Both have shared secret — Double Ratchet begins
```

### 3.2 Server's Role in E2EE

The server:
- Stores public key bundles (never private keys)
- Routes encrypted blobs between users
- CANNOT decrypt any message
- CANNOT read message content
- Only sees: sender_id, recipient_id, encrypted_payload, timestamp

### 3.3 Group Encryption (Sender Keys)

Groups use Signal's Sender Key protocol:
1. Each group member generates a unique Sender Key
2. Each member distributes their Sender Key to all other members via 1:1 E2EE channels
3. Group messages are encrypted once with the sender's Sender Key
4. Recipients decrypt with the cached Sender Key

---

## 4. Database Design — ScyllaDB

### 4.1 Rationale for ScyllaDB
- Write-optimized: Chat apps are write-heavy
- Horizontally scalable: No single point of failure
- Compatible with Cassandra CQL: Large ecosystem
- TTL support: Native per-record expiry for message cleanup
- No full scans required: All access patterns are key-based

### 4.2 Core Schema

```sql
-- Keyspace
CREATE KEYSPACE ghostlink 
WITH replication = {'class': 'NetworkTopologyStrategy', 'datacenter1': 3}
AND durable_writes = true;

-- Accounts table
CREATE TABLE accounts (
    account_id UUID PRIMARY KEY,
    username TEXT,
    password_hash TEXT,
    created_at TIMESTAMP,
    last_seen_at TIMESTAMP,
    -- NO email, NO phone, NO real name
) WITH default_time_to_live = 0;

-- Username → account_id lookup (unique index simulation)
CREATE TABLE username_index (
    username TEXT PRIMARY KEY,
    account_id UUID
);

-- Messages (time-series, partitioned by conversation)
CREATE TABLE messages (
    conversation_id UUID,
    message_id TIMEUUID,
    sender_id UUID,
    encrypted_payload BLOB,    -- Signal-encrypted ciphertext
    payload_type TINYINT,      -- 0=text, 1=image, 2=file, 3=voice
    status TINYINT,            -- 0=sent, 1=delivered, 2=read
    created_at TIMESTAMP,
    PRIMARY KEY (conversation_id, message_id)
) WITH CLUSTERING ORDER BY (message_id DESC)
  AND default_time_to_live = 2592000;  -- 30 days TTL

-- Offline message queue (for recipients who are offline)
CREATE TABLE offline_queue (
    recipient_id UUID,
    message_id TIMEUUID,
    conversation_id UUID,
    encrypted_payload BLOB,
    sender_id UUID,
    created_at TIMESTAMP,
    PRIMARY KEY (recipient_id, message_id)
) WITH CLUSTERING ORDER BY (message_id ASC)
  AND default_time_to_live = 604800;  -- 7 days TTL then purge

-- Groups
CREATE TABLE groups (
    group_id UUID PRIMARY KEY,
    name TEXT,
    description TEXT,
    creator_id UUID,
    created_at TIMESTAMP,
    encrypted_avatar_key TEXT  -- Encrypted reference to media
);

-- Group membership
CREATE TABLE group_members (
    group_id UUID,
    member_id UUID,
    role TINYINT,              -- 0=member, 1=admin, 2=owner
    joined_at TIMESTAMP,
    PRIMARY KEY (group_id, member_id)
);

-- Signal Protocol: Pre-keys
CREATE TABLE pre_keys (
    account_id UUID,
    key_id INT,
    public_key BLOB,
    PRIMARY KEY (account_id, key_id)
);

-- Signal Protocol: Signed pre-keys
CREATE TABLE signed_pre_keys (
    account_id UUID,
    key_id INT,
    public_key BLOB,
    signature BLOB,
    timestamp TIMESTAMP,
    PRIMARY KEY (account_id, key_id)
);

-- Signal Protocol: Identity keys
CREATE TABLE identity_keys (
    account_id UUID PRIMARY KEY,
    identity_key BLOB
);

-- Device push tokens (anonymous)
CREATE TABLE push_tokens (
    account_id UUID,
    device_id UUID,
    platform TINYINT,          -- 0=ios, 1=android
    token TEXT,
    updated_at TIMESTAMP,
    PRIMARY KEY (account_id, device_id)
);
```

---

## 5. Redis Schema

```
# Session tokens (account_id → token metadata)
session:{token_hash}  →  {account_id, device_id, created_at}  TTL: 30d

# Online presence (set when WS connected, del on disconnect)  
presence:{account_id}  →  {device_id, connected_at}  TTL: 65s (refreshed every 60s)

# Rate limiting (using Redis INCR + EXPIRE)
ratelimit:auth:{ip}   →  count  TTL: 60s
ratelimit:api:{account_id}  →  count  TTL: 60s

# Username availability cache (reduce DB reads)
username_check:{username}  →  "exists" | "free"  TTL: 60s
```

---

## 6. Security Hardening

### 6.1 Server-Side
- All endpoints require authentication (JWT) except `/auth/register` and `/auth/login`
- JWT validated on every request, no server-side session state
- Argon2id for password storage (m=65536, t=3, p=4)
- Rate limiting: 10 auth attempts/min per IP, 300 API calls/min per account
- Input validation on all fields (length, charset, type)
- CORS: Strict origin whitelist
- TLS 1.3 minimum, HSTS enforced
- No logging of message content, IP addresses, or user-identifiable metadata
- Structured logs contain only: request_id, endpoint, status_code, latency_ms

### 6.2 Memory Safety
- Rust prevents buffer overflows, use-after-free, data races at compile time
- `zeroize` crate used on all key material structs
- Sensitive config values loaded from environment, not hardcoded

### 6.3 Infrastructure
- Secrets managed by HashiCorp Vault (not .env in production)
- Network policies: Service-to-service only, no direct DB exposure
- Regular dependency audits: `cargo audit` in CI
- Container image scanning: Trivy

---

## 7. Deployment

### 7.1 Docker Compose (Dev)

```yaml
version: '3.9'
services:
  ghostlink-api:
    build: .
    ports: ["8080:8080"]
    env_file: .env
    depends_on: [scylladb, redis, nats]

  scylladb:
    image: scylladb/scylla:5.4
    ports: ["9042:9042"]
    volumes: ["scylla-data:/var/lib/scylla"]
    command: --developer-mode=1

  redis:
    image: redis:7-alpine
    ports: ["6379:6379"]

  nats:
    image: nats:2.10-alpine
    ports: ["4222:4222"]

  minio:
    image: minio/minio
    ports: ["9000:9000", "9001:9001"]
    environment:
      MINIO_ROOT_USER: ghostlink
      MINIO_ROOT_PASSWORD: changeme
    command: server /data --console-address ":9001"

volumes:
  scylla-data:
```

### 7.2 Kubernetes (Production)

```yaml
# k8s/deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: ghostlink-api
spec:
  replicas: 3
  selector:
    matchLabels:
      app: ghostlink-api
  template:
    spec:
      containers:
        - name: ghostlink-api
          image: ghostlink/api:latest
          ports:
            - containerPort: 8080
          resources:
            requests:
              cpu: "500m"
              memory: "512Mi"
            limits:
              cpu: "2000m"
              memory: "2Gi"
          livenessProbe:
            httpGet:
              path: /health
              port: 8080
            initialDelaySeconds: 10
            periodSeconds: 30
          readinessProbe:
            httpGet:
              path: /health/ready
              port: 8080
            initialDelaySeconds: 5
            periodSeconds: 10
          envFrom:
            - secretRef:
                name: ghostlink-secrets
```

---

## 8. CI/CD Pipeline

```yaml
# .github/workflows/ci.yml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo test --all
      - run: cargo clippy --all -- -D warnings
      - run: cargo fmt --all -- --check
      - run: cargo audit

  build:
    needs: test
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: cargo build --release
      - run: docker build -t ghostlink/api:${{ github.sha }} .
      - run: docker push ghostlink/api:${{ github.sha }}

  deploy-staging:
    needs: build
    if: github.ref == 'refs/heads/develop'
    runs-on: ubuntu-latest
    steps:
      - run: kubectl set image deployment/ghostlink-api ghostlink-api=ghostlink/api:${{ github.sha }} --namespace=staging

  deploy-production:
    needs: build
    if: github.ref == 'refs/heads/main'
    environment: production
    runs-on: ubuntu-latest
    steps:
      - run: kubectl set image deployment/ghostlink-api ghostlink-api=ghostlink/api:${{ github.sha }} --namespace=production
```

---

*End of Technical Specification v1.0*
