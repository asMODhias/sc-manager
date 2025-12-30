# ADR-0001: COPILOT Development Guide and Local-first CI

Status: Accepted

Context
-------
The SC_MANAGER_V7* documents define implementation standards for the project. The repository uses a local-first CI pattern and enforces strong linting, testing, and no-unwrap/no-panic rules in production code.

Decision
--------
We adopt the following guidance as project policy:

- Follow SC_MANAGER_V7_REFERENCE and SC_MANAGER_V7_FEATURE_MATRIX_AND_COMPLETION as authoritative.
- Use local-first CI: developers must run `scripts/run-local-ci.ps1` (Windows) or the corresponding shell script locally, and fix all issues before pushing.
- Treat Clippy warnings-as-errors on CI (fix locally before pushing).
- Avoid unwrap()/expect()/panic!() in non-test code; prefer Result-based error handling and return errors up the stack.
- Use `coverage-fallback.ps1` to gather coverage artifacts on Windows; full HTML coverage will be generated in Linux runner using llvm-profdata/llvm-cov.
- Archive non-V7 docs under `docs/auxiliary/` to keep V7 docs authoritative.

Consequences
------------
- Improves CI reliability and reproducibility.
- Ensures consistent code quality and auditability.
- Requires developers to run local CI frequently and address Clippy/style issues promptly.

Notes
-----
This ADR documents decisions already implemented in scripts and code; it can be amended in follow-up ADRs if the CI workflow or coverage approach changes.
