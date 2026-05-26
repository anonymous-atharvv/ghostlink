# Product Requirements Document (PRD)
## GhostLink — Anonymous Messaging Platform
**Version:** 1.0.0  
**Status:** Final Draft  
**Last Updated:** 2025  
**Owner:** Product Team  

---

## 1. Executive Summary

GhostLink is a privacy-first, anonymous messaging platform for mobile (Android + iOS). Users communicate without revealing any real-world identity. There is no sign-up form requiring email or phone. There is no account recovery by design. All messages are end-to-end encrypted and no message content is stored on servers beyond the routing window.

The product targets privacy-conscious individuals, journalists, activists, developers, and anyone who values communication without surveillance.

---

## 2. Problem Statement

Existing messaging applications require identity verification (phone number, email), store metadata, and are subject to government data requests. Even "private" apps like Signal require a phone number. Users who need truly anonymous communication have no polished, production-ready option on mobile.

**The gap:** No mainstream mobile app offers WhatsApp-level UX with Signal-level privacy AND zero identity requirement.

---

## 3. Goals & Non-Goals

### Goals
- Allow anyone to create a pseudonymous account using only a username and password
- Provide 1:1 direct messaging with full E2EE
- Provide group messaging (up to 256 members) with full E2EE
- Support media sharing (images, files, voice notes)
- Achieve delivery receipts, read receipts, typing indicators
- Run entirely on Rust backend for memory safety and performance
- Be deployable on standard cloud infrastructure with zero vendor lock-in

### Non-Goals
- Account recovery of any kind
- Identity verification
- Public profiles or user discovery beyond exact username search
- Push notifications via APNs/FCM in a way that leaks content
- Ad monetization
- Web client (v1)
- Desktop client (v1)

---

## 4. User Personas

### Persona A — "The Activist" (Primary)
- Age: 25-40
- Needs: Communicate with colleagues without government surveillance
- Pain: Signal requires phone number, WhatsApp stores metadata
- Expectation: Works like WhatsApp, private like a burner phone

### Persona B — "The Developer" (Secondary)
- Age: 22-35
- Needs: Private channels for team discussions without corporate surveillance
- Pain: Slack reads all messages, Discord requires real email
- Expectation: Fast, reliable, API-accessible

### Persona C — "The Privacy Enthusiast"
- Age: 18-45
- Needs: Anonymous identity as a lifestyle choice
- Pain: Everything requires a phone number or real email today
- Expectation: Slick UX with actual privacy, not just a claim

---

## 5. Feature Requirements

### 5.1 Account System

| ID | Feature | Priority | Notes |
|----|---------|----------|-------|
| ACC-01 | Create account with username + password | P0 | Username 3–32 chars, alphanumeric + underscore |
| ACC-02 | Login with username + password | P0 | — |
| ACC-03 | No email, phone, or real-name field | P0 | Hard requirement |
| ACC-04 | No account recovery option | P0 | By design — displayed clearly in onboarding |
| ACC-05 | Remember username on device | P0 | Stored in device secure storage |
| ACC-06 | Remember password (optional, user-controlled) | P1 | Stored in device keychain/keystore |
| ACC-07 | Auto-lock on background (configurable: 1min/5min/never) | P1 | PIN or biometric re-auth |
| ACC-08 | Logout clears local session keys | P0 | |
| ACC-09 | "Panic wipe" — destroy all local data | P2 | Triple-tap power button or duress PIN |

**Username Rules:**
- 3–32 characters
- Lowercase alphanumeric + underscore only
- Must be globally unique
- Cannot be changed after creation
- Cannot be searched unless known exactly (no autocomplete, no directory)

