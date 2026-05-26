# API Specification
## GhostLink REST + WebSocket API
**Version:** v1  
**Base URL:** `https://api.ghostlink.app/v1`  
**Protocol:** HTTPS (REST) + WSS (WebSocket)

---

## Authentication

All endpoints except `/auth/*` require a Bearer token in the Authorization header:

```
Authorization: Bearer <jwt_token>
```

Tokens are valid for 30 days. There is no refresh token — clients re-authenticate with username + password when expired.

---

## 1. Auth Endpoints

### POST /auth/register

Create a new anonymous account.

**Request:**
```json
{
  "username": "ghostrider42",
  "password": "my-secure-password",
  "identity_key": "BASE64_ENCODED_PUBLIC_KEY",
  "signed_pre_key": {
    "key_id": 1,
    "public_key": "BASE64",
    "signature": "BASE64"
  },
  "one_time_pre_keys": [
    { "key_id": 1, "public_key": "BASE64" },
    { "key_id": 2, "public_key": "BASE64" }
  ]
}
```

**Response 201:**
```json
{
  "token": "eyJ...",
  "account_id": "550e8400-e29b-41d4-a716-446655440000",
  "username": "ghostrider42"
}
```

**Errors:**
- `409 Conflict` — Username already taken
- `400 Bad Request` — Validation failure

```json
{ "error": "USERNAME_CONFLICT", "message": "Username is already taken" }
```

---

### POST /auth/login

```json
{
  "username": "ghostrider42",
  "password": "my-secure-password"
}
```

**Response 200:**
```json
{
  "token": "eyJ...",
  "account_id": "550e8400-e29b-41d4-a716-446655440000",
  "username": "ghostrider42"
}
```

**Errors:**
- `401 Unauthorized` — Invalid credentials (same message for wrong username OR password — never indicate which)

```json
{ "error": "INVALID_CREDENTIALS", "message": "Invalid username or password" }
```

---

### POST /auth/logout

Invalidates the current session token.

**Response 204:** (no body)

---

## 2. Account Endpoints

### GET /account/me

Returns current account info.

**Response 200:**
```json
{
  "account_id": "550e8400-...",
  "username": "ghostrider42",
  "created_at": "2025-01-01T00:00:00Z"
}
```

---

### DELETE /account/me

Permanently deletes the account and all associated data.  
Requires password confirmation.

**Request:**
```json
{ "password": "my-secure-password" }
```

**Response 204:** (no body)

---

## 3. Contact Endpoints

### GET /contacts

Returns contact list.

**Response 200:**
```json
{
  "contacts": [
    {
      "account_id": "uuid",
      "username": "shadow_wolf",
      "status": "accepted",  // "pending_sent" | "pending_received" | "accepted" | "blocked"
      "added_at": "2025-01-01T00:00:00Z"
    }
  ]
}
```

---

### POST /contacts

Send a contact request by username.

**Request:**
```json
{ "username": "shadow_wolf" }
```

**Response 201:**
```json
{
  "contact": {
    "account_id": "uuid",
    "username": "shadow_wolf",
    "status": "pending_sent"
  }
}
```

**Errors:**
- `404 Not Found` — Username does not exist (identical response to avoid username enumeration: "User not found")
- `409 Conflict` — Contact already exists or request pending

---

### PATCH /contacts/{account_id}

Accept, decline, or block a contact.

**Request:**
```json
{ "action": "accept" }  // "accept" | "decline" | "block"
```

**Response 200:**
```json
{ "status": "accepted" }
```

---

### DELETE /contacts/{account_id}

Remove a contact.

**Response 204:** (no body)

---

## 4. Key Exchange Endpoints

### GET /keys/{username}/bundle

Fetch a user's key bundle to initiate Signal Protocol X3DH.  
Consumes one OTP key from their bundle.

