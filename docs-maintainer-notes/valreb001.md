# Maintainer notes (Valreb001)

## #623 — cargo fmt --all -- --check

This environment has no access to `cargo`/`rustfmt`, so a real `cargo fmt --all`
pass could not be run or verified here. Hand-formatting the ~92 hunks across 29
files without the formatter would be error-prone and risks introducing
formatting that doesn't actually match rustfmt's output (and could silently
diverge further from the pinned 1.74.0 toolchain's behavior).

Instead, this PR adds the requested `CONTRIBUTING.md` line asking contributors
to run `cargo fmt --all` before pushing, and defers the actual formatting pass
to a maintainer with local tooling:

```
rustup run 1.74.0 cargo fmt --all
cargo fmt --all -- --check
```

That command should be run in a dedicated formatting-only PR (no logic
changes) as the issue requests, ideally merged first so other open PRs can
rebase cleanly onto the reformatted tree.
