# Master Server API (sc-manager)

Overview
--------
The Master Server provides a compact authority for update distribution, audit logging and a simple marketplace. The API is minimal and intended to be used by trusted tooling and local automation.

Endpoints
---------
- GET /health
  - 200 OK — server is alive

- POST /api/v1/updates
  - Accepts a signed `UpdateEntry` JSON payload. Verifies signature against master's keystore and appends an `UpdateSigned` audit event.
  - Responses: 200 OK, 401 Unauthorized (invalid signature), 500 on storage error.

- GET /api/v1/audit/events
  - Returns array of `AuditEvent` (NDJSON-backed append-only ledger loaded in memory).
  - Responses: 200 OK, 500 on internal error.

- GET /api/v1/marketplace/items
  - Returns array of marketplace `Item` objects.

- POST /api/v1/marketplace/items
  - Create an item with payload { id, owner, price, metadata }
  - Responses: 201 Created, 409 Conflict (item exists), 500 Storage error.

Marketplace persistence
-----------------------
- The marketplace uses an append-only NDJSON ledger (`MarketplaceLedger`).
- Events are appended as `MarketplaceEvent::Create` / `Remove`.
- On startup `Marketplace::with_ledger(path)` can replay events and reconstruct the in-memory state.

Snapshot & compaction
---------------------
- A snapshot (rollup) can be produced from the event log and written atomically to `ledgerfile.snapshot`.
- Compaction rotates the ledger file (renames to `ledgerfile.old.<ts>`) and writes a fresh ledger file after snapshot.
- The snapshot is written atomically: write to a `.tmp` file and `rename()`.

Notes & Next steps
------------------
- OpenAPI generation is planned; for now see `docs/MASTER_SERVER_API.md` for contract and example payloads.
- Integration tests explicitly exercise `Marketplace::with_ledger` replay and snapshot/compaction behavior.
