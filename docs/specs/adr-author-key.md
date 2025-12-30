# ADR: Author Key Handling

Status: Proposed
Date: 2025-12-30

## Context
The Author Key is the root of trust for the Master Server authority. It must never be exposed accidentally, and key usage must be explicit, auditable, and minimal. The key is used to sign updates and publish authority-bound events to the master ledger.

## Decision
- Author keys MUST be generated on an offline, air-gapped machine.
- Keys MUST be stored in hardware-backed secure storage where available (HSM, YubiKey, NitroKey). If hardware is not available, store in encrypted file protected by a high-entropy passphrase and PBKDF2 with sufficiently large iteration count (>=100_000).
- Private keys SHOULD never be committed to repos or pasted into build systems.
- For operational convenience, a separate `signing` keypair may be derived for day-to-day operations; the root key can be used to sign rotation events.
- Key revocation and rotation MUST be supported via the Master Server ledger (publish a revocation AuditEvent signed by the old key and a rotation entry signed by the new key).

## Implementation Notes
- Use Ed25519 for signatures (compact, fast, well-supported by `ed25519-dalek`).
- Use AES-GCM for file encryption and PBKDF2-HMAC-SHA256 with a salt and iterations to derive the encryption key. See `services/master-server/src/keys.rs` for reference helper methods.
- Canonical signing: Sign a stable canonical representation of payloads (e.g., deterministic JSON with signature field omitted) — see `UpdateEntry::canonical_bytes()`.

## File Format (Encrypted JSON)
```json
{
  "kdf": "pbkdf2",
  "iterations": 100000,
  "salt": "<base64>",
  "nonce": "<base64>",
  "ciphertext": "<base64>"
}
```

## Examples
### Generate a test keypair (for dev / CI only)
```rust
let ks = KeyStore::generate_testpair();
```

### Save encrypted key to disk
```rust
ks.save_encrypted("author.key.json", "s3cr3t-passphrase")?;
```

### Load encrypted key from disk
```rust
let ks = KeyStore::load_encrypted("author.key.json", "s3cr3t-passphrase")?;
```

### Signing flow
1. Build `UpdateEntry`, compute canonical bytes via `canonical_bytes()`.
2. Sign canonical bytes with the author key: `sig = ks.sign(&canonical)`.
3. Base64-encode signature and place in `UpdateEntry.signature`.
4. Publish via `publish_update` which verifies signature and appends `AuditEvent`.

## Operational Guidance
- Rotate keys at planned intervals; always publish rotation to ledger.
- Keep an offline copy of the root key in cold storage (e.g., air-gapped USB or printed QR with secure shredding plan).
- Use multi-signer threshold schemes (future work) for improved security.

## Consequences
- Secure signing and publish pipeline; better compliance with audit requirements.
- Requires operational discipline for key rotations and passphrase handling.

---
*This ADR is a living document; update when new requirements or technologies arise.*