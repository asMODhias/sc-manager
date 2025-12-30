# Changelog

## Unreleased

### Added
- Implemented Update System crate (`infrastructure/update-system`) — initial implementation. See PR #26 for details.
  - Ed25519 key generation & rotation (ed25519-dalek)
  - Author keystore (file-backed) and sign API
  - Client HTTP manifest & payload download and verification (reqwest)
  - Delta chunking and atomic apply with SHA3-256 verification
  - Transactional replace and rollback utilities
  - Unit & integration tests

(See `infrastructure/update-system/CHANGELOG.md` for crate-specific notes.)
