# System Architecture
## GhostLink — Production Architecture Design
**Version:** 1.0.0

---

## 1. High-Level Architecture

```
╔═════════════════════════════════════════════════════════════════════╗
║                        CLIENT APPLICATIONS                         ║
║     Android (Kotlin + Jetpack Compose)  iOS (Swift + SwiftUI)      ║
╠═════════════════════════════════════════════════════════════════════╣
║                           CDN / EDGE                               ║
║         Cloudflare (Anti-DDoS, IP Anonymization, TLS)              ║
╠══════════════════════╦══════════════════════════════════════════════╣
║    LOAD BALANCER     ║         API GATEWAY (Nginx)                  ║
║    (HAProxy / GLB)   ║    Rate Limiting | TLS Termination           ║
╠══════════════════════╩══════════════════════════════════════════════╣
║                        SERVICE MESH (Kubernetes)                   ║
║                                                                     ║
║  ┌─────────────┐  ┌──────────────┐  ┌───────────┐  ┌───────────┐  ║
║  │  Auth Svc   │  │ Message Svc  │  │ Group Svc │  │ Media Svc │  ║
║  │  (3 pods)   │  │  (5 pods)    │  │ (3 pods)  │  │ (3 pods)  │  ║
║  │  Port 8080  │  │  Port 8081   │  │ Port 8082 │  │ Port 8083 │  ║
║  └──────┬──────┘  └──────┬───────┘  └─────┬─────┘  └─────┬─────┘  ║
║         │                │                │               │        ║
╠═════════╪════════════════╪════════════════╪═══════════════╪════════╣
║         │         MESSAGE BUS (NATS)       │               │        ║
╠═════════╪════════════════╪════════════════╪═══════════════╪════════╣
║                        DATA LAYER                                   ║
║                                                                     ║
║  ┌──────────────┐  ┌──────────────┐  ┌──────────────────────────┐  ║
║  │  ScyllaDB    │  │ Redis Cluster│  │      MinIO / S3          │  ║
║  │  (3-node     │  │  (3 nodes)   │  │  (Encrypted Media Blobs) │  ║
║  │  cluster)    │  │              │  │                          │  ║
║  └──────────────┘  └──────────────┘  └──────────────────────────┘  ║
╠═════════════════════════════════════════════════════════════════════╣
║                      OBSERVABILITY                                  ║
║       OpenTelemetry → Grafana + Prometheus + Loki (no PII)         ║
╚═════════════════════════════════════════════════════════════════════╝
```

---

## 2. Service Descriptions

### 2.1 Auth Service
**Responsibility:** Account registration, login, JWT issuance  
**Scaling:** 3 replicas minimum, CPU-based autoscaling  
**State:** Stateless — all state in ScyllaDB + Redis  
**Rust crate:** `ghostlink-api` (auth handlers only)

**Key operations:**
- `POST /auth/register` → Argon2id hash, store account, return JWT
- `POST /auth/login` → Verify hash, return JWT
- JWT validation middleware used by all services

### 2.2 Message Service
**Responsibility:** WebSocket connections, message routing, offline queue  
**Scaling:** 5 replicas minimum (most resource-intensive)  
**State:** In-memory connection registry (DashMap), sync via NATS  
**Rust crate:** `ghostlink-ws` + `ghostlink-api` (message handlers)

**Key operations:**
- WebSocket connection management
- Real-time message routing (online → direct WebSocket delivery)
- Offline queue management (ScyllaDB TTL-based)
- Typing indicator relay

### 2.3 Group Service
**Responsibility:** Group CRUD, membership management, invite links  
**Scaling:** 3 replicas  
**State:** Stateless  

### 2.4 Media Service
**Responsibility:** Encrypted media upload, download, TTL cleanup  
**Scaling:** 3 replicas; can scale based on upload throughput  
**State:** Reference data in ScyllaDB, blobs in S3/MinIO  

