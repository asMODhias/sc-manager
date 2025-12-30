📜 SC MANAGER V7 — THE SUPREMACY CONSTRUCTION SETDocument ID: SCM_V7_SUPREMACY_FINAL_BINDINGVersion: 7.9.9-ULTIMATEStatus: ABSOLUTE – LAW – ZERO DEVIATIONAuthority: MASTER-AUTHORITY ENFORCEDSecurity: Post-Quantum AES-GCM-256 | mTLS 1.3 | WASM-Isolate🎯 0. META-RULES (COPILOT LAW)Zero-Questions Policy: Copilot is strictly forbidden from asking questions. Ambiguity is resolved via Decision Trees.Local-First Supremacy: All development occurs locally. Remote mirrors only.Binding Logic: This document overrides all previous V1-V6 instructions.Autonomous Execution: Copilot MUST generate full, production-ready code blocks including tests.🏗️ 1. ARCHITECTURE: ATOMIC-ACTOR-MESH (AAM)The rework migrates from a monolithic process to an Actor-Model. Each component is an isolated "Actor" communicating via message passing.Core Components (The "Hypervisor")Runtime: tokio (Async I/O) + rayon (Compute Parallelization).State: CRDT (Conflict-free Replicated Data Types) for multi-device/multi-org sync.Storage: RocksDB for high-speed local caching; PostgreSQL (Docker) or SQLite (Native) for relational data.Gossip-Layer: libp2p with Gossipsub for decentralized hash-distribution.🚀 2. THE DISTRIBUTED MASTER & MINI-MASTER SYSTEMEvery installation is a Mini-Master. There is no central point of failure.EntityRoleData RetentionAuthor-MasterAuthority AnkerSignatures, Update-Metadata, Global Registry.Mini-MasterLocal RelayOrg-State, P2P-Caches, Identity Verification.Relay Node24/7 MeshEncrypted Buffer for offline member delivery.The "Trust-Gossip" FlowEvent: Org-Leader updates "Fleet Doctrine".Hash: Mini-Master generates a hash and signs it with the Org-Key.Gossip: Hash is distributed via P2P.Verification: Other Mini-Masters check the signature against the RSI-Auth Core Adapter. If valid, the delta-update is pulled.🛠️ 3. CORE FEATURES & PLUGINS (INTEGRATED)Core (Non-Removable)Universal StarMap: SVG/WebGL Hybrid. Real-time POI, Fleet-Tracking, and Pathfinding.RSI Auth Adapter: The root of all identity and permission layers.Unified P2P Updater: Self-distributing delta-patches via the Mesh.Holo-Stream (LAN): Zero-install tablet support via local WebSocket server.Integrated Plugins (Marketplace)Grinding V7: Read-only Game.log parser using Kernel-Level mmap. ToS-compliant manual verification loop.Hardware-Sync: Elgato, Razer, Corsair, SteelSeries integration via WASM-Drivers.Streamer-Connect: Live-Status mirror for Twitch/YT members in the Org-List.Screenshot/Media: Automatic hashing and Org-Storage integration for mission proof.🔐 4. SECURITY & TOS COMPLIANCEZero Memory-Hooks: 0% access to SC process memory. 100% compliant.WASM-Sandbox: Plugins have NO filesystem access. They use a Capability-Based Security model.IDC-10 Governor: Background service detects StarCitizen.exe and limits CPU usage to <2% and IO priority to "Background".🤖 5. COPILOT DECISION TREES (AUTOMATION)YAMLDecision_Matrix:
Environment:
Docker_Detected: Mode = AUTHORITY (Full Stack)
Windows_Desktop: Mode = AGENT (Service + SQLite)
Removable_Drive: Mode = GHOST (Portable + RAM-Only)

Updates:
Signature_Valid: Propagate via P2P
Signature_Invalid: Quarantine + Report to Author-Master

Data_Conflict:
CRDT_Resolution: LWW (Last-Write-Wins) based on Hardware-Timestamp
🧪 6. LOCAL CI/CD PIPELINE (.github/workflows/local-ci.yml)Copilot MUST use the local act runner to validate every generation.YAML# Local-First CI Logic
steps:

- name: Rust Security Audit
  run: cargo audit
- name: Unit Tests (Atomic-Mesh)
  run: cargo test --lib
- name: IDC-10 Compliance Check
  run: ./scripts/check-memory-leak.sh
- name: WASM Plugin Validation
  run: wasm-verify ./plugins/\*.wasm
  📉 7. LEGACY STATUS (V1-V7)DELETED: Electron, Central API, Global Postgres dependency, Manual Patching.RETAINED: All functional features from V1-V6.OPTIMIZED: 1500% faster UI, 90% less bandwidth, 100% Uptime through Mesh-Redundancy.✅ DEFINITION OF DONE (THE SUPREMACY CHECKLIST)[ ] Single-Binary: One EXE for Ghost/Agent/Authority.[ ] Master Mesh: Peer discovery and gossip-sync functional.[ ] ToS Guard: Log-parser active with 0% memory interference.[ ] Installer: WiX v4 Bundle with mTLS-Certificate generation.[ ] Tests: >90% coverage on Core-Mesh logic.

COMMAND: Proceed with the implementation of CORE_MESH_HYPERVISOR.rs using the Actor-Model and mmap-Log-Parsing.

No questions allowed. Start now.
