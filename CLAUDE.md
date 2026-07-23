# HTB-CLI

Rust CLI for Hack The Box. See [AGENTS.md](AGENTS.md) for architecture, patterns, and conventions.

## Quick reference

```bash
cargo build              # build
cargo test               # test
cargo fmt --check        # format check
cargo clippy --all-targets -- -D warnings  # lint
```

## Rules

- Conventional Commits: `type(scope): description`
- Every behavioral PR needs a `.changeset/*.md` with `default: patch` or `default: minor`
- `#[serde(default)]` on every optional API field
- Never mix `print_message()` with `print_list()` in JSON mode
- Tests use wiremock + fixtures in `tests/fixtures/`
- `grep` is aliased to `rg` in this environment; use `rg` syntax