### 2.5 Push Service
**Responsibility:** Dispatch push notifications to APNs/FCM  
**Scaling:** 2 replicas (lower load)  
**State:** Stateless — triggered by NATS events from Message Service  

---

## 3. Data Flow: Message Delivery

### 3.1 Online Message Delivery

```
Sender (Alice)
  │
  │  1. Encrypt message with Signal Protocol (Double Ratchet)
  │  2. Send encrypted blob over WebSocket
  ▼
Message Service (WebSocket connection for Alice)
  │
  │  3. Validate JWT, verify contact relationship
  │  4. Look up Bob's connection in hub (DashMap)
  │
  ├── Bob is ONLINE? ──YES──► Bob's Message Service pod
  │                               │
  │                               │  5. Send via Bob's WebSocket
  │                               ▼
  │                           Bob's Device
  │                               │
  │                               │  6. Decrypt with Signal Protocol
  │                               │  7. Display message
  │                               │  8. Send "delivered" ack via WS
  │                               ▼
  │                           Message Service → Delivery Receipt
  │                               │
  │                               ▼
  │                           Alice's Device ← "✓✓"
  │
  └── Bob is OFFLINE? ─YES──► Store in offline_queue (ScyllaDB, 7d TTL)
                               │
                               │  Also: Dispatch to Push Service via NATS
                               ▼
                           Push Service
                               │
                               │  5. Send silent push to Bob's device
                               │     Content: {"type": "NEW_MESSAGE"} only
                               ▼
                           Bob's Device Wakes
                               │
                               │  6. Authenticate WebSocket
                               │  7. Fetch offline queue
                               │  8. Decrypt messages locally
                               │  9. ACK to server (clear offline queue)
```

### 3.2 Multi-Device Delivery

```
Sender (Alice, Device A)
  │
  ▼
Message Service
  │
  ├── Route to Bob's Device A (connected)
  ├── Route to Bob's Device B (if connected)
  └── Queue in offline_queue for Bob's Device C (if offline)
      └── Each device maintains separate Signal Protocol session
```

---

## 4. Horizontal Scaling Strategy

### 4.1 WebSocket Scaling Problem

WebSocket connections are stateful — a client's connection is pinned to one pod. This means messages for Alice (connected to Pod 1) cannot be delivered by Pod 3.

**Solution: NATS Message Bus**

```
Alice (Pod 1) ──── sends message to Bob ────────────────────────────────────┐
                                                                            │
Message Service                                                             │
  Pod 1 (has Alice's WS)                                                   │
    │                                                                       │
    │  Check local DashMap: Is Bob here? NO                                │
    │                                                                       │
    │  Publish to NATS: channel "user.{bob_account_id}"                    │
    │  Payload: encrypted_message_blob                                       │
    │                                                                       ▼
                                                              NATS Server
                                                                   │
                                                 Subscribed pods receive event
                                                                   │
                                              Message Service Pod 3 (has Bob's WS)
                                                    │
                                                    │  Found Bob in local DashMap
                                                    │
                                                    ▼
                                               Bob's Device
```

Each Message Service pod subscribes to `user.*` subjects on NATS. When a message for Bob arrives on any pod, that pod checks its local connection map. If Bob is there, deliver directly. This gives us horizontal scale without cross-pod HTTP calls.

---

## 5. Database Scaling

### 5.1 ScyllaDB Cluster

**Configuration:**
- 3 nodes minimum (RF=3 for production)
- 1 additional node per 50,000 DAU
- NVMe SSDs required (ScyllaDB is latency-sensitive)
- `NetworkTopologyStrategy` replication across availability zones

**Partition Key Strategy:**
Messages are partitioned by `conversation_id`. This ensures all messages in a conversation are on the same shard, making history queries O(1) lookups.

**Problem:** Hot partitions for popular groups  
**Solution:** Group messages use a composite partition key: `(group_id, bucket)` where bucket = `floor(created_at / 86400)` — one partition per group per day.

### 5.2 Redis Cluster

