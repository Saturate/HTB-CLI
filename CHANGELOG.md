# Changelog
## 0.1.11 (2026-07-24)

### Features

#### Challenge status tracking and team coordination

Added `htb ctf associate`, `htb ctf disassociate`, and `htb ctf progress`
for assigning team members to challenges and tracking work status.
Added `htb ctf team` to list team members with their IDs.

## 0.1.10 (2026-07-24)

### Features

#### Sticky CTF event selection

Added `htb ctf use <event_id>` to persist the active CTF event to config.
All event-scoped commands now accept the event ID as optional, falling back
to the stored value.

## 0.1.9 (2026-07-24)

### Features

#### Shell completions generation

Added `htb completions <shell>` to generate shell completions for bash, zsh,
and fish via `clap_complete`.

### Fixes

#### Parse HTML error responses into friendly messages

When the API returns HTML instead of JSON on errors (e.g. a 403 from nginx),
the CLI now extracts the `<title>` text instead of dumping raw HTML.

#### Show --mcp-stdio in help output

Moved `--mcp-stdio` from a pre-parse arg check to a proper clap flag so it
appears in `htb --help`.

## 0.1.8 (2026-07-23)

### Fixes

#### Consolidate CLAUDE.md and AGENTS.md

CLAUDE.md is now a symlink to AGENTS.md so there's one file to maintain.

#### Add contributing guide

Added CONTRIBUTING.md with build/test setup, changeset requirements, commit conventions, and code style guidelines. Removed SPEC.md and planning files in favor of GitHub issues for tracking remaining work.

## 0.1.7 (2026-07-23)

### Fixes

#### Suppress team summary in JSON output and add jq examples

`ctf challenges --json` and `ctf scoreboard --json` no longer print the team summary line before the JSON array. Added jq examples to the README showing how to filter and reshape JSON output.

## 0.1.6 (2026-07-23)

### Fixes

#### Document CTF workflow

Added CTF and PwnBox sections to the README with the full challenge workflow. Expanded `htb ctf --help` with numbered steps showing the typical flow from auth through flag submission.

#### Fix CTF API deserialization errors

`hasJoined`, `hide_scoreboard`, `docker_online`, and `docker_ports` now handle the types the API actually sends (`null`, integers, arrays) instead of failing on deserialization.

## 0.1.5 (2026-07-23)

### Features

#### CTF platform support

Interact with HTB CTF events from the terminal via `htb ctf`.

- `htb ctf auth login/status/logout` with separate CTF token
- `htb ctf events` to list ongoing and upcoming CTF events
- `htb ctf challenges <event-id>` to browse challenges
- `htb ctf submit/download/start/stop` for challenge interaction
- `htb ctf scoreboard/solves/challenge-solves` for rankings and solve feeds
- Container start polls for ready state and prints the connection URL
- Guards for non-docker and no-download challenges
- Hidden scoreboard detection via menu endpoint
- Cache key collision fix (labs vs CTF paths no longer collide)
- CTF-specific cache TTL tiers

#### HTTP response caching

Cache API responses to disk with TTL-based expiration. Reduces repeated network calls for data that changes infrequently (user profile, machine lists, challenge lists).

- Tiered TTLs: 5 min for profile data, 2 min for lists, 30 s for active/seasonal data
- Automatic invalidation on mutations (start/stop machine, submit flag)
- Cache cleared on login/logout
- `htb cache clear` command and `--no-cache` flag for bypass
- Configurable via `~/.htb-cli/config.toml`

#### MCP stdio server mode

Run `htb --mcp-stdio` to start an MCP server over stdin/stdout for AI agent integration.

Tools exposed: get_user_profile, list_machines, get_machine_info, list_challenges, get_challenge_info, start_challenge, submit_challenge_flag, get_active_machine, list_seasons, search.

#### PwnBox commands

Check PwnBox time quota and active instance status.

- `htb pwnbox usage` shows remaining/used/allowed minutes
- `htb pwnbox status` shows active instance details or "no active instance"

### Fixes

#### Fix HTB account settings URL

The login prompt and README pointed to `/profile/settings` which no longer exists. Updated to `/account-settings`.

#### Fix JSON output broken by pagination footer

`--json` output on paginated commands (machines list, challenges list, sherlocks list) had a plaintext "Page X of Y" line appended after the JSON, breaking parsers. The pagination footer is now suppressed when output format is JSON.

## 0.1.4 (2026-07-21)

### Fixes

- allow dirty Cargo.lock during crates.io publish

## 0.1.3 (2026-07-21)

### Fixes

- handle release creation in workflow instead of knope
- correct canAccessVIP field rename on UserInfo (#8)

## 0.1.2 (2026-07-21)

### Fixes

- merge duplicate with blocks in release workflow
- use correct Saturate CI variable/secret names
- handle missing home directory and deprecated set_var (#5)
- use origin check for auth token in byte downloads (#3)
- respect output format for status command (#4)
- verify todo state before toggling (#6)
- handle type-inconsistent fields from HTB API (#7)
- add required path field to knope asset config
- match knope 0.23.0 dry-run output format
- split knope command step to avoid shell parsing issue
- configure git identity for knope release commit
- push prepare commit before creating release
