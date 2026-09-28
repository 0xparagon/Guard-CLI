## Summary

Removes the self-contradicting assertions in `ignores_local_collection_write_not_storage` (#615, first acceptance criterion). This PR implements that single instruction; #614, #616 and #618 are referenced but not implemented here (see "Not done").

**Stacked on #636** (techisigu), which makes the checks crate's test target compile. Please merge #636 first; this PR's own change is the last two commits.

## #615 checks: `ignores_local_collection_write_not_storage` contradicts itself

What existed: the test asserted `findings.len() == 1`, `findings[0].line == 6` and `findings.len() == 0` in a row. The input writes to a local `Map`, not storage, and the check correctly reports nothing, so the first assertion failed.

Done:
- Deleted the stray `len() == 1` and `line == 6` assertions, leaving `assert_eq!(findings.len(), 0)`
- `cargo test -p soroban-guard-checks missing_event_for_admin_change` passes (3/3); the checks lib test target goes from 4 failures to 3 (`key_collision` #616, `reinit` #617, `transfer::ignores_transfer_ownership` remain)

Not done in this PR:
- Nothing else in #615 (its second criterion is the passing test run above)

## #614 analyzer: `ScanReport` instead of the 4-tuple

Not done in this PR: the `#[non_exhaustive] ScanReport` struct, updating callers and tests, rustdoc.

## #616 checks: `key_collision` "known gap" test

Not done in this PR: confirming the finding, renaming and asserting it, docs update.

## #618 checks: `re-initialization-risk` guard macros

Not done in this PR: `panic_with_error` / `unreachable` / `panic` as diverging macros, `assert!` / `assert_with_error!` guards, the three tests.

## Verification

`main` does not build the `checks` lib (#570, #573-#575), so this was tested on top of #636 plus a throwaway local-only repair of those files, rebased away afterwards (the diff against the tested branch equals the reversed repair).

Closes #614
Closes #615
Closes #616
Closes #618
