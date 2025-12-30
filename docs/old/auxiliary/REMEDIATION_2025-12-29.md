# Remediation Notes — 2025-12-29

This document captures remediation work completed to conform to SC_MANAGER_V7 implementation standards.

- Implemented `scripts/check_forbidden_patterns.sh` to detect `unwrap`, `expect`, `panic!`, and other disallowed constructs in production code.
- Replaced occurrences of `unwrap`/`expect` in production crates with proper `Result`-returning error handling or explicit `TODO` markers tracked in `docs/TRACKED_TODOS.md`.
- Added `docs/TRACKED_TODOS.md` and documented items that require follow-up work, assigned `TRACKED-001..` identifiers.
- Fixed CI scripts and `.gitignore` to avoid committed build artifacts (e.g., `/target`, `*.profraw`, `node_modules`).
- Debugged coverage generation: discovered `tarpaulin` parser fails on Windows after writing `.profraw`; installed `llvm-tools-preview`, merged profraws with `llvm-profdata`, and generated HTML via `llvm-cov`.
- Created a staged implementation for a CI fallback to merge profraws and generate HTML if `tarpaulin` fails; this needs to be added to the CI workflow.

Status: All remediation steps implemented locally and validated via local CI runs. Push/PR withheld until `release` signal.
