# Update System (TASK-001)

Minimal scaffold implementation for the Update System described in the project SOT.

This crate provides:
- `author` - minimal manifest signing helper (placeholder SHA3-based signature for tests)
- `signature` - verification helper compatible with `author`
- `client` - client verification API
- `delta` - chunking helper
- `rollback` - simple backup helper

This is a starting point; Ed25519 signing, chunked downloading, delta application and robust rollback logic will be added in subsequent commits as per TASK-001.
