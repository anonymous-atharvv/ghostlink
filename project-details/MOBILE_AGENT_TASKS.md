# GhostLink — Mobile Agent Task Breakdown
## Ready-to-execute tasks for Jetpack Compose (Android) & SwiftUI (iOS) agents

---

## Phase 2 Tasks (Mobile Foundations)

### TASK 2.1 — Android Scaffolding & Theme
**Prompt:** Configure the root Gradle files and app module of `ghostlink-android`. Implement a premium Jetpack Compose theme in `ui/theme/Theme.kt` with curated HSL-derived dynamic colors (dark theme primary, sleek dark backgrounds, and subtle green accents). Enable Gesture navigation and edge-to-edge layout in `MainActivity.kt`.

**Acceptance:** Android project compiles and launches into a dynamic premium dark UI.

---

### TASK 2.2 — Android Secure Storage & Database (SQLCipher)
**Prompt:** Implement `SecureKeyStore.kt` in `ghostlink-android` using Android Keystore to encrypt and decrypt sensitive fields. Wrap SQLite in SQLCipher via Room (`GhostLinkDatabase.kt`) using a database key derived and secured inside Keystore. Prevent database leakage by ensuring no plaintext keys exist in application code.

**Acceptance:** Room database operations are encrypted, database files are unreadable in plain text, and master keys reside strictly in Android Keystore.

---

### TASK 2.3 — Android Networking & Auth Repositories
**Prompt:** Set up Retrofit for HTTP REST APIs, including an `AuthInterceptor` to inject JWT bearer tokens dynamically. Implement `AuthRepository` for login and registration requests. Store authentication state (JWT, refresh token) inside encrypted storage.

**Acceptance:** API requests automatically include active JWT headers, and login status persists across application restarts.

---

### TASK 2.4 — Android Onboarding & Authentication UI
**Prompt:** Build Jetpack Compose screens for Onboarding (`WelcomeScreen.kt`, `RegisterScreen.kt`, `LoginScreen.kt`). Registration must display the mandatory privacy disclaimer: "No account recovery exists. Write down your credentials. Losing them means permanent loss of access." requiring active checkbox confirmation.

**Acceptance:** The user is forced to accept the zero-knowledge recovery warning before registration proceeds.

---

### TASK 2.5 — iOS Scaffolding & Privacy Blur
**Prompt:** Bootstrap the Swift Package Manager manifest and SwiftUI application entrypoint under `ghostlink-ios/`. In `GhostLinkApp.swift`, listen to scene transitions (`scenePhase`) and apply an elegant visual blur overlay over the screen when the app resigns active or moves to the background to prevent screenshot leak in task switchers.

**Acceptance:** Minimizing the iOS app immediately hides screen content under a premium blur overlay.

---

### TASK 2.6 — iOS Secure Storage & SQLCipher (GRDB)
**Prompt:** Implement `KeychainManager.swift` in `ghostlink-ios` to store cryptographic secrets in Apple Keychain backed by Secure Enclave. Setup GRDB.swift with SQLCipher database encryption using a master key stored inside Keychain.

**Acceptance:** Database records (contacts, chats, offline sessions) are written to SQLCipher-encrypted storage.

---

### TASK 2.7 — iOS Networking & Onboarding UI
**Prompt:** Implement the `APIClient` using URLSession. Build SwiftUI onboarding views (`WelcomeView.swift`, `RegisterView.swift`, `LoginView.swift`). Registration must block unless the zero-knowledge no-recovery checkbox is checked.

**Acceptance:** Authentic request flows work, warning checkbox acts as a strict onboarding validator, and credentials flow securely.

---

## Phase 3 Tasks (Signal Protocol & Real-time Messaging)

### TASK 3.1 — Signal Protocol Mobile Integration (X3DH / Ratchet)
**Prompt:** Integrate the official `libsignal-client` library into both Android and iOS applications. Implement `SignalManager` to manage:
1. Generation of identity keys (IK), signed pre-keys (SPK), and one-time pre-keys (OPK) on registration.
2. Atomically serializing and storing Signal sessions to the SQLCipher local database.
3. Decrypting received WS messages and encrypting outbound text payloads using the Double Ratchet session.

**Acceptance:** Messages are encrypted locally on-device and decrypted locally by the recipient with no plaintext payload passing through the server.

---

### TASK 3.2 — WebSocket Connection & Heartbeat
**Prompt:** Build a robust, power-optimized WebSocket client on Android (using OkHttp WS) and iOS (using URLSessionWebSocketTask). Implement:
1. Graceful automatic backoff reconnection on network loss.
2. Silent keep-alive ping-pong heartbeats to preserve socket status.
3. Offline queue retrieval on reconnect (`GET /messages/offline`).

**Acceptance:** Connection hub stays alive, reconnects within 3 seconds of network recovery, and fetches accumulated offline messages.

---

### TASK 3.3 — Chat Screen & Local Message Search
**Prompt:** Design a premium high-fidelity Chat view (Android Compose LazyColumn / iOS SwiftUI ScrollView) featuring smooth animations, message bubble tailoring, and inline search. Search must operate purely locally on the encrypted SQLCipher DB (using Room/GRDB FTS4 indices) to prevent exposing messages to external indexers.

**Acceptance:** Search queries are fast and executed 100% locally.

---

## Phase 4 Tasks (Groups & Media Pipelines)

### TASK 4.1 — Multi-Client Group Chats (Sender Keys)
**Prompt:** Implement group messaging using Signal's Group Sender Keys protocol. The mobile client must generate a Sender Key for group broadcasts, distribute this key via active 1:1 E2EE channels to members, and perform group message decryption/encryption.

**Acceptance:** Group broadcasts execute with single-payload broadcasts over WS, and members decrypt messages seamlessly.

---

### TASK 4.2 — Encrypted Media Upload & Decryption
**Prompt:** Implement the local media pipeline. Before upload:
1. Compress and crop pictures or record voice notes.
2. Generate an ephemeral AES-256 key, encrypt the file payload locally on-device.
3. Upload the encrypted blob to S3/MinIO.
4. Distribute the AES key and storage URL to the recipient within the E2EE Signal message.

**Acceptance:** Media is stored fully encrypted on S3, and only holding the Signal keys allows decryption during download.

---

## Phase 5 Tasks (Advanced Privacy Hardening)

### TASK 5.1 — Screen Security & Screenshot Protection
**Prompt:** Implement aggressive on-device visual shielding:
- Android: Enforce `FLAG_SECURE` globally in `MainActivity` to disable screenshots and block screen sharing.
- iOS: Detect screenshot captures via `UIApplication.userDidTakeScreenshotNotification` and display visual alerts. Inject a secure text field fallback to hide preview capture.

**Acceptance:** Screenshots are completely blocked on Android (resulting in black screens) and detected/alerted on iOS.

---

### TASK 5.2 — Biometric Re-Auth & App Locker
**Prompt:** Implement a configurable App Locker (PIN or Face ID / Fingerprint) utilizing standard security prompt integrations. If the app remains in the background longer than the defined interval (e.g., 1 min), lock the app immediately.

**Acceptance:** Returning to the app triggers a biometric check before restoring screen content.

---

### TASK 5.3 — Duress PIN & Panic Wipe
**Prompt:** Build a high-security "Panic Wipe" feature. If the user enters a pre-defined "Duress PIN" on the login or locker screen, immediately:
1. Wipe the JWT session from secure storage.
2. Invalidate all local Signal cryptographic keys.
3. Erase the Room/GRDB local SQLite database.
4. Exit the application immediately.

**Acceptance:** Entering the duress PIN instantly resets the app to its fresh-install state.
