# Tracked TODOs

This file lists non-trivial TODOs found across the workspace and assigns them a tracking id so they are no longer untracked inline comments. Each entry contains: id, file, short description, suggested remediation, and priority.

- TRACKED-001
  - File: adapters/adapter-p2p/src/transport.rs (tests)
  - Description: Replace `unwrap`/`expect`-style usage within the transport tests with proper error propagation or explicit test assertions and avoid using production panics.
  - Suggested remediation: Update test helpers to return Result and assert errors; avoid using unwrap in non-test helpers shared across crates.
  - Priority: low

- TRACKED-002
  - File: adapters/src/adapter_api.rs (MockPublisher tests)
  - Description: Replace lock usage in `MockPublisher::publish` with explicit error handling instead of silently returning Ok on mutex poisoning and document expected behavior in test harness.
  - Suggested remediation: Return error from `publish` on lock poisoning or retry with backoff in test harness.
  - Priority: low

- TRACKED-003
  - File: adapters/src/discord.rs (tests)
  - Description: Replace unexpected-variant handling (unreachable) in test with explicit match failure handling or make fetch return a Result to surface errors.
  - Suggested remediation: Call `panic!` only in tests with clear message or assert that matches the expected variant.
  - Priority: low

- TRACKED-004
  - File: core/src/domain/member.rs (production)
  - Description: Add verification logic for `rsi_handle` (RSI handle validation and normalization).
  - Suggested remediation: Implement a verifier (regex-based) to validate RSI handles, add unit tests, and document validation rules. Consider making it a helper function with clear errors.
  - Priority: medium

- TRACKED-005
  - File: services/core-application/patches/cron (multiple files)
  - Description: Several TODOs exist (e.g., Cutoff datetime, Combinator, use phf crate, doc examples). These are upstream/patch-level items and should be tracked upstream or moved to workspace issues.
  - Suggested remediation: Create upstream issues or local issues for each TODO and annotate with upstream ticket number.
  - Priority: low


If you want, I can open CHANGES/ISSUES on GitHub for higher priority items; otherwise I will keep this list in the repository and update code comments to reference the TRACKED-* ids.
