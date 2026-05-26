# Privacy Policy
## GhostLink

**Effective Date:** January 1, 2025  
**Last Updated:** January 1, 2025  

---

## Introduction

GhostLink ("we," "our," or "us") operates the GhostLink anonymous messaging application (the "Service"). This Privacy Policy explains our data practices — or more precisely, our deliberate policy of not collecting your personal data.

**The short version:** We designed GhostLink so that we technically cannot identify you, read your messages, or provide your data to third parties, because we don't have any of that data to begin with.

---

## 1. Our Core Privacy Commitment

GhostLink was built from the ground up as a zero-knowledge platform. "Zero-knowledge" means:

- We do not know who you are in the real world
- We cannot read your messages (they are end-to-end encrypted)
- We do not store your IP address
- We do not know your location
- We have no way to link your GhostLink username to your real identity

This is not a marketing claim — it is a technical fact enforced by our architecture.

---

## 2. Information We Do NOT Collect

We do not collect, store, or process:

- Your real name
- Your email address
- Your phone number
- Your date of birth
- Your IP address (not logged)
- Your device's GPS location
- Your browsing history
- Your contact list from your phone
- Advertising identifiers (IDFA, GAID)
- Behavioral analytics data
- Biometric data
- Government-issued ID

We do not use third-party advertising networks. We do not use Facebook SDK, Google Analytics, or any behavioral tracking SDK.

---

## 3. Information We Do Collect and Why

### 3.1 Username

**What:** A pseudonymous username you choose (e.g., "ghost42")  
**Why:** Required to route messages to you  
**How stored:** Plaintext in our database  
**Retention:** Until you delete your account  
**Linkable to real identity?** No — the username is chosen by you and has no inherent connection to your real identity

### 3.2 Password

**What:** Your account password  
**How stored:** We store only an Argon2id cryptographic hash of your password. The hash is a one-way transformation — even GhostLink employees cannot determine your original password from the stored hash.  
**Linkable to real identity?** No

### 3.3 Public Cryptographic Keys

**What:** Your public keys used in the Signal Protocol for end-to-end encryption  
**Why:** Required so other users can establish encrypted sessions with you  
**How stored:** Plaintext (these are public keys by design)  
**Linkable to real identity?** No

### 3.4 Message Ciphertext (Encrypted Messages)

**What:** The encrypted blob of your messages during transit and temporary storage  
**Why:** Required to deliver messages to recipients who are offline  
**Can we read it?** No. Messages are encrypted on your device using the Signal Protocol before leaving your phone. GhostLink servers receive and store only encrypted ciphertext that we mathematically cannot decrypt.  
**Retention:** Maximum 30 days, then automatically and permanently deleted. Deleted messages are gone forever — we have no backup.

### 3.5 Media Ciphertext (Encrypted Files)

**What:** Encrypted image/file/audio blobs  
**Why:** Required to deliver media to recipients  
**Can we read it?** No. Media is encrypted on your device before upload.  
**Retention:** 30 days after upload, then permanently deleted

### 3.6 Push Notification Tokens

**What:** An anonymous device token issued by Apple (APNs) or Google (FCM)  
**Why:** Required to wake your device when new messages arrive  
**Does this identify you?** The token is linked to your GhostLink account ID only — not to your real identity  
**What we send in push notifications:** Only "New message" — we never include sender names, message previews, or conversation identifiers in push payloads

### 3.7 Last Seen Timestamp

**What:** A timestamp of when you last connected  
**Why:** Optional online/offline indicator feature  
**Linkable to real identity?** No  
**Can you disable this?** Yes, in Settings → Privacy

### 3.8 Group Membership

**What:** Which groups you are a member of (stored as account ID lists)  
**Why:** Required for group message routing  
**Can we read group messages?** No — group messages are end-to-end encrypted using Signal's Sender Key protocol

---

## 4. Information We Explicitly Do Not Log

Our servers are configured to suppress the following from all log files:

- IP addresses of connecting clients
- Request bodies (your messages, credentials)
- Usernames in HTTP access logs
- Account identifiers in access logs
- Message content

