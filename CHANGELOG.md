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
- Imported CIG Fankit assets and tooling (PR #27) — added `assets/fankit/`, generated `fankit-manifest.json`, import scripts, manifest-check CI workflow, documentation and demo pages. Merged via **admin override** due to exhausted GitHub Actions quota; tag `v8.0.0-fankit` created and release published. License/TOS file `assets/fankit/LICENSE_CIG_FANKIT.txt` was included and confirmed by the repository owner.

(See `infrastructure/update-system/CHANGELOG.md` for crate-specific notes.)
