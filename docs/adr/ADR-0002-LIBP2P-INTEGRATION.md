# ADR-0002: libp2p integration strategy

Status: Proposed

Context
-------
We are implementing a P2P CRDT mesh for SC Manager (V8). The spec mandates libp2p + QUIC + Gossipsub + Kademlia + mDNS for production networking. Initial attempts at directly integrating the full behaviour caused compile/feature fragility and CI instability.

Decision
--------
We adopt an incremental approach:

1. Keep the existing deterministic TCP-backed test stub as the default behaviour under the `libp2p` feature to ensure CI/test stability.
2. Add a gated, opt-in feature `libp2p_full` that enables a minimal, compile-tested Gossipsub-only implementation first.
3. Progressively add Kademlia, mDNS, and QUIC transport behind `libp2p_full` as the library APIs stabilize and CI runners prove compatible.
4. Add clear documentation and ADR describing the gating, how to enable the full behaviour locally (`cargo test --features libp2p,libp2p_full`), and the rollout plan for CI.

Consequences
------------
- CI remains stable because the default `libp2p` feature keeps the TCP stub behaviour used in tests.
- Contributors can opt-in to `libp2p_full` locally and in dedicated CI jobs to iterate on the real libp2p integration without breaking mainline CI.
- The staged rollout reduces risk and keeps tests deterministic.

Next Steps
----------
- Document the decision here and in `infrastructure/p2p-mesh/README.md`. (this ADR)
- Implement a minimal Gossipsub-only adapter under `libp2p_full` with unit and integration tests.
- Add an optional CI job to exercise `libp2p_full` on a schedule or as a gated job until it is reliable.

Notes
-----
- When enabling QUIC, verify runner kernel/virtualization supports QUIC transport.
- Keep tests deterministic and avoid flakiness by timeouts and retries with conservative delays.