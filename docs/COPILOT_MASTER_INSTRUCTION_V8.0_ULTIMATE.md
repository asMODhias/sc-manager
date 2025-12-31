---
title: COPILOT_MASTER_INSTRUCTION_V8.0_ULTIMATE
version: 8.0.0-ULTIMATE
date: 2025-12-30
status: ABSOLUTE_BINDING_FINAL
priority: SUPREME
applies_to: ALL_CODE_GENERATION
---

# 🏛️ COPILOT MASTER INSTRUCTION V8.0 ULTIMATE

> **SUPERSSEDED BY SOT:** The definitive Source of Truth for project policy and execution is `SC_MANAGER_V8.0.3_COMPLETE_ULTIMATE_CORRECTION.md` in this `docs/` directory. **Follow the SOT exclusively.**

**Enterprise-Grade | Network-First | Zero-Deviation | Production-Ready**

---

## 📐 ARCHITECTURAL PHILOSOPHY

### The Three Pillars

```yaml
1. LOCAL_FIRST:
   - Data originates on device
   - No cloud dependency
   - Offline-capable by design
   - Privacy by architecture

2. MESH_NATIVE:
   - P2P is primary distribution
   - No single point of failure
   - Self-healing network
   - Automatic load balancing

3. AUTHORITY_BOUNDED:
   - Master Server = Trust Anchor (not Controller)
   - Authorizes, never manipulates
   - Cryptographic verification
   - Audit trail immutable
```

---

## 🌐 PART 1: NETWORK ARCHITECTURE (FOUNDATION)

### 1.1 The Three-Layer Network

```yaml
Layer_1_AUTHOR_MASTER:
  Role: Global Authority & Audit Anchor
  Location: Author-controlled server
  Connectivity: HTTPS + mTLS
  
  Responsibilities:
    ✅ Sign updates (Ed25519)
    ✅ Register plugins (marketplace)
    ✅ Collect audit events (append-only)
    ✅ Publish update manifests (read-only)
    ✅ Zero-knowledge statistics (anonymized)
  
  NOT_Responsible:
    ❌ Content distribution
    ❌ User data storage
    ❌ Real-time routing
    ❌ Gameplay interaction
  
  Data_Model:
    Storage: PostgreSQL (relational) + Redis (cache)
    Backup: Daily encrypted backups
    Retention: Audit events (7 years), Updates (indefinite)
    Size: ~100GB for 100k users

Layer_2_MINI_MASTER_MESH:
  Role: Distributed Validation & Sync Network
  Location: Every installation
  Connectivity: QUIC (P2P) + mDNS (LAN)
  
  Responsibilities:
    ✅ CRDT state synchronization
    ✅ Hash-gossip protocol
    ✅ Local validation (updates, plugins)
    ✅ Peer discovery (DHT + mDNS)
    ✅ Content distribution (chunks)
    ✅ Consensus participation
  
  Topology:
    Type: Kademlia DHT (distributed hash table)
    Routing: XOR distance metric
    Replication: k=20 (20 closest peers)
    Refresh: Every 10 minutes
  
  Protocols:
    Discovery: libp2p-kad (Kademlia)
    Transport: QUIC (UDP-based, encrypted)
    Sync: Gossipsub (efficient multicast)
    State: Automerge (CRDT)
  
  Performance:
    Latency: <50ms (LAN), <500ms (WAN)
    Throughput: 10MB/s per peer
    Connections: 20-50 active peers
    Memory: <50MB for mesh state

Layer_3_LOCAL_CLIENT:
  Role: User Interface & Local State
  Location: End-user device
  Connectivity: Local IPC + HTTP (localhost)
  
  Responsibilities:
    ✅ UI rendering (Tauri + SolidJS)
    ✅ Local state (RocksDB)
    ✅ Business logic (Rust actors)
    ✅ Event sourcing (local store)
    ✅ Plugin execution (WASM sandbox)
  
  Resources:
    Memory: <150MB (idle), <500MB (active)
    CPU: <1% (idle), <10% (active)
    Disk: ~1GB (install) + ~5GB (data per org)
    Network: <1MB/hour (background sync)
```

### 1.2 Network Flow Patterns

