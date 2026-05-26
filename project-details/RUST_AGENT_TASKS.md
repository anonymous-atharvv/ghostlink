# GhostLink — Rust Agent Task Breakdown
## Ready-to-execute tasks for Claude agents

---

## Phase 1 Tasks (Foundation)

### TASK 1.1 — Workspace Scaffolding
**Prompt:** Create the Rust workspace with 6 crates: `ghostlink-api`, `ghostlink-core`, `ghostlink-db`, `ghostlink-ws`, `ghostlink-push`, `ghostlink-media`. Set up root `Cargo.toml` with workspace dependencies. Create `Cargo.toml` for each crate with appropriate dependencies. Create `.env.example` with all config vars.

**Acceptance:** `cargo check --all` passes with no errors.

---

### TASK 1.2 — Configuration System
**Prompt:** Implement `crates/ghostlink-api/src/config.rs`. Use the `config` crate to load from environment variables. All fields from `.env.example` must be typed and validated at startup. Fail fast on missing required config.

**Key struct:**
```rust
pub struct AppConfig {
    pub server_host: String,
    pub server_port: u16,
    pub scylla_nodes: Vec<String>,
    pub scylla_keyspace: String,
    pub redis_url: String,
    pub jwt_secret: String,
    pub nats_url: String,
    pub storage_backend: StorageBackend,
    // ... all fields from .env.example
}
```

---

### TASK 1.3 — Core Domain Entities
**Prompt:** Implement `crates/ghostlink-core/src/`. Create entities: `Account`, `Message`, `Group`, `Contact`. Create shared types: `PayloadType`, `MessageStatus`, `ContactStatus`, `GroupRole`. All entities must derive `Serialize`, `Deserialize`, `Clone`, `Debug`. Password-related fields must derive `Zeroize`.

---

### TASK 1.4 — Database Layer
**Prompt:** Implement `crates/ghostlink-db/src/`. Set up ScyllaDB connection pool using `scylla` crate. Set up Redis pool using `deadpool-redis`. Implement repos: `AccountRepo` (create, find_by_username, username_exists, update_last_seen, delete), `KeyRepo` (store_key_bundle, get_key_bundle, count_pre_keys). Write CQL migration files.

---

### TASK 1.5 — AppError + Middleware
**Prompt:** Implement `crates/ghostlink-api/src/error.rs` with the `AppError` enum mapping to HTTP status codes. Implement `middleware/auth.rs` (JWT extraction from Bearer header, validation, account lookup). Implement `middleware/rate_limit.rs` (tower-governor, per-IP for auth, per-account for API). Implement `middleware/request_id.rs`. Log middleware must NEVER log PII.

---

### TASK 1.6 — Auth Handlers
**Prompt:** Implement `POST /auth/register`, `POST /auth/login`, `POST /auth/logout`. Register: validate input, normalize username lowercase, check uniqueness, Argon2id hash, store account + Signal key bundle, issue JWT. Login: find account, verify Argon2id, issue JWT. Logout: invalidate session in Redis.

---

### TASK 1.7 — Docker Compose
**Prompt:** Create `docker-compose.yml` with services: `scylladb` (scylladb/scylla:5.4), `redis` (redis:7-alpine), `nats` (nats:2.10-alpine), `minio` (minio/minio). Create `Dockerfile` for the Rust app (multi-stage: build with rust:1.78 → run with debian:bookworm-slim).

---

## Phase 2 Tasks (Messaging)

### TASK 2.1 — Contact System
**Prompt:** Implement contact endpoints: `GET /contacts`, `POST /contacts` (by exact username), `PATCH /contacts/{id}` (accept/decline/block), `DELETE /contacts/{id}`. ContactRepo with status FSM: pending_sent → accepted/declined, block. No fuzzy search — exact username match only.

---