```
Redis Cluster (3 master + 3 replica nodes)

Slot 0-5460   → Master 1 + Replica 1a
Slot 5461-10922 → Master 2 + Replica 2a  
Slot 10923-16383 → Master 3 + Replica 3a

Data distributed by key hash slot:
  session:* → hashed to appropriate master
  presence:* → hashed to appropriate master
  ratelimit:* → hashed to appropriate master
```

---

## 6. Security Architecture

### 6.1 Network Security

```
Internet
   │
   ▼
Cloudflare (DDoS protection, TLS)
   │
   ▼
Nginx (TLS termination, rate limiting)
   │  Only port 443 exposed externally
   ▼
Kubernetes Ingress
   │
   ▼
Service Mesh (mTLS between all pods — Istio or Linkerd)
   │
   ▼
Application Pods (no direct internet access)
   │
   ▼
Database Pods (separate network namespace, no external access)
```

**Network Policies:**
- No pod can reach the internet directly (egress denied except NATS internal + push provider APIs)
- No pod can reach another pod's network except via defined service ports
- Database pods accept connections only from app pods in the same namespace

### 6.2 Secret Management

```
HashiCorp Vault (production)
   │
   ├── Database credentials (rotated weekly)
   ├── JWT signing secrets
   ├── APNs private key
   ├── FCM server key
   └── S3 credentials

Vault Agent Sidecar → injects secrets as env vars into pods
Kubernetes Secrets → encrypted at rest (KMS-backed)
```

---

## 7. Observability (Privacy-Preserving)

### 7.1 Metrics (Prometheus)

Metrics we collect:
```
ghostlink_http_requests_total{method, path, status}
ghostlink_http_request_duration_seconds{method, path, quantile}
ghostlink_ws_connections_active
ghostlink_messages_sent_total{type}
ghostlink_messages_queued_total
ghostlink_media_uploads_total{media_type}
ghostlink_db_query_duration_seconds{query_type}
ghostlink_cache_hit_ratio
ghostlink_push_notifications_sent_total{platform, status}
```

Metrics we DO NOT collect:
- Per-user metrics
- Per-conversation metrics
- Message content or size histograms (would reveal usage patterns)

### 7.2 Traces (OpenTelemetry)

Span attributes:
```
http.method
http.route (path pattern only, e.g. /messages/{id} — not actual ID)
http.status_code
db.system
db.operation
rpc.service
```

Span attributes we NEVER include:
- `user.id` or any account identifier
- `http.url` (contains query params which may include token)
- Message content
- Username

---

## 8. Disaster Recovery

### 8.1 Backup Strategy

| Data | Backup Frequency | Retention | Method |
|------|-----------------|-----------|--------|
| ScyllaDB (account + key data) | Hourly snapshot | 7 days | ScyllaDB Manager |
| Redis | RDB snapshot every 5 min | 24 hours | Redis BGSAVE |
| Media blobs | Not backed up (E2EE + ephemeral) | 30 days then deleted | — |

**Why no media backup:** Media is encrypted on the client side. Backing it up provides no value (cannot decrypt) and creates unnecessary data retention.

### 8.2 Recovery Time Objectives

| Scenario | RTO | RPO |
|----------|-----|-----|
| Single pod failure | 30 seconds (K8s restarts) | 0 (stateless) |
| Single ScyllaDB node failure | 0 (RF=3, survives 1 failure) | 0 |
| Full region outage | 15 minutes (DNS failover) | < 1 hour |
| Full data corruption | 2 hours | < 1 hour |

---

## 9. Infrastructure as Code

All infrastructure defined in Terraform:

```
terraform/
├── modules/
│   ├── gke-cluster/          # Kubernetes cluster
│   ├── scylladb/             # ScyllaDB cluster
│   ├── redis/                # Redis cluster  
│   ├── load-balancer/        # GLB configuration
│   ├── storage/              # S3/GCS buckets
│   └── monitoring/           # Observability stack
├── environments/
│   ├── staging/
│   └── production/
└── main.tf
```

---

*End of Architecture Document v1.0*
