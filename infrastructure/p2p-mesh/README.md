# sc_manager_p2p_mesh

Purpose
-------
This crate contains the P2P mesh and CRDT adapters used by SC Manager (V8):
- `CRDTStateManager` (Automerge)
- Hash-gossip protocol (feature-gated libp2p support)

Feature flags
-------------
- `libp2p` (default off) — enables the crate's P2P adapters and tests that exercise the TCP-backed compatibility stub.
- `libp2p_full` (opt-in) — enables a minimal Gossipsub-only libp2p implementation intended as the first step towards a full libp2p + QUIC + Kademlia + mDNS network behaviour. This feature is intentionally gated and used for iterative development.

Running tests
-------------
- Fast/local tests (use TCP stub):

  cargo test --features libp2p

- To opt into the work-in-progress libp2p implementation (may be unstable):

  cargo test --features "libp2p libp2p_full"

CI
--
- CI runs the crate tests with `--features libp2p` by default to guarantee determinism.
- A staged CI job for `libp2p_full` will be added as an optional/draft job until the full libp2p behaviour is stable across runners.

Roadmap
-------
1. `libp2p_full`: minimal Gossipsub-only adapter and tests
2. Add Kademlia + mDNS for peer discovery
3. Replace development transport with QUIC once supported and tested in CI
4. Performance and stress tests; then promote `libp2p_full` to stable feature

Notes
-----
See `docs/adr/ADR-0002-LIBP2P-INTEGRATION.md` for the decision rationale and rollout plan.