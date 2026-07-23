# Contributing

PRs are welcome. Open an issue first if you want to discuss a larger change.

## Setup

```bash
git clone https://github.com/Saturate/HTB-CLI.git
cd HTB-CLI
cargo build
cargo test
```

MSRV is **1.97**. CI runs `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`, and `cargo audit` on every PR.

## Before submitting

1. `cargo fmt`
2. `cargo clippy --all-targets -- -D warnings`
3. `cargo test`
4. Add a changeset (see below)

## Changesets

Every PR that changes behavior needs a changeset file in `.changeset/`. Create a markdown file with this format:

```markdown
---
default: patch
---

#### Short title

What changed and why.
```

Use `patch` for fixes, `minor` for new features. The package name is always `default`. [Knope](https://knope.tech) consumes these at release time to bump the version and generate the changelog.

CI will block the PR if the changeset is missing (the "Require changes to be documented" check).

## Commits

Use [Conventional Commits](https://www.conventionalcommits.org): `feat(scope):`, `fix(scope):`, `docs:`, `chore:`, etc. Scope is optional but appreciated.

## Code style

- No `any`, `unknown`, or type casts; narrow types instead
- `#[serde(default)]` on every optional API field; the HTB API returns `null` in unexpected places
- Comments only when the "why" is non-obvious
- Don't refactor adjacent code in a bug-fix PR

## Testing

- Unit tests live next to the code in `#[cfg(test)]` modules
- Integration tests go in `tests/`
- Fixture JSON files go in `tests/fixtures/`; use realistic shapes from the live API
- If you add a new API model, add a deserialization test with a fixture

## API tokens

You need an HTB account to test live commands. The CLI uses two separate tokens:

- **Labs**: `htb auth login` (from [account settings](https://app.hackthebox.com/account-settings))
- **CTF**: `htb ctf auth login` (from [ctf.hackthebox.com](https://ctf.hackthebox.com) after logging in)

Never commit tokens or fixture data that contains real credentials.