### TASK 2.2 — Signal Key Endpoints
**Prompt:** Implement `GET /keys/{username}/bundle` (fetch + consume one OTP key), `PUT /keys/pre-keys` (upload new OTP keys), `GET /keys/pre-keys/count`. KeyRepo must atomically consume OTP keys.

---

### TASK 2.3 — WebSocket Engine
**Prompt:** Implement `crates/ghostlink-ws/`. ConnectionHub using DashMap for concurrent access. Session state machine (Connected → Authenticated → Active). Wire protocol with JSON message types. Message router dispatching by `type` field. JWT auth on WS upgrade.

---

### TASK 2.4 — NATS Bridge
**Prompt:** Implement `crates/ghostlink-ws/src/nats_bridge.rs`. Subscribe to `user.{account_id}` subjects. When a message arrives for a user on another pod, publish to NATS. Receiving pod checks local DashMap and delivers via WebSocket.

---

### TASK 2.5 — Offline Queue
**Prompt:** Implement offline message queue. When recipient is offline: store in `offline_queue` table (7-day TTL), trigger push notification via NATS event. On reconnect: `GET /messages/offline` fetches queue, `DELETE /messages/offline` acknowledges and clears.

---

### TASK 2.6 — Receipts & Typing
**Prompt:** Implement delivery receipts (sent ✓, delivered ✓✓), read receipts (blue ✓✓, user-toggleable), and typing indicators. All routed through WebSocket. Receipt status updates stored in messages table.

---

## Phase 3 Tasks (Groups & Media)

### TASK 3.1 — Group CRUD
**Prompt:** Implement `POST /groups`, `GET /groups/{id}`, `DELETE /groups/{id}`, `POST /groups/{id}/members`, `DELETE /groups/{id}/members/{id}`, `PATCH /groups/{id}/members/{id}` (role change). Roles: owner, admin, member. Max 256 members.

---

### TASK 3.2 — Group Messaging
**Prompt:** Extend WebSocket router for `group_message.send`. Fan out to all online group members. Offline members get queued. Group messages use conversation_id = group_id.

---

### TASK 3.3 — Media Pipeline
**Prompt:** Implement `crates/ghostlink-media/`. Multipart upload handler (max 50MB). S3/MinIO storage abstraction (trait-based for testing). Download endpoint returns encrypted blob. TTL cleanup job runs hourly, purges media older than 30 days.

---

### TASK 3.4 — Push Notifications
**Prompt:** Implement `crates/ghostlink-push/`. APNs client (HTTP/2, JWT auth). FCM client (HTTP v1 API). Content-free payloads only: `{"type": "NEW_MESSAGE"}`. Triggered by NATS event when message is queued for offline user.

---

## Phase 4 Tasks (Production)

### TASK 4.1 — Observability
**Prompt:** Integrate `tracing-opentelemetry` for distributed tracing. Prometheus metrics: `ghostlink_http_requests_total`, `ghostlink_ws_connections_active`, `ghostlink_messages_sent_total`, `ghostlink_db_query_duration_seconds`. NEVER include per-user or per-conversation metrics.

---

### TASK 4.2 — CI/CD Pipeline
**Prompt:** Create `.github/workflows/ci.yml`: test → clippy → fmt → audit → build → docker push → deploy staging (develop branch) → deploy production (main branch, manual approval).

---

### TASK 4.3 — Kubernetes Manifests
**Prompt:** Create `k8s/` directory with: Deployment (3 replicas, resource limits, liveness/readiness probes), Service, Ingress (TLS), HPA (CPU-based autoscaling), ConfigMap, Secret references.

---

### TASK 4.4 — Security Audit
**Prompt:** Review entire codebase for: PII in logs, unvalidated input, missing auth checks, missing rate limits, hardcoded secrets, non-zeroized key material. Run `cargo audit`. Run `cargo clippy -- -D warnings`. Document findings.

---

*Each task is self-contained and can be executed independently by a Claude agent with the CLAUDE_AGENT_CONTEXT.md file.*
