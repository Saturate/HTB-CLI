# Agent Guide

Instructions for AI agents (Claude Code, Copilot, Cursor, etc.) working on this codebase.

## Build and test

```bash
cargo build
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

MSRV is 1.97. CI runs all four on every PR (stable + nightly, ubuntu + macOS).

## Commits

Use [Conventional Commits](https://www.conventionalcommits.org):

```
type(scope): short description
```

Types: `feat`, `fix`, `docs`, `chore`, `refactor`, `test`, `perf`, `ci`
Scope: module name (`auth`, `ctf`, `machines`, `cache`, etc.) or omit for cross-cutting changes.

## Changesets

Every PR that changes behavior needs a `.changeset/<name>.md` file:

```markdown
---
default: patch
---

#### Short title

What changed and why.
```

Package name is always `default` (not the crate name). Use `patch` for fixes, `minor` for features. CI blocks PRs without one.

## Architecture

Single-crate CLI. Three parallel module trees mirror each other:

| Layer | Path | Role |
|---|---|---|
| CLI | `src/cli/*.rs` | Clap subcommands, input validation, output formatting |
| API | `src/api/*.rs` | HTTP calls via `HtbClient`, one struct per domain |
| Models | `src/models/*.rs` | Serde types for API responses |

Adding a new domain (e.g. `foo`) means adding `src/cli/foo.rs`, `src/api/foo.rs`, `src/models/foo.rs`, and wiring them into each `mod.rs`.

`HtbClient` uses sub-API structs that borrow the client:

```rust
client.machines().list().await?;
client.ctf().events().await?;
```

## API response patterns

- Every optional field gets `#[serde(default)]`; the HTB API returns `null` in unexpected places
- Integer fields that can be `null` use `Option<u32>`, not bare `u32`
- Boolean-ish fields from the API (0/1/null) use `Option<u32>`, not `bool`
- Use `deserialize_bool_or_null` (in `models/mod.rs`) only for actual booleans that arrive as `null`
- Use `deserialize_string_or_int` for fields that vary between string and integer across endpoints

## Output

- `print_list()` / `print_detail()` handle both table and JSON formats
- `print_message()` is for action feedback only (start/stop/submit responses); never mix with `print_list` in JSON mode
- Suppress status lines (team summaries, pagination footers) when `format == OutputFormat::Json`

## Testing

- Unit tests in `#[cfg(test)]` modules next to the code
- Integration tests in `tests/` using wiremock
- Fixture JSON in `tests/fixtures/`; use realistic shapes from the live API
- New API models need a deserialization test with a fixture

## Two token systems

The CLI has two separate auth flows:

- **Labs** (`app.hackthebox.com`): `htb auth login`, stored at `~/.htb-cli/.token`
- **CTF** (`ctf.hackthebox.com`): `htb ctf auth login`, stored at `~/.htb-cli/.ctf-token`

Never hardcode or commit tokens.

## Caching

`src/cache.rs` provides disk-based TTL caching in `~/.htb-cli/cache/`. GET requests are cached with tiered TTLs. POST requests invalidate related cache entries. `--no-cache` bypasses it.

## Error handling

- `HtbError` (thiserror) for domain errors in `src/error.rs`
- `anyhow` at the CLI handler boundary
- Don't `unwrap()` API responses; always handle `null` fields
