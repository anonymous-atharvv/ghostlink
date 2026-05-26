# GhostLink — Agent Context File
## Drop this into any Claude/AI agent conversation for instant project understanding

---

## WHAT IS GHOSTLINK?

GhostLink is a **zero-knowledge anonymous messaging platform** for Android & iOS. Think "WhatsApp meets Signal, but with zero identity requirements." Users create accounts with only a username and password — no email, no phone, no real name. All messages are end-to-end encrypted using the Signal Protocol. The server is a blind relay that **mathematically cannot** read message content.

**Tagline:** *Zero logs. Zero trace. Zero identity.*

---

## FIVE UNBREAKABLE RULES

These rules are NEVER violated in any code, PR, or design decision:

1. **NO REAL IDENTITY** — No email, phone, real name collected. Ever.
2. **NO PLAINTEXT MESSAGES** — Signal Protocol E2EE. Server sees only ciphertext.
3. **NO IP LOGGING** — Server logs contain only: request_id, method, path, status, latency.
4. **NO ACCOUNT RECOVERY** — Lost credentials = lost account. This is a feature.
5. **ZEROIZE SECRETS** — All key material uses `zeroize` crate. Passwords are Argon2id only.

---

## TECH STACK

### 1. Backend (100% Rust)
- **Framework:** Axum 0.7 (HTTP + WebSocket)
- **Runtime:** Tokio 1.x
- **Primary DB:** ScyllaDB (Cassandra-compatible)
- **Cache:** Redis (sessions, presence, rate limits)
- **Message Bus:** NATS (cross-pod WebSocket routing)
- **Media Store:** AWS S3 / MinIO (encrypted blobs)
- **Auth:** JWT (HS256, 30-day expiry)
- **Password:** Argon2id (m=65536, t=3, p=4)
- **Encryption:** Signal Protocol (libsignal-client)

### 2. Android App (Kotlin)
- **Framework:** Jetpack Compose + Material 3
- **Local DB:** Room + SQLCipher (encrypted database)
- **Storage:** Android Keystore System (hardware-backed)
- **Network:** Retrofit + OkHttp
- **E2EE:** libsignal-android (Signal)
- **Integrity:** RootBeer root audit, FLAG_SECURE screenshot blocking

### 3. iOS App (Swift)
- **Framework:** SwiftUI + UIKit AppDelegate
- **Local DB:** GRDB.swift + SQLCipher (encrypted database)
- **Storage:** iOS Keychain + Secure Enclave
- **Network:** URLSession + WebSocketTask
- **E2EE:** libsignal-ios (Signal)
- **Integrity:** IOSSecuritySuite audit, active scene background blurring

---

## WORKSPACE STRUCTURE (Backend + Mobile)

```
ghostlink/
├── ghostlink-server/        # Backend Cargo workspace (6 crates)
│   └── crates/
│       ├── ghostlink-api/   # HTTP handlers, middleware, routing (Axum)
│       ├── ghostlink-core/  # Domain entities, business logic (no HTTP deps)
│       ├── ghostlink-db/    # ScyllaDB repos + Redis cache layer
│       ├── ghostlink-ws/    # WebSocket hub, sessions, NATS bridge
│       ├── ghostlink-push/  # APNs + FCM push dispatcher
│       └── ghostlink-media/ # S3 upload/download, TTL cleanup
│
├── ghostlink-android/       # Jetpack Compose native Android App
│   ├── app/                 # Main app module
│   └── gradle/              # Gradle version catalogs (libs.versions.toml)
│
├── ghostlink-ios/           # SwiftUI native iOS App
│   ├── GhostLink/           # App, Core, Domain, Features components
│   └── Package.swift        # Swift Package Manager dependencies manifest
│
└── implementation_plan.md   # Unified 6-phase implementation roadmap
```

---

## REFERENCE DOCS

| File | Contents |
|------|----------|
| `PRD.md` | Full product requirements, user personas, feature tables |
| `TECHNICAL_SPEC.md` | Rust & Mobile architectures, databases, secure schemas |
| `API_SPEC.md` | All 18 REST endpoints + WebSocket protocol |
| `SECURITY_SPEC.md` | Threat model, Signal Protocol details, key hierarchies |
| `ARCHITECTURE.md` | System topology, data flows, scaling strategy |
| `implementation_plan.md` | Complete product unified implementation roadmap |
| `RUST_AGENT_TASKS.md` | Ready-to-execute Rust tasks with exact prompts |
| `MOBILE_AGENT_TASKS.md` | Ready-to-execute Android & iOS tasks with exact prompts |

---

*GhostLink — Built for privacy. Not just promised.*

