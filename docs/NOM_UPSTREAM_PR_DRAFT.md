# Draft PR: Remove trailing semicolons in macro arms

## Summary
This patch removes trailing semicolons from several macro arms in `nom` that trigger the "trailing semicolon in macro used in expression position" future-incompatibility warning (Rust issue #79813). The changes are minimal and mechanical: macros that previously ended with `... );` are changed to produce the same expansion without the trailing semicolon so macro-invocation-at-the-end-of-blocks doesn't create a stray semicolon expression.

Files changed (workspace-local copy):
- `src/macros.rs` (macro arms where `opt!/call!/flat_map!` were invoked)
- `src/sequence.rs` (tuple_parser macro arms)
- `src/simple_errors.rs` (flat_map/fix_error macro arm variants)

## Motivation
Rust's "semicolon_in_expressions_from_macros" future-incompat lint warns about macro invocations that expand to trailing semicolons in expression position; this is being hardened to an error in a future compiler release. The patch removes trailing semicolons from macro arms to keep `nom` compatible with future Rust versions.

## Test / Verification
- `cargo check -p nom` (local) — no longer errors on trailing-semicolon diagnostics; other warnings (deprecation / cfg) remain and are unrelated.
- Crate tests: run dependent crate test suites (e.g., integration in our workspace) — passed after patch.

## Suggested commit message
```
fix(nom): avoid trailing semicolons in macro arms

Remove trailing semicolons from selected macro arms (sequence.rs, macros.rs, simple_errors.rs)
so macro invocations at the end of a block are not treated as expression-position semicolons.

This prevents the `semicolon_in_expressions_from_macros` future-incompatibility warning from
being raised in downstream projects.
```

## Notes for reviewers
- Changes are intentionally minimal and conservative; they do not alter parser semantics.
- Please consider a small regression test that triggers the previously reported warning and ensures it no longer prints (a `cargo check` assertion or compile-fail test would suffice).

## Next steps
- If desirable, I can open a fork+PR against upstream `nom` (Geal/nom) with the patch and test-case.
- I can also prepare fixes for other deprecation warnings (range patterns `...` -> `..=`, and ``dyn`` for trait objects) in a follow-up PR.

--
Patch generated from internal workspace: `sc-manager` (branch `fix/nom-macro-semicols`).
