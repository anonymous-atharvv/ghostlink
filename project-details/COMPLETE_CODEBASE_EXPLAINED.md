# GhostLink — Complete Codebase Explanation

> **Every file, every folder, every line — explained in detail.**
> Generated from full project analysis of all 107 files across 3 platforms.

---

## Table of Contents

1. [Project Overview](#1-project-overview)
2. [Root Documentation Files](#2-root-documentation-files)
3. [Server: Workspace Root](#3-server-workspace-root-ghostlink-server)
4. [Crate: ghostlink-api](#4-crate-ghostlink-api)
5. [Crate: ghostlink-core](#5-crate-ghostlink-core)
6. [Crate: ghostlink-db](#6-crate-ghostlink-db)
7. [Crate: ghostlink-ws](#7-crate-ghostlink-ws)
8. [Crate: ghostlink-media](#8-crate-ghostlink-media)
9. [Crate: ghostlink-push](#9-crate-ghostlink-push)
10. [Android App](#10-android-app-ghostlink-android)
11. [iOS App](#11-ios-app-ghostlink-ios)

---

## 1. Project Overview

```
/home/elliot/v2-startup/
├── *.md                          # 17 documentation files (PRD, API, Security, etc.)
├── ghostlink-server/             # Rust backend workspace (6 crates)
│   ├── Cargo.toml                # Workspace manifest
│   ├── Dockerfile                # Multi-stage Docker build
│   ├── docker-compose.yml        # Local dev infrastructure
│   ├── .env.example              # Config template
│   └── crates/                   # 6 Rust crates
├── ghostlink-android/            # Kotlin/Compose Android app
│   ├── build.gradle.kts          # Root Gradle config
│   ├── settings.gradle.kts       # Module settings
│   ├── gradle.properties         # Build properties
│   ├── gradle/libs.versions.toml # Version catalog
│   └── app/                      # Main Android module
│       ├── build.gradle.kts      # App-level build config
│       └── src/main/             # Sources + resources
│           ├── AndroidManifest.xml
│           ├── java/com/ghostlink/app/  # Kotlin sources
│           └── res/                     # Resources
└── ghostlink-ios/                # Swift/SwiftUI iOS app
    ├── Package.swift             # SwiftPM dependencies
    └── GhostLink/                # App sources
        ├── App/                  # App entry points
        └── Core/                 # Database, Keychain, Network
```

---

## 2. Root Documentation Files

### 2.1 `README.md` (77 lines)

Project landing page. States the core identity: "Zero logs. Zero trace. Zero identity." Lists the tech stack (Rust/Axum, ScyllaDB, Redis, Signal Protocol, Kotlin/Compose, Swift/SwiftUI) and provides quick-start commands.

**Key lines:**
- L1-4: Tagline and positioning
- L9-24: Documentation index — references all spec files
- L28-34: Five core product principles (no identity, no recovery, no logs, no trace, username discovery)
- L38-48: Technology stack table
- L53-66: Quick start (docker-compose, cargo run)
- L72-74: Legal links

### 2.2 `PRD.md` (300 lines)

Product Requirements Document. The full feature specification.

**Key sections:**
- L10-16: Executive summary — "Users communicate without revealing any real-world identity"
- L18-24: Problem statement — "Even 'private' apps like Signal require a phone number"
- L28-45: Goals (E2EE DMs, groups of 256, media sharing) + Non-goals (no recovery, no web client)
- L48-67: Three user personas — The Activist (primary), The Developer, The Privacy Enthusiast
- L72-198: Detailed feature tables with priority (P0-P3):
  - ACC-01 through ACC-09: Account system (3-32 char usernames, no recovery, panic wipe)
  - CON-01 through CON-05: Contact & discovery (exact username only, contact requests, blocking)
  - DM-01 through DM-15: Direct messaging (text, images, files, voice notes, receipts, typing indicators, disappearing messages)
  - GRP-01 through GRP-13: Group messaging (max 256 members, roles, invite links, Sender Keys)
  - MED-01 through MED-06: Media (images, video, voice notes, 30-day TTL, view-once)
  - NOT-01 through NOT-05: Push notifications (content-free: "New message" only)
  - SEC-01 through SEC-06: Security (app lock, screenshot prevention, panic wipe)
- L201-238: User flows (onboarding, new conversation, create group)
- L244-254: Performance targets (cold start <2s, P50 delivery <200ms, 50K concurrent connections)
- L258-264: Freemium monetization ($0 free, $3.99/mo Pro)
- L269-275: Compliance (no GDPR Art 15 applicable, CALEA cannot be complied with)
- L279-288: Success metrics (10K MAU at 3 months, 250K at 12 months)
- L293-297: Open questions (invite sharing, abuse prevention, jurisdiction)

### 2.3 `API_SPEC.md` (639 lines)

Complete REST + WebSocket API contract.

**Key sections:**
- L9-18: Auth — Bearer JWT on all endpoints except `/auth/*`, 30-day tokens, no refresh
- L23-96: Auth endpoints:
  - `POST /auth/register` — Create account with username + password + Signal key bundle (identity_key, signed_pre_key, one_time_pre_keys)
  - `POST /auth/login` — Login, returns JWT
  - `POST /auth/logout` — Invalidate session, 204 No Content
- L99-128: Account endpoints:
  - `GET /account/me` — Returns account_id, username, created_at
  - `DELETE /account/me` — Permanent deletion with password confirmation
- L130-199: Contact endpoints:
  - `GET /contacts` — List contacts with status
  - `POST /contacts` — Send request by exact username
  - `PATCH /contacts/{id}` — Accept/decline/block
  - `DELETE /contacts/{id}` — Remove contact
- L202-257: Key exchange endpoints:
  - `GET /keys/{username}/bundle` — Fetch X3DH key bundle, consumes one OTP
  - `PUT /keys/pre-keys` — Upload new one-time pre-keys
  - `GET /keys/pre-keys/count` — Check remaining OTP count
- L260-310: Message endpoints (REST fallback):
  - `GET /messages/offline` — Fetch queued offline messages
  - `DELETE /messages/offline` — Acknowledge + clear
  - `POST /messages/send` — Fallback when WS unavailable
- L312-435: Group endpoints — Full CRUD, member management, invite links
- L436-464: Media endpoints:
  - `POST /media/upload` — Multipart upload of encrypted blob
  - `GET /media/{id}` — Download encrypted blob
- L466-598: WebSocket protocol:
  - Client→Server: `message.send`, `group_message.send`, `typing.start`, `typing.stop`, `message.read`, `ping`
  - Server→Client: `message.incoming`, `message.ack`, `typing.indicator`, `pong`, `error`
- L601-636: Error codes and rate limits

### 2.4 `ARCHITECTURE.md` (353 lines)

System architecture, topology, data flows, scaling strategy.

**Key sections:**
- L10-42: High-level architecture diagram — Cloudflare → Nginx → Kubernetes Service Mesh → Auth/Message/Group/Media services → ScyllaDB/Redis/MinIO
- L48-85: Service descriptions (Auth, Message, Group, Media, Push) with pod counts and scaling strategies
- L90-148: Data flow diagrams:
  - Online delivery: Alice encrypts with Signal → WS → Message Service → Bob's pod → Bob's WS → decrypt
  - Offline delivery: store in ScyllaDB (7d TTL) → push via NATS → device wakes → fetch queue
  - Multi-device: separate Signal sessions per device
- L153-183: Horizontal scaling — NATS pub/sub for cross-pod WS routing, DashMap instead of RwLock<HashMap>
- L188-215: Database scaling — ScyllaDB partition key strategy, Redis cluster slot distribution
- L220-263: Security architecture — Cloudflare → Nginx → mTLS service mesh → network policies, Vault secret management
- L269-306: Privacy-preserving observability — Prometheus metrics (no per-user), OpenTelemetry spans (no PII)
- L311-329: Disaster recovery — hourly ScyllaDB snapshots, RTO/RPO targets
- L334-348: Infrastructure as Code — Terraform modules

### 2.5 `ARCHITECTURE_DEEP_DIVE.md` (342 lines)

Enterprise-grade internal architecture spec. More detailed than `ARCHITECTURE.md`.

**Key sections:**
- L7-18: Executive summary — "blind relay," 6-crate Rust workspace, mobile architectures, cryptographic foundation
- L22-31: Product vision — targets journalists, activists, human rights organizations; monetization via Pro pods
- L37-82: High-level system diagram + end-to-end request lifecycle (registration, WS upgrade)
- L87-106: Folder/repository breakdown — each crate's purpose
- L109-158: Mobile architecture deep dive:
  - Android: Compose → ViewModel → Repository → WS Client + SQLCipher Room
  - iOS: SwiftUI → ViewModel → APIClient/WSClient → SQLCipher GRDB
  - Keychain: `kSecAttrAccessibleAfterFirstUnlockThisDeviceOnly`
  - Background privacy overlay on iOS
- L161-185: Backend architecture — Axum middleware pipeline (timeout → request_id → rate_limit → auth → handler)
- L189-201: AI/Agentic system analysis — only on-device models, never server-side
- L206-228: Database ER diagram with partition key strategy
- L231-245: CI/CD pipeline maturity + Kubernetes hardening (non-root, multi-stage)
- L250-258: Security audit — vulnerability matrix (client memory clearing = High, push tracking = Medium, key leakage = Low)
- L263-273: Real-time pub/sub flow with NATS
- L278-284: Feature inventory — auth, encrypted cache, WS real-time, key bundles all complete
- L289-294: Scalability assessment — Redis connection pool, ScyllaDB time-series
- L298-302: Technical debt — real Signal bindings needed, key rotation pipeline
- L307-312: Engineering quality scorecard — 8.8/10 overall
- L316-320: CTO recommendations — native libsignal bindings, push token anonymization
- L324-330: Production launch checklist
- L335-338: Future roadmap — Phase 4 (groups/media), Phase 5 (privacy overlays), Phase 6 (production)
- L341-342: Final assessment — "exceptionally clean, decoupled, and secure software design"

### 2.6 `CLAUDE_AGENT_CONTEXT.md` (99 lines)

Concise project summary designed to be dropped into any AI agent conversation.

**Key sections:**
- L6-11: What is GhostLink — "zero-knowledge anonymous messaging platform"
- L14-23: Five unbreakable rules — NO identity, NO plaintext, NO IP logging, NO recovery, ZEROIZE
- L26-54: Tech stack tables (Rust, Android Kotlin, iOS Swift) with specific library versions
- L57-79: Workspace structure diagram
- L82-95: Reference docs table

### 2.7 `GHOSTLINK_MASTER_PROMPT_V2.md` (1066 lines)

The definitive master context file — supersedes all other docs. YC-grade engineering principles.

**Key sections (15 total):**
- L1-21: Usage instructions — hierarchy of authority: Five Rules > Security > Scalability > Code Style
- L26-48: Section 1 — Product identity and market thesis
- L55-85: Section 2 — Five unbreakable rules (expanded)
- L89-136: Section 3 — Technology stack with "why" rationale
- L139-194: Section 4 — Workspace structure with crate dependency graph
- L198-327: Section 5 — Scalability architecture (NATS, DashMap, partition keys, caching, SLA targets, HPA config)
- L331-408: Section 6 — API contracts (all 18 REST endpoints + WS protocol + rate limits)
- L411-483: Section 7 — Security constraints (threat model, Signal Protocol math, PR checklist, mobile invariants)
- L486-599: Section 8 — Full database schema (CQL for all 9+ tables + Redis schema)
- L603-791: Section 9 — Rust code patterns (handler pattern, AppError, DTO pattern, repository pattern, logging rules, test structure)
- L794-835: Section 10 — Code quality standards (CI gates, review checklist)
- L837-880: Section 11 — Feature scope (v1.0, v2.0, never)
- L883-939: Section 12 — Privacy-preserving observability (metrics we collect vs never collect, alerts)
- L943-987: Section 13 — Deployment & infrastructure (K8s topology, Dockerfile, secret management)
- L990-1023: Section 14 — Agent task execution protocol (8-step process + standard prompts)
- L1025-1057: Section 15 — YC-level engineering principles

### 2.8 `MASTER_PROMPT.md` (304 lines)

Earlier version of the master prompt (v1.0.0). Superseded by GHOSTLINK_MASTER_PROMPT_V2.md.

### 2.9 `MOBILE_AGENT_TASKS.md` (131 lines)

Ready-to-execute mobile task breakdown for AI agents.

**Key sections:**
- Phase 2 (Mobile Foundations): 7 tasks — Android scaffolding, secure storage, networking, onboarding UI, iOS scaffolding, iOS secure storage, iOS networking
- Phase 3 (Signal Protocol & Real-time): 3 tasks — Signal integration, WebSocket + heartbeat, chat screen
- Phase 4 (Groups & Media): 2 tasks — Sender Key groups, encrypted media pipeline
- Phase 5 (Privacy Hardening): 3 tasks — screenshot protection, biometric re-auth, duress PIN

Each task has an exact prompt to give an AI agent and acceptance criteria.

### 2.10 `RUST_AGENT_TASKS.md` (136 lines)

Ready-to-execute Rust task breakdown for AI agents.

**Key sections:**
- Phase 1 (Foundation): 7 tasks — workspace scaffolding, config system, core entities, database layer, AppError + middleware, auth handlers, Docker Compose
- Phase 2 (Messaging): 6 tasks — contact system, Signal key endpoints, WS engine, NATS bridge, offline queue, receipts & typing
- Phase 3 (Groups & Media): 4 tasks — group CRUD, group messaging, media pipeline, push notifications
- Phase 4 (Production): 4 tasks — observability, CI/CD, Kubernetes manifests, security audit

### 2.11 `SECURITY_SPEC.md` (419 lines)

Threat model, E2EE design, zero-knowledge architecture.

**Key sections:**
- L10-38: Threat model — 5 adversary types with capabilities and defenses
- L41-82: Signal Protocol implementation — X3DH, Double Ratchet, Curve25519, key types and lifecycle, Sender Key group encryption
- L87-125: Authentication security — Argon2id parameters (64MB, 3 iterations, 4 parallelism), JWT security, brute-force prevention (10/min per IP, progressive backoff)
- L130-194: Data minimization — what we store vs never store, log sanitization policy (only request_id, method, path, status, latency)
- L200-254: Transport security — TLS 1.3, HSTS, certificate pinning (Android XML + iOS URLSessionDelegate)
- L258-298: Mobile app security — Android Keystore/SQLCipher/FLAG_SECURE/RootBeer, iOS Secure Enclave/SQLCipher/blur overlay/IOSSecuritySuite
- L301-335: Push notification privacy — content-free payloads only ("New message"), no sender/preview/conversation ID
- L341-377: Abuse prevention — client-side PhotoDNA CSAM scanning, rate limiting, contact request system, device fingerprint banning
- L381-408: Incident response — breach protocol (cannot expose message content due to E2EE)
- L411-416: Penetration testing schedule (quarterly)

### 2.12 `TECHNICAL_SPEC.md` (785 lines)

Rust backend + mobile architecture engineering reference.

**Key sections:**
- L8-35: Architecture overview diagram
- L41-61: Technology choices table with specific library versions (axum 0.7, tokio 1.x, scylla 0.13, etc.)
- L64-160: Full project structure with every file listed
- L165-295: Core Rust: Auth handler — complete code for register and login with Argon2id hashing, JWT issuance, key bundle storage
- L300-354: WebSocket engine — ConnectionHub with Arc<RwLock<HashMap>> pattern (note: later upgraded to DashMap in actual code)
- L358-408: Environment configuration (.env.example)
- L412-446: Cargo.toml workspace dependencies
- L450-493: Signal Protocol E2EE design — key exchange flow, server's role (blind relay), group encryption
- L496-605: Database design — full CQL schema for all tables
- L609-624: Redis schema — session, presence, rate limiting keys
- L629-650: Security hardening — JWT validation, Argon2id, rate limiting, input validation, memory safety
- L654-735: Deployment — Docker Compose, Kubernetes deployment YAML
- L739-781: CI/CD pipeline — test → clippy → fmt → audit → build → deploy

### 2.13 `PRIVACY_POLICY.md` (229 lines)

Legal privacy policy document (GDPR/CCPA aligned). Covers data collection (usernames, password hashes, public keys, ciphertext), data retention, third-party services, and legal requests.

### 2.14 `TERMS_AND_CONDITIONS.md` (257 lines)

Legal terms of service. Covers prohibited conduct (CSAM, terrorism, doxxing), content ownership, encryption limitations, disclaimers, and liability.

### 2.15 `implementation_plan.md` (188 lines)

Unified 6-phase implementation roadmap with proposed changes for each component.

---

## 3. Server: Workspace Root (`ghostlink-server/`)

### 3.1 `Cargo.toml` (96 lines)

The Rust workspace manifest — defines all 6 crates and shared dependencies.

**Line-by-line:**

```
L1-10: [workspace] — declares 6 member crates, resolver = "2"
L12-16: [workspace.package] — shared metadata (version 0.1.0, edition 2021, AGPL-3.0)
L18-82: [workspace.dependencies] — ALL shared dependencies with versions:
  ─ axum 0.7 with ws, multipart, macros features
  ─ axum-extra 0.9 for typed headers
  ─ tower 0.4, tower-http 0.5 (cors, trace, request-id, set-header, limit)
  ─ tower_governor 0.4 (rate limiting)
  ─ tokio 1.x with "full" features
  ─ tokio-util 0.7 for IO utilities
  ─ futures 0.3
  ─ serde 1 + serde_json 1
  ─ scylla 0.13 (ScyllaDB driver)
  ─ deadpool-redis 0.15 (Redis connection pool)
  ─ redis 0.25 with tokio-comp + connection-manager
  ─ jsonwebtoken 9 (JWT)
  ─ argon2 0.5 (password hashing)
  ─ uuid 1 with v4, v7, serde features
  ─ zeroize 1 with derive (memory wiping)
  ─ validator 0.18 with derive (input validation)
  ─ regex 1, lazy_static 1
  ─ chrono 0.4 with serde
  ─ thiserror 1, anyhow 1
  ─ tracing 0.1, tracing-subscriber 0.3
  ─ tracing-opentelemetry 0.24, opentelemetry 0.23
  ─ metrics 0.23, metrics-exporter-prometheus 0.15
  ─ async-nats 0.35
  ─ aws-sdk-s3 1, aws-config 1
  ─ dashmap 5 (concurrent HashMap)
  ─ dotenvy 0.15 (.env loading)
  ─ base64 0.22
  ─ tokio-test 0.4
L87-96: [profile.release] — LTO, single codegen unit, strip symbols, panic=abort, opt-level=3
         [profile.dev] — no optimization, debug symbols on
```

### 3.2 `.env.example` (48 lines)

Environment variable template for all configuration:

```
L1-3: Header comment
L5-6: Server — HOST, PORT
L8-12: ScyllaDB — nodes (comma-separated), keyspace, optional username/password
L14-15: Redis — URL
L17-21: Security — JWT_SECRET (required, 256-bit), Argon2id params
L23-28: Media storage — backend (s3|minio|local), bucket, region
L30-34: Push notifications — APNs key ID, team ID, private key path, FCM server key
L36-37: NATS — URL
L39-41: Observability — OTLP endpoint, log level
L44-47: Limits — max message size (64KB), max media size (50MB), max group members (256), rate limits
```

### 3.3 `Dockerfile` (26 lines)

Multi-stage Docker build for production:

```
L1-8: Stage 1 (builder) — rust:1.78-bookworm, copies all sources, cargo build --release
L10-25: Stage 2 (runtime) — debian:bookworm-slim, installs ca-certificates, copies binary, creates non-root ghostlink user, exposes 8080
```

### 3.4 `docker-compose.yml` (66 lines)

Local development infrastructure:

```
L3-16: ghostlink-api service — builds from Dockerfile, port 8080, depends on scylladb/redis/nats
L18-29: scylladb — scylladb/scylla:5.4, developer mode (--smp 2 --memory 1G), healthcheck via nodetool
L31-42: redis — redis:7-alpine, AOF persistence, healthcheck via redis-cli ping
L44-49: nats — nats:2.10-alpine, JetStream enabled, ports 4222 (client) + 8222 (monitoring)
L51-61: minio — minio/minio, ports 9000 (S3 API) + 9001 (console), default credentials
L63-66: Named volumes for persistence (scylla-data, redis-data, minio-data)
```

### 3.5 `.github/workflows/ci.yml` (68 lines)

CI/CD pipeline with 4 jobs:

```
L1-11: Trigger on push to main/develop, PRs to main; env: CARGO_TERM_COLOR, RUSTFLAGS=-D warnings
L13-35: test job — checkout, install Rust (clippy + rustfmt), cache, fmt check, clippy -D warnings, cargo test, cargo audit
L37-50: build job — needs test, builds release, builds Docker image
L52-58: deploy-staging — needs build, runs on develop branch push
L60-68: deploy-production — needs build, runs on main branch push with manual approval
```

### 3.6 `k8s/` (empty directory)

Kubernetes manifests directory (placeholder for deployment YAMLs).

### 3.7 `migrations/` (empty directory)

CQL migration files directory (migrations are inline in `ghostlink-db/src/migrations.rs`).

### 3.8 `tests/integration/` (empty directory)

Integration tests directory (placeholder).

---

## 4. Crate: `ghostlink-api`

**Purpose:** HTTP gateway — Axum router, handlers, middleware, DTOs.

### 4.1 `Cargo.toml` (47 lines)

```
L1-8: Package definition, binary target named "ghostlink-server"
L10-47: Dependencies — all 5 sibling crates + workspace dependencies
  ─ ghostlink-core, ghostlink-db, ghostlink-ws, ghostlink-push, ghostlink-media (internal crates)
  ─ All the standard Axum/Tokio/Serde/Validator/etc. from the workspace
```

### 4.2 `src/main.rs` (84 lines)

Application entry point — the binary that starts the server.

**Line-by-line:**

```
L1-3: Imports — AppConfig, create_router, AppState from ghostlink_api;
       DatabaseConfig, migrations, Database from ghostlink_db;
       tracing_subscriber setup
L5-6: #[tokio::main] — async entry point
L7-8: Load .env file (dev only, silently ignored if missing)
L10-14: Initialize tracing — env-filter for log level, JSON output format
L16: Log "GhostLink server starting..."
L18-19: Load configuration from env vars
L20-24: Log config (host + port only, no secrets)
L26-33: Build DatabaseConfig from AppConfig fields
L35: Connect to ScyllaDB + Redis (Database::connect)
L38: Run CQL migrations
L40-50: Connect to NATS — warn on failure but don't crash (runs in single-pod mode without NATS)
L52-53: Build AppState with config, db, and optional nats client
L55-71: If NATS connected, spawn background task for NATS bridge (cross-pod WS routing)
L73-74: Build the Axum router with all routes
L76-79: Bind TCP listener on configured host:port
L81: Start axum::serve — runs forever
L83: Ok(())
```

### 4.3 `src/lib.rs` (52 lines)

Library root — exports modules and defines `AppState`.

**Line-by-line:**

```
L1-6: Public module declarations (config, error, handlers, middleware, models, router)
L8-9: Imports — Arc, Database types, all 5 repos, ConnectionHub
L11-15: Import AppConfig
L17-31: AppState struct — the shared state injected into all Axum handlers via `State()`:
  ─ config: Arc<AppConfig> (thread-safe config reference)
  ─ db: Arc<Database> (ScyllaDB session + Redis pool)
  ─ account_repo, key_repo, message_repo, contact_repo, group_repo: Repository instances
  ─ session_cache, presence_cache: Redis-backed caches
  ─ hub: ConnectionHub (DashMap of active WebSocket connections)
  ─ nats: Option<async_nats::Client> (optional NATS client for cross-pod routing)
L33-51: AppState::new() constructor — extracts ScyllaDB session and Redis pool from Database,
         creates all repos and caches, wraps in Arc for sharing
```

### 4.4 `src/config.rs` (87 lines)

Configuration loading from environment variables.

**Line-by-line:**

```
L1-41: AppConfig struct — ALL config fields:
  ─ server_host, server_port
  ─ scylla_nodes (Vec<String>, comma-separated), scylla_keyspace, scylla_username, scylla_password
  ─ redis_url
  ─ jwt_secret (required, will panic if missing)
  ─ argon2_memory_kb, argon2_iterations, argon2_parallelism
  ─ storage_backend, s3_bucket, s3_region
  ─ nats_url
  ─ log_level
  ─ max_message_size_bytes, max_media_size_bytes, max_group_members
  ─ rate_limit_auth_per_min, rate_limit_api_per_min
L43-81: from_env() — reads each field from env var with env_or() helper, panics on missing JWT_SECRET
L84-86: env_or() helper — returns env var value or default if not set
```

### 4.5 `src/error.rs` (122 lines)

Application error type — the only error type that reaches HTTP clients.

**Line-by-line:**

```
L1-4: Imports — StatusCode, IntoResponse, Json, serde_json::json
L6-43: AppError enum — 11 variants:
  ─ InvalidCredentials (401) — "invalid username or password" (same for both to prevent enumeration)
  ─ UsernameConflict (409) — "username already taken"
  ─ UserNotFound (404) — "user not found"
  ─ GroupNotFound (404) — "group not found"
  ─ Forbidden (403) — "forbidden"
  ─ RateLimited (429) — "rate limited"
  ─ TokenExpired (401) — "token has expired"
  ─ TokenInvalid (401) — "invalid token"
  ─ ContactExists (409) — "contact already exists or request pending"
  ─ Validation(String) (400) — includes validation error message
  ─ InternalError (500) — "internal error occurred" (NEVER exposes details)
L45-111: IntoResponse implementation — maps each variant to (status_code, error_code_string, message)
          Produces JSON: {"error": "CODE", "message": "human readable"}
L114-122: From<anyhow::Error> — converts anyhow errors to InternalError, logs internally
```

### 4.6 `src/router.rs` (49 lines)

Axum router assembly — wires up all routes and middleware.

**Line-by-line:**

```
L1-5: Imports — Router, middleware fn, routing methods
L7-9: Handler modules and middleware imports
L11-48: create_router(state: AppState) -> Router:
  L13-16: Public routes — POST /auth/register, POST /auth/login (no auth required)
  L18-36: Protected routes — all wrapped with auth_middleware:
    ─ GET/DELETE /account/me
    ─ GET/POST /contacts, PATCH/DELETE /contacts/:contact_id
    ─ GET /keys/:username/bundle, PUT /keys/pre-keys, GET /keys/pre-keys/count
    ─ GET/DELETE /messages/offline
    ─ GET /ws/connect (WebSocket upgrade)
  L38-41: Health routes — GET /health, GET /health/ready (always public)
  L44-48: Assemble: /v1 prefix for public+protected, merge health, apply request_id middleware globally
```

### 4.7 `src/middleware/mod.rs` (3 lines)

Module declarations — `auth`, `rate_limit`, `request_id`.

### 4.8 `src/middleware/auth.rs` (73 lines)

JWT authentication middleware — extracts and validates Bearer token.

**Line-by-line:**

```
L1-8: Imports — Request, State, Authorization header, middleware traits
L7-13: Claims struct (sub, exp, iat) for JWT decoding
L15-20: AuthenticatedAccount import
L22-73: auth_middleware:
  L30-34: Extract Authorization header, fail with TokenInvalid if missing
  L36-38: Strip "Bearer " prefix
  L40-49: Decode JWT with HS256 using config.jwt_secret, distinguish expired vs invalid
  L51-56: Parse account_id UUID from claims.sub
  L58-64: Verify account still exists in ScyllaDB via account_repo.find_by_id
  L66-70: Inject AuthenticatedAccount into request extensions for downstream handlers
  L72: Call next middleware/handler
```

### 4.9 `src/middleware/rate_limit.rs` (12 lines)

Rate limiting config (placeholder — TODO).

**Line-by-line:**
- L1-8: Rate limit configuration documentation
- L9: RateLimitConfig struct (empty)
- L11: TODO comment — will be wired as tower-governor layer

### 4.10 `src/middleware/request_id.rs` (36 lines)

Request ID middleware — generates UUID for tracing, logs without PII.

**Line-by-line:**

```
L1-5: Imports
L7-31: request_id_middleware:
  L11: Generate UUID v4 request_id
  L12: Insert RequestId into extensions
  L14-16: Capture method, path before request
  L18: Run the actual request
  L20: Calculate latency
  L22-29: Privacy-safe logging — ALWAYS logs: request_id, method, path, status, latency_ms
           NEVER logs: IP, username, body, account_id
L34-36: RequestId extension type
```

### 4.11 `src/models/mod.rs` + `requests.rs` + `responses.rs` (2 lines each)

Placeholder modules. Request/response DTOs are co-located with their handlers for clarity (e.g., `handlers/auth.rs` has RegisterRequest, AuthResponse).

### 4.12 `src/handlers/mod.rs` (7 lines)

Module declarations — `auth`, `health`, `account`, `contacts`, `keys`, `messages`, `websocket`.

### 4.13 `src/handlers/auth.rs` (178 lines)

Authentication handlers — register, login, JWT issuance.

**Line-by-line:**

```
L1-14: Imports — Axum extractors, Argon2id, jsonwebtoken, validator, UUID
L17-26: RegisterRequest — validated DTO:
  ─ username: length(3-32)
  ─ password: length(min=8)
  ─ identity_key: String (Base64 public key)
  ─ signed_pre_key: SignedPreKey struct
  ─ one_time_pre_keys: Vec<OneTimePreKey> (1-100 keys)
L28-34: LoginRequest — validated DTO with username + password
L38-43: AuthResponse — token, account_id, username
L47-52: JWT Claims struct — sub (account_id UUID), exp, iat
L56-121: register() handler:
  L62-63: Validate input
  L66: Normalize username to lowercase
  L68-73: Validate charset (alphanumeric + underscore only)
  L75-78: Check username availability via LWT query
  L80-86: Hash password with Argon2id (OsRng salt)
  L88-96: Create Account struct
  L98-108: Persist account + Signal key bundle atomically
  L110-111: Issue JWT
  L113-121: Return 201 Created with AuthResponse
L125-160: login() handler:
  L129-130: Validate input
  L132: Normalize username
  L134-139: Find account, same error for wrong username OR password
  L141-147: Verify Argon2id hash
  L149-150: Update last_seen (no IP, just timestamp)
  L152-153: Issue JWT
  L155-160: Return 200 with AuthResponse
L163-178: issue_jwt() — creates HS256 JWT with 30-day expiry, sub=account_id, no PII in claims
```

### 4.14 `src/handlers/account.rs` (86 lines)

Account management handlers — GET/DELETE /account/me.

**Line-by-line:**

```
L1-9: Imports — Axum, Argon2id, validator, zeroize
L11: AuthenticatedAccount from ghostlink-core
L13-18: DeleteAccountRequest — validated DTO with password field, Zeroize on drop
L22-28: AccountMeResponse — account_id (UUID), username, created_at (RFC3339), last_seen_at (optional)
L33-50: me() handler:
  ─ Returns account info (id, username, created_at, last_seen_at)
  ─ Looks up by auth_account.id from the middleware-injected extension
  ┆ If not found (shouldn't happen since auth middleware verified existence), returns UserNotFound
L54-86: delete() handler:
  ─ Validates request body
  ─ Finds account, verifies password with Argon2id
  ─ Zeroizes password after verification
  ─ Deletes from accounts + username_index tables
  ─ Invalidates all Redis sessions for this account
  ─ Returns 204 No Content
```

### 4.15 `src/handlers/contacts.rs` (154 lines)

Contact management handlers — list, add, respond (accept/decline/block), delete.

**Line-by-line:**

```
L1-14: Imports
L17-21: AddContactRequest — username (length 3-32)
L23-26: RespondContactRequest — action: ContactAction enum (Accept/Decline/Block)
L30-36: ContactResponse — contact_id, username, status, added_at
L42-59: list() handler:
  ─ Fetches contacts from ContactRepo
  ─ Maps to ContactResponse list
L63-103: add() handler:
  ─ Validates input
  ─ Cannot add yourself
  ─ Looks up target by exact username
  ─ Checks no existing relationship
  ─ Creates dual contact request (pending_sent for owner, pending_received for target)
  ─ Returns 201 Created
L107-140: respond() handler:
  ─ Verifies target exists in contact list
  ─ Matches on ContactAction:
    ─ Accept: requires status == pending_received, updates both sides to accepted
    ─ Decline: removes both sides
    ─ Block: updates owner side to blocked
L143-154: delete() handler:
  ─ Removes mutual contact relationship
  ─ Returns 204 No Content
```

### 4.16 `src/handlers/keys.rs` (105 lines)

Signal Protocol key exchange handlers.

**Line-by-line:**

```
L1-12: Imports
L16-20: UploadPreKeysRequest — validated DTO with Vec<OneTimePreKey> (1-100)
L24-27: KeyCountResponse — count: i64
L34-60: get_bundle() handler:
  ─ Normalize username, look up account
  ─ Fetch key bundle (identity_key + signed_pre_key + consumes one OTP key)
  ─ Cache remaining OTP count in Redis presence cache
  ─ Return KeyBundle JSON
L64-83: upload_pre_keys() handler:
  ─ Upload new OTP keys in bulk
  ─ Update cached count in Redis
  ─ Returns 204 No Content
L87-105: count_pre_keys() handler:
  ─ Check Redis cache first for OTP count
  ─ On cache miss, query ScyllaDB and hydrate cache
  ─ Return { "count": N }
```

### 4.17 `src/handlers/messages.rs` (62 lines)

Offline message handlers — fetch and acknowledge.

**Line-by-line:**

```
L1-9: Imports
L12-19: OfflineMessageResponse — message_id, conversation_id, sender_id, encrypted_payload (Base64), payload_type, created_at
L26-48: fetch_offline() handler:
  ─ Fetch offline messages from ScyllaDB offline_queue
  ─ Map to response DTOs with Base64-encoded payload
  ─ Return list of offline messages
L52-62: ack_offline() handler:
  ─ Clear all offline messages for the authenticated user
  ─ Returns 204 No Content
```

### 4.18 `src/handlers/websocket.rs` (186 lines)

WebSocket upgrade handler — the most complex handler.

**Line-by-line:**

```
L1-16: Imports — Axum WS types, futures (SinkExt/StreamExt), jsonwebtoken, UUID
L17-22: Claims struct for JWT decoding in WS auth
L26-31: ws_upgrade() — Axum WebSocketUpgrade handler, returns impl IntoResponse
         Calls handle_socket() with the upgraded socket

L33-186: handle_socket(socket, state) — the main WS lifecycle:
  L34: Split socket into sender and receiver halves
  L36-108: STEP 1 — JWT Authentication Handshake (5-second window):
    ─ Loop with select! between auth timeout (5s) and incoming messages
    ─ Expects a JSON message: {"type": "auth", "payload": {"token": "..."}}
    ─ Decodes JWT, looks up account in ScyllaDB
    ─ On success: breaks loop (authenticated)
    ─ On failure: sends error and returns
    ─ On timeout: sends auth_timeout error and returns

  L110-137: STEP 2 — Active State:
    ─ Creates unbounded mpsc channel for outgoing WS messages
    ─ Registers sender with local ConnectionHub (DashMap)
    ─ Sets user as online in Redis presence cache
    ─ Fetches and drains pending offline messages, sends each via the channel
    ─ Purges delivered items from offline queue

  L139-181: STEP 3 — Drive Concurrent Write/Read Event Loops:
    ─ write_loop: receives from mpsc channel, serializes to JSON, sends via WebSocket
    ─ read_loop: receives WebSocket messages, deserializes to WsMessage,
      dispatches to WsRouter::handle_message() for processing
    ─ Both run via tokio::join! (concurrent)

  L183-185: STEP 4 — Disconnect Cleanup:
    ─ Unregisters from ConnectionHub
    ─ Sets user as offline in Redis presence cache
```

---

## 5. Crate: `ghostlink-core`

**Purpose:** Pure domain layer — zero dependencies on HTTP, database, or network libraries.

### 5.1 `Cargo.toml` (15 lines)

```
L1-15: Dependencies — ONLY serde, uuid, chrono, thiserror, zeroize, validator, regex, lazy_static
       No Axum, no ScyllaDB, no Tokio — this crate is pure domain logic
```

### 5.2 `src/lib.rs` (6 lines)

Module declarations — `account`, `contact`, `crypto`, `group`, `message`, `types`.

### 5.3 `src/account.rs` (44 lines)

Account domain entity.

**Line-by-line:**

```
L1-5: Imports — chrono, serde, uuid, zeroize
L7-16: Account struct:
  ─ id: Uuid
  ─ username: String
  ─ password_hash: String (skipped in serialization for security)
  ─ created_at: DateTime<Utc>
  ─ last_seen_at: Option<DateTime<Utc>>
L18-24: AuthenticatedAccount — lightweight struct carried in request extensions:
  ─ id: Uuid, username: String
L26-37: SensitivePassword — wrapper for password bytes, zeroized on drop
L39-44: NewAccount — creation parameters (username + hash)
```

### 5.4 `src/contact.rs` (24 lines)

Contact relationship entity.

**Line-by-line:**

```
L1-5: Imports — chrono, serde, uuid
L8-15: Contact struct:
  ─ owner_id: Uuid, contact_id: Uuid, username: String
  ─ status: ContactStatus, added_at: DateTime<Utc>
L17-24: ContactAction enum — Accept, Decline, Block (rename_all = "snake_case")
```

### 5.5 `src/crypto.rs` (42 lines)

Signal Protocol key material types.

**Line-by-line:**

```
L1-4: Imports — serde, uuid, zeroize
L6-10: OneTimePreKey — key_id: i32, public_key: String (Base64)
L13-18: SignedPreKey — key_id: i32, public_key + signature: String (Base64)
L21-28: KeyBundle — account_id, identity_key, signed_pre_key, one_time_pre_key (Option)
L31-42: SensitiveKeyMaterial — Vec<u8> bytes, zeroized on drop
```

### 5.6 `src/group.rs` (38 lines)

Group chat entities.

**Line-by-line:**

```
L1-6: Imports
L8-16: Group struct — id, name, description (optional), creator_id, created_at, encrypted_avatar_key
L18-26: GroupMember struct — group_id, member_id, username, role (GroupRole), joined_at
L28-35: GroupInvite struct — group_id, token, created_by, expires_at
L38: MAX_GROUP_MEMBERS constant = 256
```

### 5.7 `src/message.rs` (56 lines)

Message entities and disappearing message configuration.

**Line-by-line:**

```
L1-6: Imports
L8-19: Message struct:
  ─ conversation_id, message_id, sender_id (UUIDs)
  ─ encrypted_payload: Vec<u8> (Signal ciphertext — server CANNOT decrypt)
  ─ payload_type: PayloadType, status: MessageStatus, created_at: DateTime<Utc>
L21-32: OfflineMessage — queued for offline recipients, includes recipient_id
L34-56: DisappearingTimer enum:
  ─ Off, OneHour, TwentyFourHours, SevenDays, ThirtyDays
  ─ ttl_seconds() returns Option<i64> (None for Off)
```

### 5.8 `src/types.rs` (113 lines)

Shared types and enums used across all crates.

**Line-by-line:**

```
L1: Imports
L3-26: PayloadType — Text(0), Image(1), File(2), Voice(3), Video(4)
        from_u8() for deserialization from DB (TINYINT)
L28-47: MessageStatus — Sent(0), Delivered(1), Read(2)
L49-58: ContactStatus — PendingSent, PendingReceived, Accepted, Blocked, Declined
L60-84: GroupRole — Member(0), Admin(1), Owner(2)
         is_admin_or_above() checks for Admin or Owner
L86-103: Platform — Ios(0), Android(1)
L106-113: MediaType — Image, Video, Audio, File
```

---

## 6. Crate: `ghostlink-db`

**Purpose:** Database access layer — ScyllaDB repos + Redis cache.

### 6.1 `Cargo.toml` (19 lines)

Dependencies: ghostlink-core, scylla, deadpool-redis, redis, tokio, uuid, chrono, serde, thiserror, tracing.

### 6.2 `src/lib.rs` (6 lines)

Exports: `Database`, `DatabaseConfig` from connection; modules: cache, connection, migrations, repos.

### 6.3 `src/connection.rs` (77 lines)

Database connection setup.

**Line-by-line:**

```
L1-6: Imports — Arc, Redis config/pool, Scylla Session/SessionBuilder
L8-16: DatabaseConfig struct:
  ─ scylla_nodes: Vec<String>, scylla_keyspace: String
  ─ scylla_username, scylla_password: Option<String>
  ─ redis_url: String
L18-23: Database struct:
  ─ scylla: Arc<Session>
  ─ redis: RedisPool
L25-77: Database::connect() — async:
  L28-45: Build ScyllaDB session with 3 connections per host, optional auth
  L48-56: Use keyspace (warn if not found — migrations will create it)
  L60-69: Create Redis connection pool from URL, verify with PING
  L72-76: Return Database handle
```

### 6.4 `src/migrations.rs` (151 lines)

CQL schema creation — idempotent via `IF NOT EXISTS`.

**Line-by-line:**

```
L7-151: run_migrations(session, keyspace):
  L12-20: Create keyspace with SimpleStrategy RF=1 (dev default)
  L22: Use keyspace
  L24-89: Migration 001 — Core schema:
    ─ accounts: account_id UUID PK, username, password_hash, created_at, last_seen_at
    ─ username_index: username TEXT PK, account_id UUID
    ─ messages: conversation_id UUID, message_id TIMEUUID, sender_id, encrypted_payload BLOB,
                payload_type TINYINT, status TINYINT, created_at TIMESTAMP
                CLUSTERING ORDER BY message_id DESC, TTL 30 days
    ─ offline_queue: recipient_id UUID, message_id TIMEUUID (ASC), conversation_id,
                     encrypted_payload BLOB, sender_id, payload_type, created_at
                     TTL 7 days
    ─ contacts: owner_id UUID, contact_id UUID, username, status TINYINT, added_at
    ─ identity_keys: account_id UUID PK, identity_key BLOB
    ─ signed_pre_keys: account_id UUID, key_id INT, public_key BLOB, signature BLOB, timestamp
    ─ pre_keys: account_id UUID, key_id INT, public_key BLOB
  L92-121: Migration 002 — Groups:
    ─ groups: group_id UUID PK, name, description, creator_id, created_at, encrypted_avatar_key
    ─ group_members: group_id UUID, member_id UUID, username, role TINYINT, joined_at
    ─ group_invites: token TEXT PK, group_id, created_by, expires_at (TTL 24h)
  L123-147: Migration 003 — Media + push tokens:
    ─ media: media_id UUID PK, account_id, media_type TINYINT, s3_key, encrypted_key, size_bytes, created_at (TTL 30d)
    ─ push_tokens: account_id UUID, device_id UUID, platform TINYINT, token TEXT, updated_at
```

### 6.5 `src/repos/mod.rs` (11 lines)

Re-exports all 5 repositories: AccountRepo, ContactRepo, GroupRepo, KeyRepo, MessageRepo.

### 6.6 `src/repos/account_repo.rs` (140 lines)

Account CRUD operations.

**Line-by-line:**

```
L1-10: Imports
L13-17: AccountRepo struct — wraps Arc<Session>
L19-29: username_exists() — SELECT from username_index, returns bool
L31-62: create() — INSERT into username_index with IF NOT EXISTS (LWT for uniqueness),
         checks if LWT was applied via rows_typed<(bool,)>, INSERT into accounts table
L64-81: find_by_username() — two-step: look up account_id from username_index,
         then find_by_id() with that ID
L83-110: find_by_id() — SELECT from accounts, map row to Account struct,
          handle Option<chrono::DateTime> for last_seen_at
L112-122: update_last_seen() — UPDATE accounts SET last_seen_at (timestamp only, NO IP)
L124-139: delete() — DELETE from accounts + DELETE from username_index
```

### 6.7 `src/repos/contact_repo.rs` (134 lines)

Contact relationship operations.

**Line-by-line:**

```
L1-8: Imports
L12-14: ContactRepo struct
L16-41: create_request() — dual INSERT:
  ─ Owner: status=0 (pending_sent)
  ─ Target: status=1 (pending_received)
  Uses toTimestamp(now()) for added_at
L43-57: accept() — UPDATE both sides to status=2 (accepted)
L59-68: block() — UPDATE owner side to status=4 (blocked)
L70-79: remove() — DELETE contact row
L82-96: are_contacts() — check if status == 2 for the pair
L98-133: list() — SELECT all contacts for an owner, map i8 status values to ContactStatus enum
```

### 6.8 `src/repos/group_repo.rs` (90 lines)

Group CRUD and membership operations.

**Line-by-line:**

```
L1-8: Imports
L17-30: create() — INSERT into groups with creator_id
L32-47: add_member() — INSERT into group_members with role
L49-62: remove_member() — DELETE from group_members
L64-73: delete() — DELETE from groups + group_members
L76-89: member_count() — SELECT count(*) from group_members
```

### 6.9 `src/repos/key_repo.rs` (169 lines)

Signal Protocol key storage — store, fetch-with-consumption, count, upload.

**Line-by-line:**

```
L1-10: Imports — OneTimePreKey, SignedPreKey from core
L14-16: KeyRepo struct
L18-58: store_key_bundle() — INSERT identity_key, signed_pre_key, all one_time_pre_keys
L60-133: get_key_bundle() — X3DH bundle fetch:
  ─ Fetch identity_key from identity_keys table
  ─ Fetch latest signed_pre_key (LIMIT 1)
  ─ Fetch + DELETE one OTP key atomically (read-then-delete — potential race, see CTO note)
  ─ Return KeyBundle with Option<OneTimePreKey>
L135-149: count_pre_keys() — SELECT count(*) from pre_keys
L151-168: upload_pre_keys() — bulk INSERT new OTP keys
```

### 6.10 `src/repos/message_repo.rs` (99 lines)

Message storage and offline queue management.

**Line-by-line:**

```
L1-8: Imports
L16-33: store_message() — INSERT into messages with now() for TIMEUUID
L35-52: enqueue_offline() — INSERT into offline_queue with now()
L54-87: fetch_offline() — SELECT all offline_queue for recipient, map to OfflineMessage structs
L89-98: clear_offline() — DELETE all offline_queue for recipient
```

### 6.11 `src/cache/mod.rs` (2 lines)

Module declarations: `presence_cache`, `session_cache`.

### 6.12 `src/cache/session_cache.rs` (49 lines)

Redis session management.

**Line-by-line:**

```
L1-8: Imports
L12-14: SessionCache struct — wraps deadpool-redis Pool
L16-34: store_session() — SETEX "session:{token_hash}" with JSON value, 30-day TTL
L36-41: invalidate() — DEL "session:{token_hash}"
L43-48: exists() — EXISTS "session:{token_hash}"
```

### 6.13 `src/cache/presence_cache.rs` (38 lines)

Redis presence tracking.

**Line-by-line:**

```
L1-8: Imports
L12-14: PresenceCache struct
L17-23: set_online() — SETEX "presence:{account_id}" with 65-second TTL
L25-30: set_offline() — DEL presence key
L32-37: is_online() — EXISTS presence key
```

---

## 7. Crate: `ghostlink-ws`

**Purpose:** WebSocket engine — connection hub, session management, wire protocol, cross-pod NATS routing.

### 7.1 `Cargo.toml` (22 lines)

Dependencies: ghostlink-core, ghostlink-db, axum, tokio, futures, serde, dashmap, async-nats, base64.

### 7.2 `src/lib.rs` (8 lines)

Public exports: `hub::ConnectionHub`, `protocol::WsMessage`. Module declarations.

### 7.3 `src/hub.rs` (82 lines)

Global WebSocket connection registry using DashMap.

**Line-by-line:**

```
L1-7: Imports — DashMap, Arc, mpsc, Uuid, WsMessage
L9: WsSender type alias — mpsc::UnboundedSender<WsMessage>
L11-18: ConnectionHub struct — Arc<DashMap<Uuid, Vec<WsSender>>> (account_id → list of device senders)
L20-25: new() — create empty DashMap
L27-35: register() — push sender to account's sender list (supports multi-device)
L37-46: unregister() — remove sender from list, remove account if empty
L48-62: send_to_account() — send message to all devices of an account, returns bool (was anyone connected)
L64-67: is_online_local() — check if account has connections on THIS pod
L69-76: connection_count() — total active connections across all accounts (for metrics)
L78-82: Default impl
```

### 7.4 `src/nats_bridge.rs` (57 lines)

NATS pub/sub for cross-pod WebSocket routing.

**Line-by-line:**

```
L1-6: Imports
L8-11: NatsBridge struct — NATS client + ConnectionHub
L13-18: new() — connect to NATS server
L20-32: publish_for_user() — serialize WsMessage to JSON, publish to "user.{account_id}" subject
L34-57: start_subscriber() — subscribe to "user.*" NATS subject:
  ─ Parse account_id from subject (strip "user." prefix)
  ─ Deserialize JSON payload to WsMessage
  ─ Deliver via local ConnectionHub (if the recipient is on this pod)
  ─ Runs as infinite loop in background task
```

### 7.5 `src/protocol.rs` (90 lines)

WebSocket wire protocol — all message types in a tagged enum.

**Line-by-line:**

```
L1-8: Imports — serde, uuid. Enum tagged by "type" field with "payload" content
L9-90: WsMessage enum:
  Client→Server:
    ─ MessageSend { request_id, recipient_id, conversation_id, encrypted_payload, payload_type }
    ─ GroupMessageSend { request_id, group_id, encrypted_payload, payload_type }
    ─ TypingStart { conversation_id }
    ─ TypingStop { conversation_id }
    ─ MessageRead { conversation_id, last_read_message_id }
    ─ Ping

  Server→Client:
    ─ MessageIncoming { message_id, conversation_id, sender_id, encrypted_payload, payload_type, created_at }
    ─ MessageAck { request_id, message_id, status }
    ─ TypingIndicator { conversation_id, account_id, is_typing }
    ─ Pong
    ─ Error { request_id, code, message }
```

### 7.6 `src/router.rs` (149 lines)

Incoming WebSocket message dispatcher — routes messages to handlers.

**Line-by-line:**

```
L1-6: Imports — Uuid, ConnectionHub, WsMessage, MessageRepo, PresenceCache
L8-148: WsRouter::handle_message():
  Match on WsMessage variant:
  
  MessageSend { recipient_id, conversation_id, encrypted_payload, payload_type }:
    ─ Generate message_id (Uuid v4)
    ─ Build MessageIncoming payload
    ─ STEP 1: Try local delivery via ConnectionHub
    ─ IF delivered locally:
      ─ Send MessageAck with status="delivered" to sender
    ─ ELSE (not on this pod):
      ─ Check Redis presence cache for global online status
      ─ IF globally online (another pod):
        ─ Publish to NATS "user.{recipient_id}" for cross-pod delivery
        ─ Send MessageAck with status="sent" to sender
      ─ ELSE (offline):
        ─ Enqueue in ScyllaDB offline_queue
        ─ Trigger push notification via NATS "push.wakeup"
        ─ Send MessageAck with status="sent" to sender
  
  TypingStart / TypingStop:
    ─ Currently just logs (TODO: route to conversation partner)
  
  MessageRead:
    ─ Currently just logs (TODO: update status in DB + notify sender)
  
  Ping:
    ─ Send Pong back
  
  _: Warn about unexpected server→client messages from client
```

### 7.7 `src/session.rs` (56 lines)

Per-connection session state machine.

**Line-by-line:**

```
L1: Import Uuid
L3-12: SessionState enum:
  ─ Connected (initial state, awaiting auth)
  ─ Authenticated { account_id, username } (JWT validated)
  ─ Disconnected (terminated)
L14-56: WsSession struct:
  ─ state: SessionState, connected_at: DateTime<Utc>
  ─ new(), authenticate(), disconnect()
  ─ account_id() -> Option<Uuid>
  ─ is_authenticated() -> bool
```

---

## 8. Crate: `ghostlink-media`

**Purpose:** Encrypted media blob upload/download/cleanup with S3/MinIO.

### 8.1 `Cargo.toml` (19 lines)

Dependencies: ghostlink-core, ghostlink-db, tokio, aws-sdk-s3, uuid, chrono, serde, thiserror, tracing.

### 8.2 `src/lib.rs` (3 lines)

Modules: `cleanup`, `storage`, `upload`.

### 8.3 `src/storage.rs` (54 lines)

S3/MinIO object storage abstraction.

**Line-by-line:**

```
L1-3: Imports — S3Client, ByteStream
L5-9: MediaStorage struct — client (S3Client), bucket (String)
L11-14: new() constructor
L16-28: upload() — PUT object to S3 via ByteStream, returns key
L30-41: download() — GET object from S3, collect bytes
L43-53: delete() — DELETE object from S3
```

### 8.4 `src/upload.rs` (14 lines)

Upload handler (stub).

**Line-by-line:**

```
L1-2: UploadHandler struct
L4-13: handle_upload() — returns UUID string, TODO: validate size, store via MediaStorage, record in DB
```

### 8.5 `src/cleanup.rs` (12 lines)

TTL cleanup job (stub).

**Line-by-line:**

```
L4-11: CleanupJob::start() — logs start, TODO: query expired media, delete from S3 + DB
```

---

## 9. Crate: `ghostlink-push`

**Purpose:** Content-free push notifications to APNs (iOS) and FCM (Android).

### 9.1 `Cargo.toml` (18 lines)

Dependencies: ghostlink-core, tokio, serde, uuid, async-nats, reqwest (rustls-tls), jsonwebtoken, chrono.

### 9.2 `src/lib.rs` (3 lines)

Modules: `apns`, `dispatcher`, `fcm`.

### 9.3 `src/apns.rs` (19 lines)

Apple Push Notification Service client (stub).

**Line-by-line:**

```
L3-6: ApnsClient struct — TODO: HTTP/2 client, team ID, key ID, private key
L8-10: new()
L12-18: send_notification() — TODO: implement HTTP/2 APNs request
         Payload: {"aps": {"alert": {"title": "GhostLink", "body": "New message"}, "content-available": 1}}
         Content-free — no sender, no preview, no conversation ID
```

### 9.4 `src/fcm.rs` (19 lines)

Firebase Cloud Messaging client (stub).

**Line-by-line:**

```
L3-5: FcmClient struct — TODO: FCM HTTP v1 API credentials
L7-9: new()
L11-18: send_notification() — TODO: implement FCM HTTP v1 request
         Payload: {"data": {"type": "NEW_MESSAGE"}}
         Content-free — no sender, no preview, no conversation ID
```

### 9.5 `src/dispatcher.rs` (26 lines)

Push notification router.

**Line-by-line:**

```
L1-7: Imports — ApnsClient, FcmClient
L5-8: PushDispatcher struct — holds APNs + FCM clients
L10-16: new() constructor
L18-25: notify() — match on platform (0=iOS→APNs, 1=Android→FCM), dispatch
```

---

## 10. Android App (`ghostlink-android/`)

### 10.1 `build.gradle.kts` (7 lines)

Root Gradle build — declares plugins (AGP, Kotlin Android, Kotlin Serialization, Hilt) without applying them.

### 10.2 `settings.gradle.kts` (17 lines)

Module settings — Google + Maven Central repos, root project name "GhostLink", includes ":app" module.

### 10.3 `gradle.properties` (7 lines)

Build properties — 2GB JVM heap, AndroidX, Jetifier, Kotlin style, parallel builds, caching.

### 10.4 `gradle/libs.versions.toml` (61 lines)

Version catalog — centralized dependency management.

**Key versions:**
- AGP 8.4.0, Kotlin 2.0.0, Compose BOM 2024.05.00
- Hilt 2.51.1, Retrofit 2.11.0, OkHttp 4.12.0
- Room 2.6.1, SQLCipher 4.5.4
- libsignal-client 0.62.1
- RootBeer 0.1.0, Coil 2.6.0

### 10.5 `app/build.gradle.kts` (101 lines)

App module build configuration.

**Key details:**
- L10: namespace `com.ghostlink.app`
- L14-17: minSdk 26 (Android 8.0), targetSdk 34, version 1.0.0
- L28-33: Release build — minification (R8/ProGuard), resource shrinking
- L36-53: Java 17, Compose enabled, Kotlin Compiler extension 1.5.14
- L57-101: Dependencies — Compose BOM, Hilt, Retrofit, OkHttp, Room, SQLCipher, libsignal, RootBeer, Coil

### 10.6 `AndroidManifest.xml` (32 lines)

- Permissions: INTERNET, ACCESS_NETWORK_STATE, RECORD_AUDIO, POST_NOTIFICATIONS
- `android:allowBackup="false"` (SECURITY — no cloud backup)
- Activity with MAIN/LAUNCHER intent filter
- Dark theme (Theme.GhostLink)

### 10.7 `GhostLinkApp.kt` (15 lines)

Application class — `@HiltAndroidApp` for DI, loads SQLCipher native library on startup.

### 10.8 `MainActivity.kt` (65 lines)

Main activity — sets `FLAG_SECURE` (blocks screenshots), enables edge-to-edge, sets Compose content with dark theme.

**Dark theme colors:** Primary #64B5F6 (blue), Secondary #81C784 (green), Background #0A0E17 (near-black), Surface #161B22 (dark gray).

### 10.9 `crypto/KeyBundleManager.kt` (11 lines)

Stub — manages signed pre-key renewal cycle. Currently empty body, just constructor injection of SignalManager.

### 10.10 `crypto/MediaEncryptor.kt` (55 lines)

AES-CBC media encryption/decryption for files.

**Line-by-line:**

```
L12: Singleton class with Hilt injection
L14: Uses "AES/CBC/PKCS5Padding"
L16-34: encryptStream() — reads input in 4KB chunks, encrypts with Cipher.update(), finalizes with doFinal()
L36-54: decryptStream() — same pattern but decrypting
```

### 10.11 `crypto/SignalManager.kt` (48 lines)

Signal Protocol integration manager — **currently stubs**.

**Line-by-line:**

```
L8-25: generateKeysAndRegister():
  ─ Creates dummy identity key "IK_{uuid}"
  ─ Creates dummy signed pre key "SPK_{uuid}"
  ─ Creates dummy signature "SIG_{uuid}"
  ─ Creates 50 dummy one-time pre keys "OPK_{N}_{uuid}"
  ─ Returns KeyBundle (all stubs — not real Signal keys)
L27-30: encryptPayload() — returns "CIPHERTEXT({plainText})_FOR_{recipient}"
L31-40: decryptPayload() — extracts text between parentheses
L43-48: KeyBundle data class — identityKey, signedPreKey, signature, oneTimePreKeys
```

**CRITICAL NOTE:** This is a structural placeholder. Real Signal Protocol integration via `libsignal-client` is not yet implemented.

### 10.12 `data/local/db/GhostLinkDatabase.kt` (23 lines)

Room database with 4 entities: AccountEntity, MessageEntity, ContactEntity, GroupEntity. Version 1, no schema export.

### 10.13 `data/local/db/dao/DAOs.kt` (74 lines)

Four Room DAOs with Flow-based reactive queries:

**AccountDao:** getMyAccount(), insertAccount(), clear()
**ContactDao:** getContacts() (Flow), insertContacts(), insertContact(), deleteContact(), clear()
**MessageDao:** getMessagesForConversation (Flow sorted ASC), insertMessage, updateMessageStatus, deleteMessage, clearConversation, clearAll
**GroupDao:** getGroups() (Flow), insertGroups(), insertGroup(), deleteGroup(), clear()

### 10.14 `data/local/db/entity/Entities.kt` (40 lines)

Four Room entities:

**AccountEntity:** id (PK), username, lastSeenAt
**MessageEntity:** messageId (PK), conversationId, senderUsername, payloadCiphertext, status (0/1/2), createdAt, isDisappeared, disappearTimerSeconds
**ContactEntity:** id (PK), contactUsername, status (0-4), createdAt
**GroupEntity:** groupId (PK), name, avatarUrl, role (0/1/2), createdAt

### 10.15 `data/local/keystore/SecureKeyStore.kt` (77 lines)

Android Keystore wrapper for AES-GCM encryption.

**Line-by-line:**

```
L13: Constructor — initializes master key in Android Keystore
L16: provider = "AndroidKeyStore" (hardware-backed on supported devices)
L17: keyAlias = "GhostLinkMasterKey"
L18: transformation = "AES/GCM/NoPadding"
L23-39: generateMasterKeyIfNeeded() — creates AES key with KeyGenParameterSpec
         (PURPOSE_ENCRYPT + PURPOSE_DECRYPT, BLOCK_MODE_GCM, NO_PADDING, randomized IV)
L41-44: getSecretKey() — retrieves key from Android Keystore
L46-58: encrypt() — AES-GCM encrypt, prepend 12-byte IV to ciphertext, Base64 encode
L60-76: decrypt() — Base64 decode, extract IV (first 12 bytes), AES-GCM decrypt with GCMParameterSpec
```

### 10.16 `data/local/keystore/SessionStore.kt` (71 lines)

Encrypted SharedPreferences for session persistence.

**Line-by-line:**

```
L6-15: Constructor — Context + SecureKeyStore, SharedPreferences "ghostlink_secure_prefs"
L17-20: saveToken() — encrypts JWT with SecureKeyStore, stores in prefs
L22-29: getToken() — reads encrypted token, decrypts
L31-43: saveUsername/getUsername — same pattern
L45-55: getDatabaseKey() — reads encrypted DB key from prefs, or generates new one
L57-66: generateAndSaveNewDbKey() — generates 32 random bytes, Base64 encodes, encrypts with Keystore, saves
L68-70: clear() — removes token and username from prefs
```

### 10.17 `data/remote/api/AuthInterceptor.kt` (22 lines)

OkHttp interceptor — injects `Authorization: Bearer {token}` header from SessionStore on all requests.

### 10.18 `data/remote/api/GhostLinkApi.kt` (40 lines)

Retrofit API interface — all REST endpoints:

- `POST auth/register` — `register(RegisterRequest): AuthResponse`
- `POST auth/login` — `login(LoginRequest): AuthResponse`
- `GET contacts` — `getContacts(): List<ContactDto>`
- `POST/PATCH/DELETE contacts` — CRUD
- `PUT keys/pre-keys` — `uploadPreKeys(PreKeyUploadRequest)`
- `GET keys/{username}/bundle` — `getKeyBundle(String): KeyBundleResponse`
- `GET/DELETE messages/offline` — offline message management

### 10.19 `data/remote/dto/Dtos.kt` (65 lines)

Kotlinx Serialization DTOs:

- `RegisterRequest`: username, password_hash, identity_key, signed_pre_key, signature, one_time_pre_keys (List)
- `LoginRequest`: username, password_hash
- `AuthResponse`: token, expires_in
- `ContactDto`: id, contact_username, status, created_at
- `AddContactRequest`, `UpdateContactRequest`
- `PreKeyUploadRequest`
- `KeyBundleResponse`: identity_key, signed_pre_key, signature, one_time_pre_key (nullable)
- `OfflineMessageDto`: message_id, sender_username, recipient_username, payload_ciphertext, created_at

### 10.20 `data/remote/websocket/ReconnectManager.kt` (36 lines)

Exponential backoff reconnection scheduler.

**Line-by-line:**

```
L7: Constructor — takes onReconnect callback
L9-13: State — Handler on main looper, attempt counter, maxDelay=30s, baseDelay=1s
L15-28: scheduleReconnect() — calculates delay = baseDelay * 2^attempt, clamped to maxDelay,
         adds random jitter (0-200ms), posts delayed reconnect callback
L30-35: reset() — reset attempt counter, cancel pending reconnects
```

### 10.21 `data/remote/websocket/WsClient.kt` (69 lines)

OkHttp WebSocket client.

**Line-by-line:**

```
L12-14: Constructor — SessionStore + WsMessageHandler
L16-23: State — OkHttpClient, WebSocket, ReconnectManager, connectionState (SharedFlow)
L25-58: connect() — build WebSocket request with JWT auth header:
  ─ OkHttpClient with 30s ping interval (keepalive)
  ─ WebSocketListener callbacks:
    ─ onOpen: reset reconnect, emit true to connectionState
    ─ onMessage: delegate to WsMessageHandler
    ─ onClosing: close socket, emit false
    ─ onFailure: emit false, schedule reconnect
L60-62: sendMessage() — sends JSON string via WebSocket
L64-68: disconnect() — reset reconnect, close socket, emit false
```

### 10.22 `data/remote/websocket/WsMessageHandler.kt` (79 lines)

Handles incoming WebSocket messages.

**Line-by-line:**

```
L15-18: Constructor — MessageDao + SignalManager
L22-58: handleIncomingJsonMessage():
  ─ Deserialize to WsWireMessage
  ─ Match on type:
    ─ "message.incoming":
      ─ Decrypt payload ciphertext with SignalManager
      ─ Create MessageEntity (conversationId = senderUsername for DMs)
      ─ Insert into local SQLCipher Room DB
    ─ "message.ack":
      ─ Update message status in local DB
L61-79: Wire format structs:
  ─ WsWireMessage { type, payload }
  ─ IncomingMessagePayload { message_id, sender_username, payload_ciphertext, created_at }
  ─ MessageAckPayload { message_id, status }
```

### 10.23 `data/repository/AuthRepository.kt` (106 lines)

Authentication business logic.

**Line-by-line:**

```
L16-21: Constructor — GhostLinkApi, SessionStore, SignalManager, AccountDao
L22: getMyAccount() — returns Flow from Room
L24: isLoggedIn() — checks if token exists in SessionStore
L26-62: register() — generates Signal key bundle, computes client password hash (SHA-256 stub),
         calls API, saves token + username to SessionStore, inserts local account
L64-91: login() — same pattern but no key generation
L93-96: logout() — clears SessionStore and local DB
L98-105: computeClientPasswordHash() — SHA-256 of (username + password + salt) — client-side stub
          **NOTE: Real production would use Argon2id on server, not client SHA-256**
```

### 10.24 `data/repository/ChatRepository.kt` (84 lines)

Chat business logic.

**Line-by-line:**

```
L17-22: Constructor — MessageDao, WsClient, SignalManager, SessionStore
L25-27: getMessagesForConversation() — Flow from Room
L29-75: sendMessage():
  ─ Gets own username from SessionStore
  ─ Encrypts with SignalManager (Double Ratchet stub)
  ─ Builds outbound message payload
  ─ Sends via WebSocket
  ─ Saves plaintext copy in local encrypted DB (for search)
  ─ Returns Result.success/failure

L78-84: OutboundMessagePayload data class
```

### 10.25 `data/repository/ContactRepository.kt` (94 lines)

Contact business logic.

**Line-by-line:**

```
L13-16: Constructor — GhostLinkApi + ContactDao
L17: getLocalContacts() — Flow from Room
L19-35: syncContactsFromServer() — fetches from API, saves to Room
L37-51: addContact() — API call + local insert
L53-67: acceptContactRequest() — PATCH with status=2, insert to Room
L69-83: blockContact() — PATCH with status=3, insert to Room
L85-93: deleteContact() — API call + local Room delete
```

### 10.26 `di/AppModule.kt` (35 lines)

Hilt module providing `IoDispatcher` and `DefaultDispatcher` qualifiers.

### 10.27 `di/CryptoModule.kt` (35 lines)

Hilt module providing SignalManager, KeyBundleManager, MediaEncryptor as singletons.

### 10.28 `di/DatabaseModule.kt` (67 lines)

Hilt module providing:

- SecureKeyStore (singleton, takes Context)
- SessionStore (singleton, takes Context + SecureKeyStore)
- GhostLinkDatabase (Room with SQLCipher — passphrase from SessionStore, SupportOpenHelperFactory)
- All 4 DAOs (AccountDao, ContactDao, MessageDao, GroupDao)

### 10.29 `di/NetworkModule.kt` (78 lines)

Hilt module providing:

- HttpLoggingInterceptor (Level.BASIC for zero-PII compliance)
- AuthInterceptor (takes SessionStore)
- OkHttpClient (auth + logging interceptors, 15s timeouts)
- Retrofit (base URL `https://api.ghostlink.app/v1/`, Gson converter)
- GhostLinkApi (Retrofit interface)
- WsClient (takes SessionStore + WsMessageHandler)

### 10.30 `ui/screens/onboarding/Imports.kt` (4 lines)

Import placeholder — just imports foundation/border and runtime/Composable.

### 10.31 `ui/screens/onboarding/OnboardingScreens.kt` (323 lines)

Three Compose screens with dark theme styling:

**WelcomeScreen (L22-102):**
- GhostLink branding (42sp blue text with letter spacing)
- Tagline: "Zero logs. Zero trace. Zero identity." in green
- "Create Encrypted Wallet ID" button (blue, rounded)
- "Access Existing Account" outlined button (gradient border)

**RegisterScreen (L105-239):**
- Username field (3-32 char limit)
- Password + Confirm Password fields (masked)
- Mandatory warning card with checkbox:
  "I understand that GhostLink DOES NOT store real-name details..."
- Client-side validation: username >= 3 chars, password >= 8 chars, passwords match, warning accepted
- "Generate Secure Identity" button

**LoginScreen (L242-323):**
- Username + Password fields
- "Unlock Wallet" button (green)

### 10.32 `res/values/strings.xml` (4 lines)

Only `app_name = "GhostLink"`.

### 10.33 `res/values/themes.xml` (9 lines)

Material NoActionBar dark theme — status bar color #0A0E17, window background #0A0E17.

### 10.34 `res/xml/backup_rules.xml` (8 lines)

Excludes `ghostlink.db`, `security_prefs.xml`, and `signal_keys/` from backup.

### 10.35 `res/xml/data_extraction_rules.xml` (14 lines)

Same exclusions for both cloud backup and device transfer — prevents SQLCipher database, encrypted prefs, and Signal keys from being extracted.

---

## 11. iOS App (`ghostlink-ios/`)

### 11.1 `Package.swift` (39 lines)

Swift Package Manager manifest — iOS 16.0 minimum, dependencies:
- GRDB.swift 6.24.0 (SQLite with SQLCipher)
- libsignal-client 0.36.0 (Signal Protocol)
- SDWebImageSwiftUI 2.2.0 (image loading)
- IOSSecuritySuite 1.9.0 (jailbreak detection)

### 11.2 `App/AppDelegate.swift` (21 lines)

iOS app delegate — checks for jailbreak and debugger on startup using IOSSecuritySuite. Prints warning if found (production would trigger panic wipe).

### 11.3 `App/GhostLinkApp.swift` (45 lines)

SwiftUI app entry point:

**Line-by-line:**

```
L1-7: Imports and @main struct
L9-34: body — WindowGroup with:
  ─ Dark background (#0A0E17)
  ─ "GhostLink: Anonymity Preserved" title in blue
  ─ "Secure E2EE Channel Active" subtitle
  ─ Privacy shield overlay: VisualEffectView (UIBlurEffect.dark) when scenePhase != active
     (prevents screen content from being visible in iOS task switcher)
L36-45: VisualEffectView — UIViewRepresentable wrapper for UIBlurEffect
```

### 11.4 `Core/Database/DatabaseManager.swift` (69 lines)

GRDB + SQLCipher database manager (singleton pattern).

**Line-by-line:**

```
L4-8: Shared instance, DatabaseQueue
L8-10: Private init — calls setupDatabase()
L12-34: setupDatabase():
  ─ Creates database at "ghostlink.sqlite" in documents directory
  ─ Retrieves SQLCipher passphrase from KeychainManager
  ─ Sets prepareDatabase callback to use passphrase
  ─ Creates DatabaseQueue with SQLCipher configuration
  ─ Runs migrations
L36-68: migrate():
  ─ v1_schema: creates accounts, contacts, messages tables with same schema as Android
```

### 11.5 `Core/Keychain/KeychainManager.swift` (69 lines)

iOS Keychain wrapper (singleton).

**Line-by-line:**

```
L4-6: Shared instance
L8-23: save() — kSecClassGenericPassword with kSecAttrAccessibleAfterFirstUnlockThisDeviceOnly
         (NEVER syncs to iCloud — privacy requirement)
L25-40: get() — kSecClassGenericPassword with return data
L42-50: delete() — kSecClassGenericPassword
L52-68: getDatabaseKey() — retrieve or generate 32-byte crypto-secure passphrase for SQLCipher
         Uses SecRandomCopyBytes for generation
```

### 11.6 `Core/Network/APIClient.swift` (47 lines)

URLSession REST client (singleton).

**Line-by-line:**

```
L3-7: Shared instance, base URL "https://api.ghostlink.app/v1"
L9-46: request<T>() — generic HTTP request:
  ─ Builds URLRequest with path, method, body
  ─ Sets Content-Type and Accept headers
  ─ Injects JWT from Keychain if available
  ─ URLSession data task with JSON decoding
  ─ Completion handler pattern (Result<T, Error>)
```

### 11.7 `Core/Network/WebSocketClient.swift` (160 lines)

URLSession WebSocket client (singleton with auto-reconnect).

**Line-by-line:**

```
L4-16: Shared instance, URLSessionWebSocketTask, exponential backoff state
L14-17: init — creates URLSession with self as delegate
L19-33: connect() — builds WebSocket request with JWT auth, resumes task, starts listen + ping loops
L35-53: listen() — recursive receive loop, decodes WsWireMessage, delegates to handleIncomingMessage
L55-67: sendPing() — 30-second ping interval, triggers disconnect on failure
L69-74: send() — sends JSON string via WebSocket with completion callback
L76-108: handleIncomingMessage() — decodes message.incoming payloads, decrypts locally, persists to GRDB
L110-113: decryptLocal() — stub (returns ciphertext as-is)
L115-126: handleDisconnect() — exponential backoff reconnect (baseDelay * 2^attempt, max 30s)
L128-132: disconnect() — normal closure
L135-146: Wire format structs: WsWireMessage, IncomingMessagePayload
L148-160: MessageRow — GRDB FetchableRecord/PersistableRecord for messages table
```

---

## Architecture Summary

```
┌─────────────────────────────────────────────────────────┐
│                    Mobile Clients                        │
│  ┌─────────────────────────────────────────────────┐   │
│  │    Android (Kotlin)         iOS (Swift)          │   │
│  │  ┌──────────────────┐  ┌──────────────────┐      │   │
│  │  │ Compose UI       │  │ SwiftUI Views     │      │   │
│  │  │ ViewModel (Hilt) │  │ ObservableObject  │      │   │
│  │  │ Repositories     │  │ Repositories      │      │   │
│  │  │ OkHttp WS + REST │  │ URLSession WS+HTTP│      │   │
│  │  │ SQLCipher Room   │  │ SQLCipher GRDB    │      │   │
│  │  │ Android Keystore │  │ Keychain+SecureEnclave│   │   │
│  │  └──────────────────┘  └──────────────────┘      │   │
│  └─────────────────────────────────────────────────┘   │
└──────────────────────┬──────────────────────────────────┘
                       │ WSS + HTTPS (TLS 1.3)
┌──────────────────────▼──────────────────────────────────┐
│               Rust Backend (Axum)                        │
│  ┌─────────────┐  ┌──────────────┐  ┌────────────────┐  │
│  │ ghostlink-  │  │ ghostlink-ws │  │ ghostlink-     │  │
│  │ api (HTTP)  │  │ (WS + NATS)  │  │ media (S3)     │  │
│  └──────┬──────┘  └──────┬───────┘  └──────┬─────────┘  │
│         │                │                  │            │
│  ┌──────▼────────────────▼──────────────────▼─────────┐  │
│  │ ghostlink-db (ScyllaDB repos + Redis cache)         │  │
│  └──────────────────────┬──────────────────────────────┘  │
│  ┌──────────────────────▼──────────────────────────────┐  │
│  │ ghostlink-core (Pure domain — zero deps)            │  │
│  └─────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────┘
```

## Security Invariants (Enforced Everywhere)

1. **No PII in logs** — `request_id_middleware.rs` verifies only method/path/status/latency
2. **No plaintext messages** — server stores only `encrypted_payload` (opaque BLOB)
3. **Zeroize secrets** — `SensitivePassword`, `SensitiveKeyMaterial` use `#[derive(Zeroize, ZeroizeOnDrop)]`
4. **Argon2id** — 64MB memory, 3 iterations, 4 parallelism; stored as hash only
5. **FLAG_SECURE** — Android `MainActivity.kt` blocks screenshots
6. **Background blur** — iOS `GhostLinkApp.swift` applies `UIBlurEffect.dark` on inactive
7. **No key backup** — Android `backup_rules.xml` + iOS `kSecAttrAccessibleAfterFirstUnlockThisDeviceOnly`

## Known Gaps (TODO Items)

| Location | Gap | Priority |
|----------|-----|----------|
| `crypto/SignalManager.kt` | Stub — returns "CIPHERTEXT(...)" strings | Critical |
| `push/apns.rs` | Empty struct — HTTP/2 client not implemented | High |
| `push/fcm.rs` | Empty struct — HTTP v1 client not implemented | High |
| `media/upload.rs` | Returns UUID stub — no real S3 storage | High |
| `media/cleanup.rs` | Empty loop — no TTL purge implemented | Medium |
| `middleware/rate_limit.rs` | Comment-only struct — tower-governor not wired | Medium |
| `key_repo.rs:94-121` | Read-then-delete race on OTP consumption | Medium |
| All Rust crates | Zero unit/integration tests | Critical |
| `AuthRepository.kt:98-105` | Client SHA-256 instead of server Argon2id | High |
| `handlers/websocket.rs:115-120` | Typing indicator routing not implemented | Low |
| `handlers/websocket.rs:130-138` | Read receipt not implemented | Low |
