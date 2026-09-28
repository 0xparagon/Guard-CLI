## Summary

Pins the structured scan output with golden files (#608, first acceptance criterion), plus a one-line analyzer fix needed so the goldens record correct output. This PR implements that single instruction; #605, #606 and #607 are referenced but not implemented here (see "Not done").

**Stacked on #636** (techisigu), which makes the CLI and test targets compile. Please merge #636 first; this PR's own changes are the last two commits.

## Out-of-scope fix: every file was scanned twice

`collect_rust_paths` (`crates/analyzer/src/lib.rs`) pushed each accepted path in the `PathVerdict::Scan` arm and then again with a stray `paths.push(path.to_path_buf())` after the `match`, which also queued rejected/unreadable files once. `scan test-contracts/vulnerable` reported `Scanned 2 file(s)` for a one-file crate and every finding twice. Removing the stray line:
- `cargo test -p soroban-guard-analyzer`: 10 failures -> 1 (the dedup, file-count, include-filter, generated-file, unreadable-file and panic tests now pass; the remaining failure is `function_scoped_suppression_silences_empty_function_name_findings`)
- `cargo test -p soroban-guard-cli --test fixture_scans`: 3 failures -> 2 (`cli_scan_path_does_not_emit_duplicate_findings` passes; `division_fixtures` #620 and `zero_address_fixtures` #621 / #622 remain)

## #608 tests: golden files for `--json`, `--sarif` and `--markdown`

What existed: nothing pinned the structured outputs (`docs/json-schema.md` documents them).

Done:
- `crates/cli/tests/golden/scan-vulnerable.{json,sarif,md}`: output of `soroban-guard scan test-contracts/vulnerable` in each format (6 findings)
- `crates/cli/tests/golden_output.rs`: runs the real binary (`CARGO_BIN_EXE_soroban-guard`) from the workspace root with `NO_COLOR=1`, expects exit code 1 (High findings), and compares stdout byte-for-byte with the golden file after normalising `\` to `/` and CRLF to LF. File paths in the output are already relative to the scan root
- Checked that the test catches drift: changing one `"line"` value in the JSON golden fails `json_output_matches_golden`

Not done in this PR:
- `UPDATE_GOLDEN=1` opt-in regeneration (for now the failure message says to update the golden file; regenerate by running the scan and redirecting stdout)
- Documenting the workflow in `CONTRIBUTING.md`

## #605 checks: `float-in-contract`

Not done in this PR: the check module, registration, unit tests, fixture pair, all-findings entry, docs section, suggestion.

## #606 checks: `unchecked-numeric-cast`

Not done in this PR: the check module, registration, `try_from` suggestion, tests, fixture pair, all-findings entry, docs with heuristic limits.

## #607 checks: `single-step-admin-transfer`

Not done in this PR: the check module, registration, propose/accept suggestion, tests, fixture pair, all-findings entry, docs.

## Verification

`main` does not build the `checks` lib (#570, #573-#575), so this was tested on top of #636 plus a throwaway local-only repair of those files, rebased away afterwards (the diff against the tested branch equals the reversed repair). `cargo test -p soroban-guard-cli --test golden_output`: 3/3. New test file is `rustfmt`-clean.

Closes #605
Closes #606
Closes #607
Closes #608
