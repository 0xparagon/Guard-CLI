# Branch protection

`main` should be protected so broken commits cannot land while CI is red.

Recommended GitHub branch rule for `main`:

- Require a pull request before merging.
- Require status checks to pass before merging.
- Require branches to be up to date before merging.
- Required status checks:
  - `CI / ci`
  - `Security / security-audit`
- Restrict force pushes and branch deletion.
- Require CODEOWNERS review for files covered by `.github/CODEOWNERS`.

The repository CI workflow uses the stable Rust toolchain so it can read the
current Cargo lockfile format and should be the required merge gate for changes
to `main`.