```yaml
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# PATTERN 1: UPDATE DISTRIBUTION
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Step_1_Author_Signs:
  Author → Master Server:
    - Upload new build
    - Generate manifest
    - Sign with Ed25519 key
    - Publish metadata
  
  Master Server Action:
    - Verify author signature
    - Store manifest (immutable)
    - Generate CID (content hash)
    - Publish to /api/v1/updates/latest

Step_2_Client_Detection:
  Local Client → Master Server:
    - Poll every 6 hours (or manual)
    - GET /api/v1/updates/latest
    - Compare version
    - Verify signature
  
  Decision:
    IF new_version > current_version:
      IF required: Notify user (must update)
      ELSE: Notify user (optional update)

Step_3_P2P_Download:
  Local Client → P2P Mesh:
    - Request chunks by CID
    - Connect to 5-10 peers
    - Download in parallel
    - Verify each chunk (SHA3-256)
  
  Peer Selection:
    Priority_1: LAN peers (mDNS)
    Priority_2: Closest peers (XOR distance)
    Priority_3: High-bandwidth peers
  
  Chunking:
    Size: 1MB per chunk
    Parallelism: 10 concurrent
    Retry: 3 attempts per chunk
    Timeout: 30s per chunk

Step_4_Apply_Update:
  Local Client:
    - Create backup (current version)
    - Verify all chunks
    - Apply delta patches
    - Replace files
    - Write update marker
    - Report success to Master
  
  Rollback_Triggers:
    - Crash on startup
    - Plugin load failure >50%
    - Critical error rate >10%
    - Timeout (5 minutes)

# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# PATTERN 2: ORGANIZATION STATE SYNC
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Step_1_Local_Change:
  User Action:
    - Create operation
    - Add member
    - Update fleet
  
  Local Client:
    - Apply to CRDT document
    - Generate event (domain event)
    - Store in local event log
    - Compute new state hash

Step_2_Hash_Broadcast:
  Local Client → P2P Mesh:
    - Broadcast new hash (Gossipsub)
    - Topic: "scm/org/{org_id}/state"
    - Frequency: Every 10 seconds
    - Peers receive hash

Step_3_Hash_Comparison:
  Receiving Peer:
    - Compare received hash with local hash
    
    IF hashes_match:
      - No action (already in sync)
    
    IF hashes_differ:
      - Request sync message
      - Apply CRDT merge
      - Recompute hash
      - Broadcast new hash

Step_4_CRDT_Merge:
  Algorithm: Automerge
  Conflict_Resolution: Automatic (last-write-wins with vector clocks)
  
  Example:
    User_A: Sets operation.name = "Alpha Strike"
    User_B: Sets operation.name = "Beta Raid"
    
    Result: Depends on vector clock
      IF A_timestamp > B_timestamp: "Alpha Strike"
      IF B_timestamp > A_timestamp: "Beta Raid"
      IF timestamps equal: Lexicographic (deterministic)

# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# PATTERN 3: AUDIT EVENT REPORTING
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Step_1_Event_Generation:
  Trigger:
    - Error occurs
    - Plugin crashes
    - ToS violation detected
    - Security event
    - User action (high-level)
  
  Local Client:
    - Create AuditEvent
    - Anonymize (hash user ID)
    - Add context
    - Store locally (queue)

Step_2_Batch_Upload:
  Frequency: Every 5 minutes OR 100 events
  
  Local Client → Master Server:
    - POST /api/v1/audit/events
    - Batch of events (JSON array)
    - Encrypted (TLS 1.3)
    - Compressed (gzip)
  
  Master Server:
    - Verify source hash
    - Append to audit ledger
    - Update Merkle chain
    - Store in PostgreSQL

Step_3_Aggregation:
  Master Server (background job):
    - Aggregate events (hourly)
    - Compute statistics
    - Detect anomalies
    - Generate alerts (if needed)
  
  Dashboards:
    - Real-time: Last 24 hours
    - Historical: 7 days, 30 days, 1 year
    - Metrics: Error rate, crash rate, active users, feature usage

# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# PATTERN 4: PLUGIN DISTRIBUTION
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Step_1_Plugin_Registration:
  Author → Master Server:
    - Upload WASM binary
    - Submit metadata (permissions, version, etc.)
    - Sign with author key
  
  Master Server:
    - Security scan (static analysis)
    - ToS compliance check
    - Generate CID
    - Publish to marketplace index

Step_2_Plugin_Discovery:
  Local Client → Master Server:
    - GET /api/v1/marketplace/plugins
    - Filter by category, rating, etc.
    - Display in UI

Step_3_Plugin_Installation:
  User clicks "Install"
  
  Local Client:
    - Request plugin by CID
    - Download via P2P mesh
    - Verify signature
    - Extract WASM
    - Request permissions (user approval)
    - Load into sandbox
    - Execute onLoad()

Step_4_Plugin_Update:
  Same as program update but per-plugin
  
  Auto-Update:
    - Check every 24 hours
    - Download if available
    - User approval (optional, configurable)
    - Apply update
    - Reload plugin (hot-reload)
```

### 1.3 Network Resilience & Fault Tolerance

```yaml
Failure_Scenarios:

1. Master_Server_Offline:
   Impact: Cannot check for updates, cannot register new plugins
   Mitigation:
     - Clients continue operating (offline-first)
     - P2P mesh continues syncing
     - Updates can wait (not critical)
     - Plugins continue working
   
   Recovery:
     - Master comes back online
     - Clients reconnect automatically
     - Queue of audit events uploaded
     - Update check resumes

2. Network_Partition:
   Impact: Mesh splits into 2+ groups
   Mitigation:
     - Each partition operates independently
     - CRDT ensures eventual consistency
     - No data loss
   
   Recovery:
     - Partitions reconnect
     - CRDT merge automatically
     - Conflicts resolved (vector clocks)
     - State converges

3. Peer_Churn (peers joining/leaving):
   Impact: Topology changes frequently
   Mitigation:
     - DHT routing table updates
     - Replication factor (k=20)
     - Content re-distributed
   
   Performance:
     - <100ms to detect peer leave
     - <1s to find replacement peers
     - <10s to re-replicate content

4. Byzantine_Peers (malicious):
   Impact: Attempt to distribute fake updates or poison state
   Mitigation:
     - Signature verification (all content)
     - Majority consensus (for state)
     - Ban list (distributed via mesh)
     - Reputation system (planned V8.2)
   
   Detection:
     - Hash mismatch
     - Invalid signature
     - Timeout/unresponsive
     - Reported by peers

5. DDoS_Attack (on Master Server):
   Impact: Master Server overwhelmed
   Mitigation:
     - Rate limiting (per IP)
     - CDN (Cloudflare) in front
     - API key required (registered clients)
     - Fallback to cached data
   
   Client_Behavior:
     - Exponential backoff
     - Switch to P2P-only mode
     - Continue operating offline
```

---

## Post-merge: CIG Fankit import (2025-12-30)

