# Changelog for `infrastructure/update-system`

## Unreleased

### Added
- Initial implementation of the Update System crate (TASK-001) including:
  - Ed25519 key generation & rotation (ed25519-dalek)
  - Author keystore (file-backed) and sign API
  - Client HTTP manifest & payload download and verification (reqwest)
  - Delta chunking and atomic apply with SHA3-256 verification
  - Transactional replace and rollback utilities
  - Unit & integration tests (httpmock, tempfile)

### Notes
- No breaking changes.
- All new code is covered by unit and integration tests. See `infrastructure/update-system/README.md` for usage and test instructions.

### Test Instructions
1. Checkout branch: `git checkout feat/update-system`
2. Run: `cd infrastructure/update-system && cargo test`
3. Verify tests pass locally and in CI.

---

(PR: #26)