**Password Rules:**
- Minimum 8 characters
- No maximum (server stores Argon2id hash only)
- No complexity requirements forced (user's choice)
- Never transmitted in plaintext

---

### 5.2 Contact & Discovery

| ID | Feature | Priority | Notes |
|----|---------|----------|-------|
| CON-01 | Add contact by exact username | P0 | No fuzzy search |
| CON-02 | Contact list stored locally + server-side encrypted | P0 | |
| CON-03 | Block a user | P0 | Blocks all contact attempts |
| CON-04 | Remove contact | P1 | Does not delete existing chat history |
| CON-05 | Contact request system | P1 | User must accept before receiving messages |

**Discovery Model:**
There is NO user directory. There is NO search-by-name, search-by-bio, or any discovery mechanism. To chat with someone, you must know their exact username. This is fundamental to the anonymity model.

---

### 5.3 Direct Messaging (1:1)

| ID | Feature | Priority | Notes |
|----|---------|----------|-------|
| DM-01 | Send text messages | P0 | |
| DM-02 | Send images (up to 10 MB) | P0 | Encrypted before upload |
| DM-03 | Send files (up to 50 MB) | P1 | |
| DM-04 | Send voice notes (up to 5 min) | P1 | |
| DM-05 | Message delivery receipts (single tick / double tick) | P0 | |
| DM-06 | Read receipts (blue tick) — user-toggleable | P1 | |
| DM-07 | Typing indicator | P1 | |
| DM-08 | Message reactions (emoji) | P2 | |
| DM-09 | Reply to specific message (quote) | P1 | |
| DM-10 | Forward message | P2 | |
| DM-11 | Delete message for me | P0 | |
| DM-12 | Delete message for everyone (within 1 hour) | P1 | |
| DM-13 | Disappearing messages (configurable: off/1h/24h/7d/30d) | P1 | |
| DM-14 | Message search (local only) | P2 | |
| DM-15 | End-to-end encryption mandatory | P0 | Signal Protocol |

---

### 5.4 Group Messaging

| ID | Feature | Priority | Notes |
|----|---------|----------|-------|
| GRP-01 | Create group with name + optional avatar | P0 | |
| GRP-02 | Add members by username | P0 | Max 256 members |
| GRP-03 | Admin role — promote/demote members | P0 | |
| GRP-04 | Remove member from group | P0 | Admin only |
| GRP-05 | Leave group | P0 | |
| GRP-06 | Delete group (creator/admin) | P1 | |
| GRP-07 | Group invite link | P1 | Expiring link, shareable externally |
| GRP-08 | Group invite link revocation | P1 | |
| GRP-09 | All DM features in group context | P0 | |
| GRP-10 | @mention specific member | P1 | |
| GRP-11 | Group description (admin-editable) | P2 | |
| GRP-12 | Group announcement mode (admin-only posts) | P2 | |
| GRP-13 | E2EE for groups using Sender Keys | P0 | Signal's group encryption |

---

### 5.5 Media & Files

| ID | Feature | Priority | Notes |
|----|---------|----------|-------|
| MED-01 | Image preview in chat | P0 | |
| MED-02 | Video playback in chat (max 50 MB) | P1 | |
| MED-03 | Audio waveform for voice notes | P1 | |
| MED-04 | Encrypted media stored on server with TTL | P0 | 30 days default, then purged |
| MED-05 | "View once" media (auto-deletes after viewed) | P2 | |
| MED-06 | No cloud backup of media | P0 | Hard requirement |

---

### 5.6 Notifications

| ID | Feature | Priority | Notes |
|----|---------|----------|-------|
| NOT-01 | Local notifications for new messages | P0 | |
| NOT-02 | Push notifications via anonymous push token | P0 | Content never in payload |
| NOT-03 | Notification content: "New message" only — no sender, no preview | P0 | |
| NOT-04 | Mute conversation | P1 | |
| NOT-05 | Notification badge on app icon | P0 | |

**Push Notification Architecture:**
GhostLink uses anonymous push tokens. The server stores the device push token linked to an account ID but never includes message content in the push payload. The mobile app fetches message content via authenticated WebSocket after wake-up.

---

### 5.7 Security & Privacy Settings

| ID | Feature | Priority | Notes |
|----|---------|----------|-------|
| SEC-01 | App lock (PIN / biometric) | P0 | |
| SEC-02 | Screen security (block screenshots) | P0 | Android FLAG_SECURE, iOS private mode |
| SEC-03 | Incognito keyboard (disable keyboard learning) | P1 | |
| SEC-04 | Panic wipe | P2 | |
| SEC-05 | View linked devices | P2 | |
| SEC-06 | Export encryption keys (advanced users) | P3 | |

---

## 6. User Flows

### 6.1 Onboarding Flow
```
App Launch
    └── First Launch?
        ├── YES → Welcome Screen
        │         └── "Create Account" button
        │               ├── Enter username (availability check)
        │               ├── Enter password
        │               ├── Confirm password
        │               ├── WARNING SCREEN: "No account recovery exists. 
        │               │   Write down your credentials. Losing them means
        │               │   permanent loss of access."
        │               ├── Checkbox: "I understand and accept" (required)
        │               └── → Home Screen
        └── NO → Login Screen
                    ├── Enter username
                    ├── Enter password
                    └── → Home Screen (or App Lock if enabled)
```

### 6.2 Start New Conversation
```
Home Screen
    └── Tap "New Chat" (+)
        └── Enter exact username
            ├── Username found? → Open chat / Send contact request
            └── Username not found? → "User not found" (no suggestion)
```

### 6.3 Create Group
```
Home Screen
    └── Tap "New Group"
        ├── Enter group name
        ├── (Optional) Set group avatar
        ├── Add members by username (one by one)
        └── Create → Group Chat opens
```

---

## 7. Performance Requirements

| Metric | Target |
|--------|--------|
| App cold start time | < 2 seconds |
| Message delivery latency (P50) | < 200ms |
| Message delivery latency (P99) | < 1 second |
| Media upload time (10 MB image) | < 5 seconds on 4G |
| WebSocket reconnect time | < 3 seconds |
| Backend API response (P99) | < 500ms |
| Concurrent connections per server | 50,000+ |
| Message throughput | 1M messages/day per server |

---

## 8. Monetization (v1 — Freemium)

| Tier | Price | Features |
|------|-------|---------|
| Free | $0 | 1:1 DMs, groups up to 10 members, 10 MB media |
| Pro | $3.99/mo | All features, groups up to 256, 50 MB media, disappearing messages |

**Payment:** Anonymous payment options explored (crypto, gift cards). No real name tied to subscription. Payment processor sees transaction; GhostLink never stores payment details.

---

## 9. Compliance & Legal

- **No GDPR Article 15 (right of access) applicable** — We do not store personal data. We cannot provide data we do not have.
- **No CCPA applicable** — We do not sell data. We do not have data to sell.
- **CALEA compliance** — We technically cannot comply with wiretap orders because we cannot decrypt messages. This must be stated clearly in the legal docs and accepted as a business risk.
- **App Store compliance** — Must meet Apple App Store and Google Play content policies.
- **Age restriction** — 17+ (App Store) / 13+ with parental consent (Play Store — not recommended for this use case; use 18+).

---

## 10. Success Metrics

| Metric | 3-Month Target | 12-Month Target |
|--------|---------------|-----------------|
| MAU | 10,000 | 250,000 |
| D7 Retention | 35% | 50% |
| D30 Retention | 20% | 35% |
| Avg daily messages per active user | 15 | 30 |
| Crash-free sessions | 99.5% | 99.9% |
| App Store rating | 4.0+ | 4.5+ |

---

## 11. Open Questions

1. **Invite system**: How do users share their username without revealing it to third parties? (QR code? Ephemeral link?)
2. **Abuse prevention**: Without identity, how do we handle spam/CSAM while maintaining anonymity? (Hash-based CSAM scanning without reading content?)
3. **Infrastructure jurisdiction**: Which country to incorporate in for maximum legal protection?
4. **Crypto payments**: Integrate Monero/crypto payment in v1 or v2?

---

*End of PRD v1.0*