- **Merged** via **admin override** on 2025-12-30 due to exhausted GitHub Actions quota (PR #27). ⚠️
- **Local Quick-Tests and Smoke-Run** completed successfully prior to merge.
- **Tag & Release:** `v8.0.0-fankit` created and GitHub release published.
- **License/TOS:** `assets/fankit/LICENSE_CIG_FANKIT.txt` included and confirmed by the repository owner.
- **Governance note:** Admin overrides must be documented and are exceptional; re-enable external CI quota as soon as possible and follow up with a re-run of remote checks where feasible.

---

## 🏢 PART 2: ENTERPRISE PROGRAM LOGIC

### 2.1 Multi-Tenant Architecture

```yaml
Isolation_Model:
  Tenant: Organization (Org)
  
  Data_Isolation:
    Database:
      - Each org has unique prefix: "org_{org_id}_"
      - RocksDB column families per org
      - No cross-org queries possible
    
    CRDT_Documents:
      - Separate Automerge doc per org
      - No shared mutable state
      - Read-only references (member lookup)
    
    P2P_Topics:
      - Each org has dedicated topic: "scm/org/{org_id}/*"
      - Members subscribe only to their org topics
      - No cross-org message leakage
  
  Resource_Limits:
    Per_Organization:
      Memory: 50MB
      CPU: 5% (average)
      Disk: 5GB
      Events: 10,000/hour
      API_Calls: 1,000/hour
    
    Enforcement:
      - Resource monitor (background task)
      - Graceful degradation (throttle, not block)
      - Alerts (if limits exceeded)
      - Auto-scaling (authority mode)

Service_Levels:
  Community (Free):
    Members: 1,000
    Operations: 10,000
    Storage: 5GB
    Support: Community forum
  
  Pro ($99/month):
    Members: 10,000
    Operations: 100,000
    Storage: 50GB
    Support: Email (8-hour response)
    SLA: 99.5% uptime
  
  Enterprise (Custom):
    Members: Unlimited
    Operations: Unlimited
    Storage: Unlimited
    Support: Dedicated (1-hour response)
    SLA: 99.9% uptime
    Features: SSO, advanced analytics, custom integrations
```

### 2.2 Actor Model Implementation

```yaml
Actor_System:
  Framework: Tokio (async actors via channels)
  
  Core_Actors:
    - OrganizationActor (1 per org)
    - MemberActor (1 per member)
    - OperationActor (1 per operation)
    - FleetActor (1 per fleet)
    - DiplomacyActor (1 per org)
    - P2PNodeActor (1 per installation)
    - UpdateClientActor (1 per installation)
    - PluginManagerActor (1 per installation)
  
  Message_Flow:
    User_Action → UI_Event → Actor_Message → Actor_Handler → State_Update → CRDT_Merge → P2P_Broadcast
  
  Supervision:
    Strategy: Restart (3 attempts, exponential backoff)
    Isolation: Actor crashes don't affect siblings
    Recovery: Reload state from event log

Example_Actor:
  ```rust
  pub struct OrganizationActor {
      org_id: String,
      state: Organization,
      event_log: EventLog,
      crdt: Arc<CRDTStateManager>,
      inbox: mpsc::UnboundedReceiver<OrgMessage>,
  }
  
  impl OrganizationActor {
      pub async fn run(mut self) {
          while let Some(msg) = self.inbox.recv().await {
              let result = match msg {
                  OrgMessage::AddMember(cmd) => {
                      self.handle_add_member(cmd).await
                  }
                  OrgMessage::RemoveMember(cmd) => {
                      self.handle_remove_member(cmd).await
                  }
                  OrgMessage::UpdateSettings(cmd) => {
                      self.handle_update_settings(cmd).await
                  }
                  // ... more handlers
              };
              
              match result {
                  Ok(event) => {
                      // Persist event
                      self.event_log.append(event.clone()).await?;
                      
                      // Update CRDT
                      self.crdt.apply_event(&event).await?;
                      
                      // Broadcast hash
                      self.broadcast_state_hash().await?;
                  }
                  Err(e) => {
                      error!(org_id = %self.org_id, error = %e, "Command failed");
                  }
              }
          }
      }
  }
  ```

Concurrency_Model:
  - Each actor processes messages sequentially
  - No locks needed within actor
  - Message passing between actors
  - Backpressure via bounded channels
  - Supervision for fault tolerance
```

### 2.3 Event Sourcing & CQRS

```yaml
Event_Sourcing:
  Principle: State = Aggregate(Events)
  
  Event_Store:
    Storage: RocksDB (local) + PostgreSQL (authority mode)
    Format: Protobuf (binary, efficient)
    Ordering: Lamport timestamps + causal order
    Retention: Indefinite (compaction after 1 year)
  
  Event_Types:
    - OrganizationCreated
    - MemberAdded
    - MemberRemoved
    - MemberRoleChanged
    - OperationPlanned
    - OperationStarted
    - OperationCompleted
    - FleetDeployed
    - DiplomaticRelationEstablished
    - ... (50+ event types)
  
  Replay:
    Purpose: Rebuild state from events
    Use_Cases:
      - Disaster recovery
      - Audit trail
      - Time travel debugging
      - Migration to new version
    
    Performance:
      - 100,000 events/second (replay)
      - Snapshots every 1,000 events
      - Incremental replay from snapshot

CQRS (Command-Query Responsibility Segregation):
  Commands (Write):
    Path: UI → Command → Actor → Event → Event Store
    Validation: In aggregate (domain logic)
    Authorization: RBAC + permissions
    Idempotency: Command ID (deduplicate)
  
  Queries (Read):
    Path: UI → Query → Read Model → Response
    Source: CRDT state (eventually consistent)
    Caching: Aggressive (1-minute TTL)
    Projections: Pre-computed (background)
  
  Example_Command:
    ```rust
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AddMemberCommand {
        pub command_id: Uuid,
        pub org_id: String,
        pub rsi_handle: String,
        pub rank: Rank,
        pub roles: Vec<Role>,
        pub requester_id: String,
    }
    
    impl AddMemberCommand {
        pub fn validate(&self) -> Result<(), CommandError> {
            // Validation logic
            if self.rsi_handle.is_empty() {
                return Err(CommandError::InvalidHandle);
            }
            
            if self.rsi_handle.len() > 50 {
                return Err(CommandError::HandleTooLong);
            }
            
            // ... more validation
            
            Ok(())
        }
    }
    ```
  
  Example_Query:
    ```rust
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct GetOrganizationQuery {
        pub org_id: String,
        pub requester_id: String,
    }
    
    impl GetOrganizationQuery {
        pub async fn execute(
            &self,
            crdt: &CRDTStateManager,
        ) -> Result<Organization, QueryError> {
            // Read from CRDT (eventually consistent)
            let state = crdt.get_org(&self.org_id).await?;
            
            Ok(state)
        }
    }
    ```

Eventual_Consistency:
  Acceptable_Lag: <1 second (typical), <10 seconds (worst case)
  
  User_Feedback:
    - Optimistic updates (UI updates immediately)
    - Conflict notification (if CRDT merge changes result)
    - Sync indicator (in UI)
  
  Conflict_Resolution:
    Strategy: Last-write-wins (LWW) with vector clocks
    
    Example:
      Officer_A: Sets operation.start_time = 20:00
      Officer_B: Sets operation.start_time = 21:00
      
      IF B.timestamp > A.timestamp:
        Result: 21:00 (B wins)
      
      UI shows:
        "Operation start time updated by Officer B (21:00)"
```

### 2.4 Business Logic Patterns

```yaml
Domain_Driven_Design:
  Aggregates:
    - Organization (root)
    - Member (entity)
    - Operation (aggregate root)
    - Fleet (aggregate root)
    - DiplomaticRelation (aggregate root)
  
  Value_Objects:
    - RSIHandle
    - Rank
    - Role
    - Qualification
    - Ship
    - Location
    - TimeWindow
  
  Domain_Services:
    - OperationPlanner
    - FleetOptimizer
    - DiplomacyNegotiator
    - MissionVerifier
  
  Repository_Pattern:
    - Trait definition in domain
    - Implementation in adapter
    - Async methods (tokio)

Example_Aggregate:
  ```rust
  pub struct Organization {
      id: String,
      name: String,
      members: HashMap<String, Member>,
      settings: OrgSettings,
      events: Vec<DomainEvent>,
  }
  
  impl Organization {
      pub fn add_member(
          &mut self,
          handle: RSIHandle,
          rank: Rank,
      ) -> Result<(), OrgError> {
          // Business rules
          if self.members.contains_key(handle.as_str()) {
              return Err(OrgError::MemberAlreadyExists);
          }
          
          if self.members.len() >= self.settings.max_members {
              return Err(OrgError::MemberLimitReached);
          }
          
          // Create event
          let event = DomainEvent::MemberAdded {
              org_id: self.id.clone(),
              handle: handle.clone(),
              rank,
              timestamp: Utc::now(),
          };
          
          // Apply event
          self.apply(event.clone());
          
          // Store for persistence
          self.events.push(event);
          
          Ok(())
      }
      
      fn apply(&mut self, event: DomainEvent) {
          match event {
              DomainEvent::MemberAdded { handle, rank, .. } => {
                  let member = Member::new(handle.clone(), rank);
                  self.members.insert(handle.to_string(), member);
              }
              // ... other event handlers
          }
      }
  }
  ```

Validation_Strategy:
  Layers:
    1. Input_Validation (UI layer)
       - Type checking
       - Format validation
       - Length limits
    
    2. Command_Validation (Application layer)
       - Business rules (simple)
       - Authorization check
       - Rate limiting
    
    3. Aggregate_Validation (Domain layer)
       - Complex business rules
       - Invariants
       - Consistency checks
  
  Example:
    Adding member:
      UI: Check handle format (alphanumeric, 3-50 chars)
      Command: Check requester has "AddMember" permission
      Aggregate: Check member doesn't exist, member limit not reached
```

---

## 🔐 PART 3: SECURITY ARCHITECTURE (DEFENSE IN DEPTH)

### 3.1 Security Layers

```yaml
Layer_1_Network:
  Transport_Encryption:
    - TLS 1.3 (Master Server)
    - QUIC with TLS 1.3 (P2P)
    - No plaintext transmission
  
  Certificate_Pinning:
    - Master Server certificate pinned
    - Prevents MITM attacks
    - Rotation every 90 days
  
  mTLS_P2P:
    - Mutual authentication
    - Each peer has certificate
    - Certificate = Ed25519 public key + signature
  
  DDoS_Protection:
    - Rate limiting (application level)
    - Connection limits (per IP)
    - CDN (Cloudflare) for Master Server

Layer_2_Authentication:
  RSI_OAuth:
    - Primary identity (Star Citizen account)
    - PKCE flow (public client)
    - Scope: openid, profile, org:read
    - Token refresh automatic
    - Token storage encrypted
  
  Local_Keypair:
    - Ed25519 keypair (per installation)
    - Hardware-bound (optional, TPM/YubiKey)
    - Used for P2P authentication
    - Never transmitted
  
  Multi_Factor (Optional):
    - TOTP (Time-based One-Time Password)
    - Hardware key (YubiKey, etc.)
    - Biometric (Windows Hello)

Layer_3_Authorization:
  RBAC (Role-Based Access Control):
    Roles:
      - Founder (all permissions)
      - Officer (management permissions)
      - Member (basic permissions)
      - Affiliate (read-only)
      - Recruit (limited)
    
    Permissions:
      - CreateOrganization
      - AddMember, RemoveMember
      - AssignRole, ChangeRank
      - PlanOperation, StartOperation
      - DeployFleet, RecallFleet
      - EstablishDiplomacy
      - ViewFinances, ManageFinances
      - ... (50+ permissions)
    
    Inheritance:
      Founder → Officer → Member → Affiliate → Recruit
  
  Attribute_Based (Future V8.2):
    - Context-aware (location, time)
    - Dynamic permissions
    - Risk-based (suspicious activity)

Layer_4_Data_Protection:
  Encryption_At_Rest:
    - AES-256-GCM
    - Per-user key (derived from password or hardware key)
    - Key rotation every 90 days
    - Transparent (application handles)
  
  Encryption_In_Transit:
    - TLS 1.3 (Master Server)
    - QUIC (P2P)
    - End-to-end for sensitive data
  
  Zero_Knowledge:
    - Master Server never sees:
      - User passwords
      - Organization secrets
      - Private messages
      - Sensitive files
    
    - Master Server only sees:
      - Hashed identifiers
      - Anonymized statistics
      - Audit events (no PII)

Layer_5_Application:
  Input_Validation:
    - All inputs sanitized
    - SQL injection prevention (parameterized queries)
    - XSS prevention (escaped output)
    - Path traversal prevention (validate paths)
  
  CSRF_Protection:
    - Token-based (for web API)
    - Origin validation
    - SameSite cookies
  
  Rate_Limiting:
    - Per user: 100 requests/minute
    - Per IP: 1000 requests/minute
    - Sliding window algorithm
  
  Audit_Logging:
    - All security events logged
    - Immutable audit trail
    - Alert on suspicious activity

Layer_6_Plugin_Sandbox:
  WASM_Isolation:
    - No file system access (default)
    - No network access (default)
    - No system calls
    - Memory limit (50MB)
    - CPU limit (1s per event)
  
  Capability_Based:
    - Explicit permission grants
    - Fine-grained control
    - User approval required
  
  Code_Signing:
    - All plugins signed
    - Signature verification
    - Revocation list (distributed)
```

### 3.2 Threat Model

```yaml
Threats:

T1_Network_Eavesdropping:
  Risk: HIGH
  Mitigation:
    - TLS 1.3 everywhere
    - No plaintext transmission
    - Certificate pinning
  Status: MITIGATED

T2_Man_In_The_Middle:
  Risk: HIGH
  Mitigation:
    - Certificate pinning (Master)
    - mTLS (P2P)
    - Signature verification (updates)
  Status: MITIGATED

T3_Malicious_Update:
  Risk: CRITICAL
  Mitigation:
    - Ed25519 signature (author key)
    - Hash verification (SHA3-512)
    - Rollback on failure
    - Multiple verification points
  Status: MITIGATED

T4_Plugin_Sandbox_Escape:
  Risk: HIGH
  Mitigation:
    - WASM sandbox (memory-safe)
    - Capability-based permissions
    - Resource limits
    - Security scan (static analysis)
  Status: MITIGATED

T5_State_Poisoning:
  Risk: MEDIUM
  Mitigation:
    - CRDT conflict resolution
    - Majority consensus
    - Ban list (malicious peers)
    - Hash verification
  Status: MITIGATED

T6_Denial_Of_Service:
  Risk: MEDIUM
  Mitigation:
    - Rate limiting
    - Connection limits
    - CDN (Cloudflare)
    - P2P fallback
  Status: MITIGATED

T7_Data_Breach:
  Risk: HIGH
  Mitigation:
    - Encryption at rest (AES-256-GCM)
    - Encryption in transit (TLS 1.3)
    - Zero-knowledge architecture
    - No PII in Master Server
  Status: MITIGATED

T8_Account_Takeover:
  Risk: HIGH
  Mitigation:
    - MFA (optional but recommended)
    - Session management (timeouts)
    - Suspicious activity detection
    - Password strength enforcement (via RSI)
  Status: PARTIALLY_MITIGATED
  Recommendation: Enforce MFA for officers in V8.1

T9_Social_Engineering:
  Risk: MEDIUM
  Mitigation:
    - User education
    - Verification prompts (high-risk actions)
    - Audit trail (detect fraud)
  Status: PARTIALLY_MITIGATED
  Recommendation: Add verification challenges for sensitive operations

T10_Supply_Chain_Attack:
  Risk: HIGH
  Mitigation:
    - Dependency audit (cargo audit)
    - License check (cargo deny)
    - Reproducible builds
    - Code review (all deps)
  Status: MITIGATED
```

---

## 📊 PART 4: ENTERPRISE OBSERVABILITY

### 4.1 OpenTelemetry Integration

```yaml
Instrumentation:
  Traces:
    - Span per function call (critical paths)
    - Attributes: user_id (hashed), org_id, action
    - Sampling: 100% (errors), 10% (normal)
  
  Metrics:
    System:
      - CPU usage (%)
      - Memory usage (MB)
      - Disk I/O (MB/s)
      - Network I/O (MB/s)
    
    Application:
      - Request rate (req/s)
      - Request latency (p50, p95, p99)
      - Error rate (%)
      - Active users
      - Active organizations
    
    P2P:
      - Connected peers
      - Sync latency (ms)
      - Hash mismatches (#)
      - Bandwidth (MB/s)
    
    CRDT:
      - Merge operations (ops/s)
      - Conflict rate (%)
      - Document size (MB)
    
    Database:
      - Query latency (ms)
      - Connection pool usage (%)
      - Transaction rate (tx/s)
  
  Logs:
    - Structured (JSON)
    - Contextual (trace_id, span_id)
    - Levels: ERROR, WARN, INFO, DEBUG, TRACE
    - Centralized (Loki or similar)

Exporters:
  Development:
    - Console (stdout)
    - Jaeger (local)
  
  Production:
    - Prometheus (metrics)
    - Jaeger or Tempo (traces)
    - Loki (logs)
    - Grafana (dashboards)

Dashboards:
  Overview:
    - System health (RED method)
    - Request rate, Error rate, Duration
  
  P2P_Mesh:
    - Network topology (graph)
    - Peer list (table)
    - Bandwidth usage (time series)
  
  Organization:
    - Active orgs (gauge)
    - Members per org (histogram)
    - Operations per org (counter)
  
  Performance:
    - Latency (heatmap)
    - Throughput (time series)
    - Resource usage (stacked area)

Alerting:
  Critical:
    - Error rate >5%
    - Crash on startup
    - Master Server unreachable
    - P2P mesh disconnected
  
  Warning:
    - Error rate >1%
    - Latency p99 >1s
    - Memory usage >80%
    - Disk usage >90%
  
  Channels:
    - PagerDuty (critical)
    - Slack (warning)
    - Email (info)
```

### 4.2 Health Checks

```yaml
Liveness_Probe:
  Endpoint: GET /health/live
  Frequency: Every 10 seconds
  Timeout: 5 seconds
  
  Checks:
    - Process running
    - No deadlocks
    - Memory not exhausted
  
  Response:
    200: Healthy
    500: Unhealthy (restart)

Readiness_Probe:
  Endpoint: GET /health/ready
  Frequency: Every 30 seconds
  Timeout: 10 seconds
  
  Checks:
    - Database connected
    - P2P mesh connected
    - CRDT initialized
    - Plugins loaded
  
  Response:
    200: Ready (accept traffic)
    503: Not ready (wait)

Startup_Probe:
  Endpoint: GET /health/startup
  Frequency: Every 5 seconds
  Timeout: 2 seconds
  Max_Attempts: 60 (5 minutes total)
  
  Checks:
    - Application started
    - Config loaded
    - Database migrated
    - Actors spawned
  
  Response:
    200: Started (switch to readiness probe)
    503: Starting (retry)
```

---

## 🎯 PART 5: COMPREHENSIVE CODE GENERATION RULES

### 5.1 Rust Code Standards

```rust
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// FILE HEADER (EVERY FILE)
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

//! Module name
//!
//! Brief description of what this module does.
//!
//! # Architecture
//! - Layer: Domain/Application/Adapter/Infrastructure
//! - Pattern: Actor/Repository/Service/etc.
//!
//! # Example
//! ```
//! use crate::module::Thing;
//!
//! let thing = Thing::new();
//! ```

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// IMPORTS (ORGANIZED)
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

// Standard library
use std::collections::HashMap;
use std::sync::Arc;

// External crates (alphabetical)
use anyhow::Context;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tracing::{error, info, instrument};

// Internal crates
use crate::domain::Organization;
use crate::events::DomainEvent;

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// TYPE DEFINITIONS
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// Organization aggregate
///
/// Represents a Star Citizen organization with members, operations, fleets.
///
/// # Invariants
/// - Name must be 3-50 characters
/// - Must have at least one founder
/// - Member count must match members.len()
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Organization {
    /// Unique identifier
    pub id: String,
    
    /// Organization name
    pub name: String,
    
    /// Members (keyed by RSI handle)
    pub members: HashMap<String, Member>,
    
    /// Organization settings
    pub settings: OrgSettings,
    
    /// Uncommitted domain events
    #[serde(skip)]
    events: Vec<DomainEvent>,
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// IMPLEMENTATION
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

impl Organization {
    /// Create a new organization
    ///
    /// # Arguments
    /// * `name` - Organization name (3-50 characters)
    ///
    /// # Returns
    /// * `Ok(Organization)` - Created organization
    /// * `Err(OrgError)` - Validation failed
    ///
    /// # Examples
    /// ```
    /// let org = Organization::new("Test Org".to_string())?;
    /// assert_eq!(org.name, "Test Org");
    /// ```
    pub fn new(name: String) -> Result<Self, OrgError> {
        // Validate name
        if name.len() < 3 {
            return Err(OrgError::NameTooShort);
        }
        
        if name.len() > 50 {
            return Err(OrgError::NameTooLong);
        }
        
        // Create organization
        let id = Uuid::new_v4().to_string();
        
        let mut org = Self {
            id: id.clone(),
            name: name.clone(),
            members: HashMap::new(),
            settings: OrgSettings::default(),
            events: Vec::new(),
        };
        
        // Emit creation event
        org.emit(DomainEvent::OrganizationCreated {
            org_id: id,
            name,
            timestamp: Utc::now(),
        });
        
        Ok(org)
    }
    
    /// Add a member to the organization
    ///
    /// # Business Rules
    /// - Member must not already exist
    /// - Member limit must not be exceeded
    /// - Requester must have AddMember permission
    ///
    /// # Errors
    /// Returns error if validation fails or business rules violated
    #[instrument(skip(self))]
    pub fn add_member(
        &mut self,
        handle: String,
        rank: Rank,
    ) -> Result<(), OrgError> {
        info!(org_id = %self.id, handle = %handle, "Adding member");
        
        // Business rule: Member must not exist
        if self.members.contains_key(&handle) {
            return Err(OrgError::MemberAlreadyExists { handle });
        }
        
        // Business rule: Member limit
        if self.members.len() >= self.settings.max_members {
            return Err(OrgError::MemberLimitReached {
                limit: self.settings.max_members,
            });
        }
        
        // Emit event
        self.emit(DomainEvent::MemberAdded {
            org_id: self.id.clone(),
            handle: handle.clone(),
            rank,
            timestamp: Utc::now(),
        });
        
        Ok(())
    }
    
    /// Emit a domain event
    fn emit(&mut self, event: DomainEvent) {
        // Apply event to update state
        self.apply(&event);
        
        // Store event for persistence
        self.events.push(event);
    }
    
    /// Apply event to state
    fn apply(&mut self, event: &DomainEvent) {
        match event {
            DomainEvent::OrganizationCreated { .. } => {
                // Already applied in constructor
            }
            DomainEvent::MemberAdded { handle, rank, .. } => {
                let member = Member::new(handle.clone(), *rank);
                self.members.insert(handle.clone(), member);
            }
            // ... more event handlers
            _ => {}
        }
    }
    
    /// Take uncommitted events
    pub fn take_events(&mut self) -> Vec<DomainEvent> {
        std::mem::take(&mut self.events)
    }
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// ERROR TYPES (THISERROR)
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

#[derive(Debug, thiserror::Error)]
pub enum OrgError {
    #[error("Organization name too short (minimum 3 characters)")]
    NameTooShort,
    
    #[error("Organization name too long (maximum 50 characters)")]
    NameTooLong,
    
    #[error("Member already exists: {handle}")]
    MemberAlreadyExists { handle: String },
    
    #[error("Member limit reached: {limit}")]
    MemberLimitReached { limit: usize },
    
    #[error("Member not found: {handle}")]
    MemberNotFound { handle: String },
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// TESTS (COMPREHENSIVE)
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_organization_success() {
        // Arrange
        let name = "Test Org".to_string();
        
        // Act
        let result = Organization::new(name.clone());
        
        // Assert
        assert!(result.is_ok());
        let org = result.unwrap();
        assert_eq!(org.name, name);
        assert!(!org.id.is_empty());
        assert_eq!(org.members.len(), 0);
        assert_eq!(org.events.len(), 1);
    }
    
    #[test]
    fn test_create_organization_name_too_short() {
        // Arrange
        let name = "AB".to_string();
        
        // Act
        let result = Organization::new(name);
        
        // Assert
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), OrgError::NameTooShort));
    }
    
    #[test]
    fn test_add_member_success() {
        // Arrange
        let mut org = Organization::new("Test Org".to_string()).unwrap();
        let handle = "test_user".to_string();
        
        // Act
        let result = org.add_member(handle.clone(), Rank::Member);
        
        // Assert
        assert!(result.is_ok());
        assert_eq!(org.members.len(), 1);
        assert!(org.members.contains_key(&handle));
    }
    
    #[test]
    fn test_add_member_already_exists() {
        // Arrange
        let mut org = Organization::new("Test Org".to_string()).unwrap();
        let handle = "test_user".to_string();
        org.add_member(handle.clone(), Rank::Member).unwrap();
        
        // Act
        let result = org.add_member(handle.clone(), Rank::Member);
        
        // Assert
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            OrgError::MemberAlreadyExists { .. }
        ));
    }
    
    #[test]
    fn test_event_sourcing() {
        // Arrange
        let mut org = Organization::new("Test Org".to_string()).unwrap();
        
        // Act
        org.add_member("user1".to_string(), Rank::Member).unwrap();
        org.add_member("user2".to_string(), Rank::Officer).unwrap();
        let events = org.take_events();
        
        // Assert
        assert_eq!(events.len(), 3); // Created + 2 members
    }
}
```

### 5.2 Critical Rules Summary

```yaml
NEVER:
  - ❌ Use unwrap() in production code
  - ❌ Use expect() in production code
  - ❌ Use panic!() in production code
  - ❌ Ignore errors with let _ =
  - ❌ Use .ok() to swallow errors
  - ❌ Use unsafe without EXTREMELY good reason
  - ❌ Block async runtime (use spawn_blocking)
  - ❌ Use Arc<Mutex<T>> (prefer RwLock or channels)
  - ❌ Clone unnecessarily
  - ❌ Allocate in hot loops