**Response 200:**
```json
{
  "account_id": "uuid",
  "identity_key": "BASE64",
  "signed_pre_key": {
    "key_id": 1,
    "public_key": "BASE64",
    "signature": "BASE64"
  },
  "one_time_pre_key": {
    "key_id": 5,
    "public_key": "BASE64"
  }
}
```

---

### PUT /keys/pre-keys

Upload new one-time pre-keys (top up when running low).

**Request:**
```json
{
  "one_time_pre_keys": [
    { "key_id": 10, "public_key": "BASE64" }
  ]
}
```

**Response 200:**
```json
{ "accepted": 1 }
```

---

### GET /keys/pre-keys/count

Check how many OTPs remain on the server (to know when to top up).

**Response 200:**
```json
{ "count": 42 }
```

---

## 5. Message Endpoints (REST — offline delivery)

### GET /messages/offline

Fetch messages queued while offline (call on reconnect).

**Response 200:**
```json
{
  "messages": [
    {
      "message_id": "timeuuid",
      "conversation_id": "uuid",
      "sender_id": "uuid",
      "encrypted_payload": "BASE64_CIPHERTEXT",
      "payload_type": 0,
      "created_at": "2025-01-01T12:00:00Z"
    }
  ]
}
```

---

### DELETE /messages/offline

Acknowledge receipt (clears offline queue for this account).

**Response 204:** (no body)

---

### POST /messages/send

Send a message when WebSocket is unavailable (fallback).

**Request:**
```json
{
  "recipient_id": "uuid",
  "conversation_id": "uuid",
  "encrypted_payload": "BASE64_CIPHERTEXT",
  "payload_type": 0
}
```

**Response 201:**
```json
{ "message_id": "timeuuid" }
```

---

## 6. Group Endpoints

### POST /groups

Create a group.

**Request:**
```json
{
  "name": "The Resistance",
  "member_usernames": ["shadow_wolf", "anon99"]
}
```

**Response 201:**
```json
{
  "group_id": "uuid",
  "name": "The Resistance",
  "members": [
    { "account_id": "uuid", "username": "ghostrider42", "role": "owner" },
    { "account_id": "uuid", "username": "shadow_wolf", "role": "member" }
  ],
  "created_at": "2025-01-01T00:00:00Z"
}
```

---

### GET /groups/{group_id}

**Response 200:**
```json
{
  "group_id": "uuid",
  "name": "The Resistance",
  "member_count": 3,
  "created_at": "2025-01-01T00:00:00Z"
}
```

---

### GET /groups/{group_id}/members

**Response 200:**
```json
{
  "members": [
    { "account_id": "uuid", "username": "ghostrider42", "role": "owner" }
  ]
}
```

---

### POST /groups/{group_id}/members

Add a member (admin only).

**Request:**
```json
{ "username": "new_member" }
```

**Response 201:**
```json
{ "account_id": "uuid", "username": "new_member", "role": "member" }
```

---

### DELETE /groups/{group_id}/members/{account_id}

Remove a member (admin) or self-leave.

**Response 204:** (no body)

---

### PATCH /groups/{group_id}/members/{account_id}

Promote/demote a member.

**Request:**
```json
{ "role": "admin" }  // "admin" | "member"
```

**Response 200:**
```json
{ "role": "admin" }
```

---

### DELETE /groups/{group_id}

Delete group (owner only).

**Response 204:** (no body)

---

### POST /groups/{group_id}/invite-link

Generate an invite link.

**Request:**
```json
{ "expires_hours": 24 }
```

**Response 201:**
```json
{
  "link": "https://ghostlink.app/join/abc123xyz",
  "token": "abc123xyz",
  "expires_at": "2025-01-02T00:00:00Z"
}
```

---

## 7. Media Endpoints

### POST /media/upload

Initiate a media upload. Client encrypts media locally before sending.

**Request:** `multipart/form-data`
- `file`: encrypted blob
- `media_type`: `image | video | audio | file`
- `encrypted_key`: base64 AES key encrypted with recipient's public key