Our server logs contain only: timestamp, HTTP method, API endpoint path, HTTP status code, and response latency. No user-identifying information.

---

## 5. End-to-End Encryption

All private messages and group messages are protected by the **Signal Protocol**, the gold standard of end-to-end encryption. This means:

- Messages are encrypted on your device before transmission
- Only the intended recipient(s) can decrypt messages
- GhostLink servers are a blind relay — we cannot read your messages
- Even if our servers were seized or hacked, the attacker cannot read past messages
- We cannot comply with requests to provide message plaintext because we do not have it

---

## 6. Account Recovery

GhostLink has **no account recovery mechanism.** This is intentional.

To provide account recovery, we would need to collect at least one piece of identifying information (email, phone) — which we refuse to do. The absence of recovery is a privacy feature, not a bug.

**If you lose your credentials, your account is permanently inaccessible.** We strongly recommend you remember or securely store your username and password.

---

## 7. Data Retention and Deletion

| Data Type | Retention Period |
|-----------|-----------------|
| Account (username + password hash) | Until you delete your account |
| Public keys | Until you delete your account |
| Encrypted messages (in transit) | 30 days (offline queue), then auto-deleted |
| Delivered messages | Immediately deleted from server after delivery |
| Encrypted media | 30 days, then auto-deleted |
| Last seen timestamp | Overwritten on each connection |
| Push tokens | Until you log out or delete your account |
| Server logs | 7 days rolling window (no user-identifying data) |

**Account Deletion:**
You may delete your account at any time via Settings → Account → Delete Account. Deletion is immediate and permanent. We do not retain any data after account deletion.

---

## 8. Third-Party Services

We use the following infrastructure providers who may process metadata (not message content):

| Provider | Purpose | Data Shared |
|----------|---------|-------------|
| Cloudflare | DDoS protection, CDN | IP addresses (processed by Cloudflare, not stored by us) |
| AWS / Google Cloud | Server infrastructure | Encrypted server data only |
| Apple (APNs) | iOS push notifications | Anonymous device tokens + "New message" notification |
| Google (FCM) | Android push notifications | Anonymous device tokens + "New message" notification |

We have no advertising partners. We do not sell data to any third party.

**Note on Cloudflare:** We use Cloudflare in front of our API, which means Cloudflare's infrastructure processes user IP addresses according to Cloudflare's privacy policy. We recommend users concerned about IP privacy use a VPN or Tor.

---

## 9. International Data Transfers

Our servers are located in [jurisdiction TBD]. If you are located in a different country, your data (limited to the information described above) may be transferred to and processed in that country. We apply the same data minimization principles regardless of jurisdiction.

---

## 10. Legal Requests

We receive and review legal process requests from law enforcement agencies and courts. Our position:

**We will disclose only what we have, which is very little:**
- If compelled, we can provide: username, account creation timestamp, last seen timestamp, group membership (account IDs), push token
- We **cannot** provide: message content (we cannot decrypt it), your real identity (we don't have it), your IP address (we don't log it), your location (we don't collect it)

**We will:**
- Challenge overbroad requests
- Seek to notify affected users when legally permitted
- Publish a transparency report twice per year

**We will not:**
- Voluntarily provide data without legal compulsion
- Build backdoors into our encryption

---

## 11. Children

GhostLink is not intended for users under 18 years of age. We do not knowingly create accounts for individuals under 18. Because we do not collect age information (as we collect no personal information), we rely on users to self-certify their age at account creation. If you believe a minor is using our Service, please contact us.

---

## 12. Changes to This Policy

We will notify you of material changes to this Privacy Policy via an in-app notification. Your continued use of the Service after the effective date of the revised policy constitutes acceptance.

---

## 13. Contact Us

For privacy-related questions:  
**Email:** privacy@ghostlink.app  
**PGP Key:** [Key fingerprint published at ghostlink.app/pgp]  

We aim to respond to all privacy inquiries within 72 hours.

---

*GhostLink — Built for privacy. Not just promised.*