ALWAYS:
  - ✅ Use Result<T, E> for fallible functions
  - ✅ Use thiserror for domain errors
  - ✅ Use anyhow for application errors
  - ✅ Add context to errors (.context("..."))
  - ✅ Use tracing for logging (not println!)
  - ✅ Use #[instrument] for critical functions
  - ✅ Write comprehensive tests (unit + integration)
  - ✅ Document public APIs (rustdoc)
  - ✅ Validate inputs at boundaries
  - ✅ Use type safety (newtype pattern)

PERFORMANCE:
  - ✅ Profile before optimizing
  - ✅ Use iterators (avoid collect unless needed)
  - ✅ Prefer &str over String
  - ✅ Prefer &[T] over Vec<T>
  - ✅ Use Cow<'_, str> when appropriate
  - ✅ Memory-map large files
  - ✅ Stream large responses
  - ✅ Cache expensive operations
  - ✅ Use rayon for parallel operations
  - ✅ Use tokio for async I/O

SECURITY:
  - ✅ Validate ALL inputs
  - ✅ Sanitize ALL outputs
  - ✅ Use parameterized queries
  - ✅ Encrypt sensitive data
  - ✅ Hash passwords (Argon2)
  - ✅ Use secure random (OsRng)
  - ✅ Rate limit API calls
  - ✅ Audit log security events
  - ✅ Follow principle of least privilege
  - ✅ Check CIG ToS compliance
