# 👻 GhostLink — Anonymous Messaging Platform

> **Zero logs. Zero trace. Zero identity.**
> Production-ready anonymous chat for Android & iOS, built in Rust.

---

## 📁 Project Documentation Index

| File | Purpose |
|------|---------|
| `PRD.md` | Product Requirements Document — full feature spec |
| `TECHNICAL_SPEC.md` | Rust backend + mobile architecture |
| `API_SPEC.md` | Complete REST + WebSocket API contracts |
| `DATABASE_SCHEMA.md` | ScyllaDB + Redis schema & data model |
| `SECURITY_SPEC.md` | Threat model, E2EE design, zero-knowledge architecture |
| `MOBILE_SPEC.md` | Android (Kotlin) + iOS (Swift) implementation guide |
| `ARCHITECTURE.md` | System design, service topology, data flow diagrams |
| `DEPLOYMENT.md` | Docker, Kubernetes, CI/CD, infra-as-code |
| `PRIVACY_POLICY.md` | Legal privacy policy (GDPR/CCPA aligned) |
| `TERMS_AND_CONDITIONS.md` | Terms of service |
| `CLAUDE_MASTER_PROMPT.md` | Single prompt to reconstruct full project context for AI teams |

---

## 🔑 Core Product Principles

1. **No identity required** — Username + password only. No email, phone, or real name.
2. **No recovery** — Lost credentials = lost account. By design.
3. **No logs** — Server never stores message content. Only routing metadata with TTL.
4. **No trace** — End-to-end encrypted. Server cannot read messages.
5. **Username-based discovery** — You must know someone's exact username to contact them.
6. **WhatsApp-parity features** — DMs, group chats, media sharing, delivery receipts.

---

## 🛠 Technology Stack

| Layer | Technology |
|-------|-----------|
| Backend | Rust (Axum framework) |
| Real-time | WebSocket (tokio-tungstenite) |
| Database | ScyllaDB (primary), Redis (sessions/cache) |
| Encryption | Signal Protocol (libsignal-protocol-rust) |
| Mobile | Kotlin (Android), Swift (iOS) |
| Infrastructure | Docker → Kubernetes (GKE/EKS) |
| Observability | OpenTelemetry (no user-data in traces) |

---

## 🚀 Quick Start (Dev)

```bash
# Clone and setup
git clone https://github.com/your-org/ghostlink
cd ghostlink

# Start backend services
docker-compose up -d scylladb redis

# Run Rust backend
cargo run --bin ghostlink-server

# Run migrations
cargo run --bin migrator
```

---

## ⚖️ Legal

- [Privacy Policy](./PRIVACY_POLICY.md)
- [Terms & Conditions](./TERMS_AND_CONDITIONS.md)

---

*GhostLink is built for privacy-conscious individuals. It is not a tool for illegal activity. See T&C.*