**Response 201:**
```json
{
  "media_id": "uuid",
  "download_url": "https://media.ghostlink.app/m/abc123",
  "expires_at": "2025-01-31T00:00:00Z"
}
```

---

### GET /media/{media_id}

Download encrypted media.

**Response 200:** `application/octet-stream` (encrypted blob)

---

## 8. WebSocket Protocol

### Connection

```
WSS https://api.ghostlink.app/v1/ws/connect
Authorization: Bearer <token>   (as query param or header)
```

---

### Client → Server Messages

All WS messages are JSON with a `type` field.

**Send Message:**
```json
{
  "type": "message.send",
  "request_id": "client-generated-uuid",
  "payload": {
    "recipient_id": "uuid",
    "conversation_id": "uuid",
    "encrypted_payload": "BASE64",
    "payload_type": 0
  }
}
```

**Send Group Message:**
```json
{
  "type": "group_message.send",
  "request_id": "client-uuid",
  "payload": {
    "group_id": "uuid",
    "encrypted_payload": "BASE64",
    "payload_type": 0
  }
}
```

**Typing Indicator:**
```json
{
  "type": "typing.start",
  "payload": { "conversation_id": "uuid" }
}
```

```json
{
  "type": "typing.stop",
  "payload": { "conversation_id": "uuid" }
}
```

**Read Receipt:**
```json
{
  "type": "message.read",
  "payload": {
    "conversation_id": "uuid",
    "last_read_message_id": "timeuuid"
  }
}
```

**Ping (keepalive):**
```json
{ "type": "ping" }
```

---

### Server → Client Messages

**Message Received:**
```json
{
  "type": "message.incoming",
  "payload": {
    "message_id": "timeuuid",
    "conversation_id": "uuid",
    "sender_id": "uuid",
    "encrypted_payload": "BASE64",
    "payload_type": 0,
    "created_at": "2025-01-01T12:00:00Z"
  }
}
```

**Delivery Acknowledgment:**
```json
{
  "type": "message.ack",
  "payload": {
    "request_id": "client-uuid",
    "message_id": "timeuuid",
    "status": "delivered"
  }
}
```

**Typing Indicator:**
```json
{
  "type": "typing.indicator",
  "payload": {
    "conversation_id": "uuid",
    "account_id": "uuid",
    "is_typing": true
  }
}
```

**Pong:**
```json
{ "type": "pong" }
```

**Error:**
```json
{
  "type": "error",
  "payload": {
    "request_id": "client-uuid",
    "code": "RECIPIENT_NOT_FOUND",
    "message": "Recipient does not exist or is not a contact"
  }
}
```

---

## 9. Error Codes

| Code | HTTP | Meaning |
|------|------|---------|
| `INVALID_CREDENTIALS` | 401 | Wrong username or password |
| `TOKEN_EXPIRED` | 401 | JWT has expired |
| `TOKEN_INVALID` | 401 | Malformed JWT |
| `FORBIDDEN` | 403 | Insufficient permissions |
| `USER_NOT_FOUND` | 404 | User does not exist |
| `GROUP_NOT_FOUND` | 404 | Group does not exist |
| `USERNAME_CONFLICT` | 409 | Username already taken |
| `CONTACT_EXISTS` | 409 | Contact request already sent |
| `RATE_LIMITED` | 429 | Too many requests |
| `VALIDATION_ERROR` | 400 | Input validation failed |
| `INTERNAL_ERROR` | 500 | Server error (no details exposed) |

---

## 10. Rate Limits

| Endpoint | Limit |
|----------|-------|
| POST /auth/register | 3/hour per IP |
| POST /auth/login | 10/min per IP |
| POST /contacts | 30/hour per account |
| POST /media/upload | 50/hour per account |
| WebSocket messages | 60/min per connection |
| All other API | 300/min per account |

Rate limit headers returned on all responses:
```
X-RateLimit-Limit: 300
X-RateLimit-Remaining: 297
X-RateLimit-Reset: 1704067200
```

---

*End of API Specification v1*
