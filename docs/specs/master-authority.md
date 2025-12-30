# Master Authority — Spec (Draft)

Version: v8.0.0-ALPHA.0.0.1
Status: DRAFT

## Purpose

This document specifies the **Master Server Authority** (Author Master) responsibilities, API surface, data models and operational constraints.

Goals:
- Provide append-only **Audit Ledger** (content-hashed + Merkle chain) for ToS/audit events
- Provide **Update Authority** ledger for signed releases
- Provide **Plugin Marketplace Registry** with signed WASM artifacts and approval workflow
- Expose health/metrics endpoints for monitoring
- Ensure privacy: only store geo_region (ISO-3166) and hashed/anonymized sources

## Design Principles
- Append-only storage (no updates/deletes) for audit records
- Author signature is the root of trust. Keys are created offline and loaded securely.
- Local-first by default; Master is authoritative but not controlling gameplay
- Performance: target IDC-10 constraints (minimize CPU when game process detected)

## Data Models (summary)

- AuditEvent
  - event_id: SHA3-512 of event content (hex)
  - timestamp: ISO8601 UTC
  - event_type: enum
  - source_hash: anonymized identifier
  - payload_hash: SHA3-512 of payload
  - geo_region: ISO-3166-1
  - software_version: semver string
  - signature: Option<Ed25519 Signature>
  - previous_hash: previous event_id (Merkle/chain linkage)

- UpdateEntry
  - content_hash: CID/content-addressed hash
  - version: semver
  - channel: enum (Alpha/Beta/RC/Stable)
  - min_supported_version
  - signature: Ed25519 signature
  - signed_at: timestamp
  - changelog_url, size_bytes, update_type

- PluginEntry (WASM)
  - plugin_id
  - name, author, version
  - content_hash
  - signature
  - approved_by: Authority signature
  - status: Registered/Approved/Blocked/Deprecated

## API Endpoints (initial)

All endpoints require TLS and auth where appropriate. Author-only operations require an offline-signed request (signature in header) or an mTLS client certificate.

### Public Endpoints (read-only)
- GET /health
  - 200 OK `{ "status": "ok" }`
- GET /metrics (Prometheus)
- GET /audit/events?limit=&after=&type=
  - Returns paginated audit events (payloads omitted, only hashes and metadata)
- GET /audit/events/{event_id}
  - Returns metadata for an event
- GET /updates?channel=&since=
- GET /plugins?status=&author=

### Author-only (write)
- POST /author/updates
  - Payload: `UpdateEntry` + signature header
  - Validates signature and appends to update ledger and emits AuditEvent (UpdateSigned)
- POST /author/plugins/register
  - Payload: PluginEntry + signature header; stores plugin metadata and emits PluginRegistered event
- POST /author/audit
  - Payload: AuditEvent (signed) — used for author-anchored events

### Operational / Admin
- POST /maintenance/reindex — restricted, for offline use only (requires author OTP)
- GET /debug/ledgersize — read-only internal metrics (auth required)

## Security Model
- AuthorKeyManager: keys are generated offline and loaded on the author machine only
- All write operations must either be executed by the author using an air-gapped process or via signed payloads verified by the Master
- Plugin content validated by signature before approving and before marketplace distribution

## Acceptance Criteria
- `docs/specs/master-authority.md` exists (this file) and includes API + data models
- A basic server skeleton exists under `infrastructure/master-server` exposing `/health`, `/metrics`, and a read-only `/audit/events` with mocked data
- Unit tests for `AuditEvent::hash` and `verify_chain` are present
- Integration test: create `UpdateEntry` signed by a test key and verify server accepts and emits an `AuditEvent` entry

## Next steps (implementation plan)
1. Create server skeleton and routing (actix-web or axum) with `/health` and `/metrics` endpoints
2. Implement `AuditEvent` domain and unit tests
3. Implement append-only storage with simple file-backed or RocksDB-backed ledger and test chain integrity
4. Implement AuthorKeyManager (basic load/sign/verify) and secure examples (ADR)
5. Implement UpdateEntry endpoint and signed publishing pipeline
6. Implement Plugin registry and approval workflow (WASM validation later)

---

Document created by automation (assistant) — please review and adjust details/formatting as needed.
