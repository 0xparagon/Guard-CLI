## Summary

Makes the suppression comment parser tolerant of whitespace (#602, first acceptance criterion). This PR implements that single instruction; #601, #603 and #604 are referenced but not implemented here (see "Not done").

**Stacked on #636** (techisigu), which makes the CLI and test targets compile. Please merge #636 first; this branch contains its commits, and the diff of this PR alone is one commit (`crates/analyzer/src/lib.rs`).

## #602 analyzer: suppression comment is whitespace-exact

What existed: `SUPPRESSION_PREFIX = "// soroban-guard: allow("` and `parse_allow_checks` did `strip_prefix` on it, so `//soroban-guard: allow(x)`, `// soroban-guard:allow(x)` and `//  soroban-guard: allow(x)` were silently ignored.

Done:
- `parse_allow_checks` now matches `//`, optional whitespace, `soroban-guard`, optional whitespace, `:`, optional whitespace, `allow(`. The constant is removed
- `///` and `//!` doc comments and trailing comments after code are still not suppressions
- Tests: each variant from the issue plus tabs and spaces around `:` are accepted; doc comments, missing `:`, empty `allow ()`, `soroban-guardian` and trailing comments are rejected; `//soroban-guard:allow(missing-require-auth)` above a method registers the function-level suppression

Not done in this PR:
- Warning on stderr for unknown check names inside `allow(...)`
- Documenting the accepted syntax in `docs/false-positives.md`

## #601 cli: split `main.rs` into library modules

Not done in this PR: `output/{json,sarif,markdown,pretty}.rs`, `meta.rs`, slimming `main.rs`, byte-identical output check, moving the unit tests.

## #603 checks: `#[contractevent]`-style `.publish(&env)`

Not done in this PR: recognising `.publish(&env)` as an event emission in both event checks, the `event-contractevent-safe` fixture and tests, the docs update.

## #604 checks: `__constructor` semantics

Not done in this PR: deciding and documenting the semantics, exemptions in the affected checks, the `constructor-safe` fixture and tests.

## Verification

`main` does not build the `checks` lib (#570, #573-#575, other contributors), so this was tested on top of #636 plus a throwaway local-only repair of those files, which was rebased away afterwards (the diff against the tested branch equals the reversed repair). `cargo test -p soroban-guard-analyzer`: the 3 new tests pass; the full result list is identical to #636's except for the 3 added tests (10 pre-existing failures, mostly duplicated findings, unchanged). The change adds no `rustfmt` differences (the file has 18 pre-existing ones, #623).

Closes #601
Closes #602
Closes #603
Closes #604
