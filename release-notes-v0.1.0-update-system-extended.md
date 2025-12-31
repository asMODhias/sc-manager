v0.1.0-update-system

Initial release for the Update System crate (PR #26)

### Changelog (summary)
- Ed25519 key generation & rotation (ed25519-dalek)
- Author keystore (file-backed) and sign API
- Client HTTP manifest & payload download and verification (reqwest)
- Delta chunking and atomic apply with SHA3-256 verification
- Transactional replace and rollback utilities
- Unit & integration tests

### Assets
- Source tarball: release-update-system-v0.1.0.tar.gz
- Documentation ZIP: release-update-system-v0.1.0-docs.zip

### Links
- Release: https://github.com/asMODhias/sc-manager/releases/tag/v0.1.0-update-system
- PR: https://github.com/asMODhias/sc-manager/pull/26
- Tag: v0.1.0-update-system

### CI / Notes
- CI matrix updated to install `protoc` when running `libp2p` feature jobs to avoid prost/protobuf build failures.
- Please check GitHub Actions run for the release and report any issues.

### Suggested Tweet (DE)
"Veröffentlicht: v0.1.0-update-system — Neues Update‑System für SC Manager!
Ed25519‑Signing, file‑keystore, HTTP‑Client, atomarer Delta‑Apply & Rollback. Details + Assets: https://github.com/asMODhias/sc-manager/releases/tag/v0.1.0-update-system #scmanager #rustlang #p2p"

(Unter 280 Zeichen)

### Suggested Tweet (EN)
"Release v0.1.0-update-system: New Update System for SC Manager — Ed25519 signing, keystore, HTTP client, atomic delta apply & rollback. Details & assets: https://github.com/asMODhias/sc-manager/releases/tag/v0.1.0-update-system #scmanager #rustlang"

### Suggested LinkedIn / Blog blurb
"We released v0.1.0-update-system: an initial implementation of the Update System for SC Manager. Highlights: Ed25519 key management, a file-backed keystore with rotation, HTTP-based manifest & payload verification, chunked delta apply with SHA3 verification, and transactional rollback. Download source and docs from the release page: https://github.com/asMODhias/sc-manager/releases/tag/v0.1.0-update-system"

---

If you'd like, I can also directly post the Tweet via a connected social account, or open a GitHub Discussion post summarizing the release. Reply: "Post Tweet", "Open Discussion", or "No further action".