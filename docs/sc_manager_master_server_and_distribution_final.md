# SC MANAGER — MASTER SERVER & DISTRIBUTION SPECIFICATION

Document Bundle: 1–5 (BINDING)
Status: ABSOLUTE / ZERO DEVIATION
Compliance: CIG ToS, CORE ToS, PLUGIN ToS, IDC-10, DSGVO, DSA

---

## FILE 1 — MASTER_SERVER_FINAL_SPEC.md

### 1. Purpose
The Master Server is the single cryptographic and legal authority of the SC Manager ecosystem. It does not distribute binaries directly. It authorizes, signs, audits, and records immutable events.

### 2. Responsibilities
- Authoritative signing of:
  - Core updates
  - Plugin marketplace entries
  - Theme & language packs
- Immutable audit ledger
- Compliance & ToS violation registry
- Global metadata distribution (read-only)

### 3. Non-Responsibilities
- No gameplay interaction
- No automation
- No real-time player tracking
- No P2P routing

### 4. Data Model (Immutable)
```yaml
Stored:
  - InstanceHash
  - UserHash (salted)
  - OrgHash
  - EventType
  - Timestamp
  - ContentHash
  - GeoRegion (ISO-3166)

Forbidden:
  - IP addresses
  - RSI credentials
  - Cleartext identifiers
```

### 5. Author Access
- Offline-generated Ed25519 root key
- Public key registered once
- Private key NEVER stored

---

## FILE 2 — PLUGIN_MARKETPLACE_PROTOCOL.md

### 1. Marketplace Authority
The Master Server is the only authority for plugin registration.

### 2. Plugin Rules
- Format: WASM only
- Pricing: FREE ONLY
- Permissions declared explicitly
- Signed by author + verified by Master

### 3. Distribution
- Marketplace index is global read-only
- Actual plugin binaries distributed via adapter-p2p

### 4. Categories
- Gameplay Assist (Read-only)
- UI / Themes
- Language Packs
- Streaming / Hardware Integration
- RP / Character Extensions

---

## FILE 3 — SECURITY_DSGVO_DSA_AUDIT.md

### 1. Security Model
- mTLS for all connections
- AES-256-GCM for payloads
- Ed25519 signatures

### 2. DSGVO
- Local-first storage
- No PII off-device
- Right to erasure via local wipe

### 3. DSA
- No hosting of user content
- No algorithmic amplification
- Read-only metadata registry

### 4. Audit Ledger
- Append-only
- Merkle-hash chained
- Non-editable

---

## FILE 4 — INSTALLER_ARCHITECTURE_WINDOWS_DOCKER.md

### 1. Installer Modes

| Mode | Description |
|-----|------------|
| Ghost | Portable, no disk writes |
| Agent | Native Windows service |
| Authority | Docker-based full stack |

### 2. Windows Integration
- Start Menu entry
- Desktop icon
- Taskbar pin
- AppUserModelID set

### 3. Dependencies
- Auto-install optional
- Docker optional
- Native fallback guaranteed

### 4. Clean Uninstall
- Registry cleanup
- Service removal
- Data wipe prompt

---

## FILE 5 — COPILOT_MASTER_COMMAND_FINAL.yml

```yaml
Execution:
  Language: Rust
  Runtime: Tokio + Rayon
  Architecture: Actor Model
  UI: Tauri + SolidJS

Rules:
  - No deviation from spec
  - No questions
  - Full file generation only
  - Tests mandatory

Testing:
  - Unit per Actor
  - Integration per Adapter
  - Local Docker CI

Compliance:
  - CIG ToS read-only
  - IDC-10 enforced
  - DSGVO local-first
```

---

## DEFINITION OF DONE
- Master Server operational
- Marketplace functional
- P2P update flow validated
- Installer verified
- Copilot instructions enforced

END OF BINDING SPECIFICATION.