```

---

## 🚀 PART 6: IMMEDIATE IMPLEMENTATION TASKS

### Task Priority Matrix

```yaml
CRITICAL (Week 1-2):
  TASK-001: Update System ← START HERE
    Priority: P0
    Effort: 3 days
    Blockers: None
    Files:
      - infrastructure/update-system/src/lib.rs ✅ PROVIDED
      - infrastructure/update-system/src/author/mod.rs
      - infrastructure/update-system/src/client/mod.rs
      - infrastructure/update-system/src/delta/mod.rs
      - infrastructure/update-system/src/signature/mod.rs
      - infrastructure/update-system/src/rollback/mod.rs
      - infrastructure/update-system/tests/*.rs
      - infrastructure/update-system/Cargo.toml
      - infrastructure/update-system/README.md
  
  TASK-002: P2P Mesh (CRDT + Gossip)
    Priority: P0
    Effort: 5 days
    Blockers: None (parallel)
    Files:
      - infrastructure/p2p-mesh/src/lib.rs
      - infrastructure/p2p-mesh/src/crdt/mod.rs
      - infrastructure/p2p-mesh/src/gossip/mod.rs
      - infrastructure/p2p-mesh/src/dht/mod.rs
      - infrastructure/p2p-mesh/src/mini_master/mod.rs
      - infrastructure/p2p-mesh/tests/*.rs
  
  TASK-003: Master Server (IN PROGRESS → marketplace persistence implemented)
    Priority: P0
    Effort: 4 days (ongoing)
    Blockers: TASK-001 (signature module)
    Status: API implemented (`/health`, `/api/v1/updates`, `/api/v1/audit/events`, `/api/v1/marketplace/items`); Audit module and Marketplace implemented with append-only NDJSON ledger and snapshot/compaction support. Unit and integration tests added.
    Files:
      - services/master-server/src/lib.rs
      - services/master-server/src/api.rs
      - services/master-server/src/audit/mod.rs
      - services/master-server/src/marketplace/mod.rs
      - services/master-server/src/marketplace/storage.rs
      - services/master-server/tests/*.rs
    Docs:
      - docs/MASTER_SERVER_API.md (API contract, persistence and snapshot notes)

HIGH (Week 3-4):
  TASK-004: Domain Models
    Priority: P1
    Effort: 3 days
    Files:
      - core/domain/src/organization.rs
      - core/domain/src/member.rs
      - core/domain/src/operation.rs
      - core/domain/src/fleet.rs
      - core/domain/src/diplomacy.rs
  
  TASK-005: Event Store
    Priority: P1
    Effort: 2 days
    Files:
      - infrastructure/persistence/src/event_store.rs
      - infrastructure/persistence/src/rocksdb_adapter.rs
  
  TASK-006: Actor System
    Priority: P1
    Effort: 3 days
    Files:
      - core/application/src/actors/organization_actor.rs
      - core/application/src/actors/operation_actor.rs
      - core/application/src/actors/fleet_actor.rs

MEDIUM (Week 5-8):
  TASK-007: RSI Auth Adapter
  TASK-008: Game.log Parser
  TASK-009: Discord Integration
  TASK-010: Plugin SDK v3.0
  TASK-011: Desktop UI (Tauri + SolidJS)
  TASK-012: Installer (WiX v4)

LOW (Week 9-12):
  TASK-013-020: Official Plugins (13 plugins)
  TASK-021: Documentation
  TASK-022: E2E Tests
  TASK-023: Performance Benchmarks
```

---

## 🏁 EXECUTION PROTOCOL

### When I Say "Implement X"

```yaml
Step_1_Analyze:
  - Read this instruction COMPLETELY
  - Identify architecture layer
  - Check dependencies
  - Review related code

Step_2_Plan:
  - Determine file structure
  - List all types needed
  - Identify error cases
  - Plan test cases

Step_3_Generate:
  - Complete file (not snippet)
  - All imports
  - All error handling (Result<T, E>)
  - All tests (unit + integration)
  - All documentation (rustdoc)
  - No TODO, no placeholder

Step_4_Verify:
  - Self-check against rules
  - CIG ToS compliance check
  - Security review
  - Performance considerations

Step_5_Output:
  - File path
  - Complete content
  - Brief explanation (technical)
  - Next recommended task
```

### Output Format

```yaml
# File: infrastructure/update-system/src/author/mod.rs

[COMPLETE FILE CONTENT HERE - NO TRUNCATION]

## Explanation
[Brief technical explanation of what this file does, key design decisions, and how it fits into the architecture]

## Dependencies
[List of dependencies this file has on other modules]

## Next Task
[Recommendation for next file to implement]
```

---

## ✅ FINAL CHECKLIST (BEFORE EVERY CODE GENERATION)

```yaml
Technical:
  - [ ] Specification section read completely?
  - [ ] Architecture layer identified (Domain/Application/Adapter/Infrastructure)?
  - [ ] Design pattern appropriate (Actor/Repository/Service/etc.)?
  - [ ] Tech stack matches specification?
  - [ ] Error handling complete (Result<T, E> everywhere)?
  - [ ] Tests included (unit + integration)?
  - [ ] Documentation complete (rustdoc on all public items)?
  - [ ] Performance considered (profiling points, caching, etc.)?
  - [ ] No unwrap/expect/panic in production code?

Security:
  - [ ] Security rules followed?
  - [ ] Input validation present?
  - [ ] Output sanitization present?
  - [ ] Encryption applied (if sensitive data)?
  - [ ] Authentication checked (if applicable)?
  - [ ] Authorization enforced (if applicable)?
  - [ ] Audit logging added (if security-relevant)?
  - [ ] Rate limiting applied (if API endpoint)?

ToS_Compliance:
  - [ ] CIG ToS specifically checked?
  - [ ] No game manipulation?
  - [ ] No automation?
  - [ ] No unfair advantage?
  - [ ] Manual verification enforced (if game data)?
  - [ ] Read-only access only (if game files)?
  - [ ] Privacy preserved (anonymized data)?

Enterprise:
  - [ ] Multi-tenant isolation enforced?
  - [ ] Resource limits applied?
  - [ ] Observability instrumentation added?
  - [ ] Health checks implemented?
  - [ ] Graceful degradation considered?
  - [ ] Backward compatibility maintained?

Network:
  - [ ] P2P-first approach?
  - [ ] Offline capability preserved?
  - [ ] Fault tolerance considered?
  - [ ] Network partition handling?
  - [ ] Signature verification (if distributed content)?

IF_ANY_UNCHECKED:
  - STOP immediately
  - Review specification section
  - Fix the issue
  - Re-run checklist
  - THEN generate code
```

---

**🎯 COPILOT MASTER INSTRUCTION V8.0 ULTIMATE - COMPLETE**

**This is the definitive, complete, enterprise-grade instruction.**

**Zero questions. Zero deviations. Maximum quality.**

**First Task: Implement TASK-001 (Update System) - All files**

**Status: READY TO EXECUTE** ✅

