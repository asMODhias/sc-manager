# SC Manager — Master Server (sc_manager_master_server)

Minimal scaffold for the Master Server authority crate. This crate contains the authoritative signing and registry responsibilities and is intentionally minimal for initial integration and testing.

Run tests:

```bash
cd services/master-server

## Master Server (API)

Endpoints:
- GET /health
- POST /api/v1/updates
- GET /api/v1/audit/events
- GET /api/v1/marketplace/items
- POST /api/v1/marketplace/items

Persistence:
- Marketplace uses an append-only NDJSON ledger (see `src/marketplace/storage.rs`) with snapshot/compaction support. Use `Marketplace::with_ledger(path)` to load from disk.

Testing:
- Unit and integration tests exist in `services/master-server/tests` and `src/*`.

See `docs/MASTER_SERVER_API.md` for API contract and examples.

cargo test
```
