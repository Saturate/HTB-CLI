# Spec: HTB-CLI

## Objective

A Rust CLI for interacting with the Hack The Box platform from the terminal. Query machines, challenges, seasons, and Sherlocks; spawn/stop instances; submit flags; manage VPN connections. Works with any HTB account (free or VIP).

Target users: pentesters and CTF players who prefer staying in the terminal over the web UI.

Success looks like: `htb machines list --os linux --difficulty easy` returns a colored table in under 2 seconds.

## Tech Stack

- **Language:** Rust (2021 edition)
- **CLI:** clap v4 (derive macros)
- **HTTP:** reqwest + rustls-tls
- **Async:** tokio (full features)
- **Errors:** thiserror (domain) + anyhow (binary boundary)
- **Config:** TOML via `toml` crate
- **Logging:** tracing + tracing-subscriber + EnvFilter
- **Serialization:** serde + serde_json + chrono
- **Output:** colored tables via `comfy-table`, `--json` flag for machine-readable output
- **Versioning:** knope (conventional commits + changesets, semver, changelog generation)
- **CI:** GitHub Actions

## Commands

### Auth

```
htb auth login              # Prompt for API token, save to ~/.htb-cli/.token
htb auth status             # Show current auth state and user info
htb auth logout             # Remove stored token
```

### Machines

```
htb machines list                         # List active machines (table)
htb machines list --retired               # Include retired machines
htb machines list --os linux              # Filter by OS
htb machines list --difficulty easy       # Filter by difficulty
htb machines info <name-or-id>            # Detailed machine info
htb machines start <name-or-id>           # Spawn machine
htb machines stop                         # Stop active machine
htb machines reset <name-or-id>           # Reset machine
htb machines submit <name-or-id> <flag>   # Submit flag (user or root)
htb machines active                       # Show currently active machine
htb machines todo                         # List todo machines
htb machines todo add <name-or-id>        # Add to todo list
htb machines todo remove <name-or-id>     # Remove from todo list
```

### Challenges

```
htb challenges list                          # List all challenges
htb challenges list --category <cat>         # Filter by category
htb challenges categories                    # List categories
htb challenges info <slug>                   # Challenge details
htb challenges download <slug>               # Download challenge files
htb challenges start <slug>                  # Start challenge instance
htb challenges stop <slug>                   # Stop challenge instance
htb challenges submit <id> <flag>            # Submit flag
```

### Seasons

```
htb seasons list                             # List all seasons
htb seasons machines <season-id>             # Machines in a season
htb seasons leaderboard <season-id>          # Season leaderboard
htb seasons rank                             # Your rank in current season
```

### Sherlocks

```
htb sherlocks list                           # List all Sherlocks
htb sherlocks info <slug>                    # Sherlock details
htb sherlocks download <slug>                # Download case files
htb sherlocks tasks <slug>                   # List tasks for a Sherlock
htb sherlocks submit <id> <task-id> <flag>   # Submit task flag
```

### VPN

```
htb vpn status                               # Current connection status
htb vpn list                                 # List available servers
htb vpn switch <server-id>                   # Switch VPN server
htb vpn download [server-id]                 # Download .ovpn file
htb vpn connect [server-id]                  # Download + start OpenVPN (optional feature)
htb vpn disconnect                           # Stop OpenVPN process (optional feature)
```

### User

```
htb user me                                  # Your profile summary
htb user info <username-or-id>               # Another user's profile
htb user activity                            # Your recent activity
```

### Search

```
htb search <query>                           # Global search across machines, challenges, users
```

### Global Flags

```
--json          # Output as JSON instead of table
--no-color      # Disable colored output
--verbose / -v  # Enable debug logging (RUST_LOG=debug)
--config <path> # Override config file path
```

## Build / Dev Commands

```
cargo build                    # Debug build
cargo build --release          # Release build
cargo test                     # Run all tests
cargo clippy -- -D warnings    # Lint
cargo fmt --check              # Format check
```

## Project Structure

```
htb-cli/
  src/
    main.rs                    # Entry point, clap setup, dispatch
    cli/
      mod.rs                   # Top-level CLI enum
      machines.rs              # Machine subcommands
      challenges.rs            # Challenge subcommands
      seasons.rs               # Season subcommands
      sherlocks.rs             # Sherlock subcommands
      vpn.rs                   # VPN subcommands
      user.rs                  # User subcommands
      auth.rs                  # Auth subcommands
      search.rs                # Search subcommand
    api/
      mod.rs                   # HtbClient struct, shared request logic
      machines.rs              # Machine API calls
      challenges.rs            # Challenge API calls
      seasons.rs               # Season API calls
      sherlocks.rs             # Sherlock API calls
      vpn.rs                   # VPN API calls
      user.rs                  # User API calls
      search.rs                # Search API calls
    models/
      mod.rs                   # Re-exports
      machine.rs               # Machine response types
      challenge.rs             # Challenge response types
      season.rs                # Season response types
      sherlock.rs              # Sherlock response types
      vpn.rs                   # VPN response types
      user.rs                  # User response types
    output/
      mod.rs                   # OutputFormat enum, format dispatch
      table.rs                 # Table rendering (comfy-table)
      json.rs                  # JSON output
    config.rs                  # Config loading (~/.htb-cli/config.toml)
    error.rs                   # thiserror domain errors
  tests/
    api_integration.rs         # Integration tests against mock server
  .github/
    workflows/
      ci.yml                   # Clippy, test, fmt on PR/push
      release.yml              # knope release + binary builds
  .changeset/                  # knope changeset files
  knope.toml                   # knope config
  Cargo.toml
  CHANGELOG.md
  LICENSE
  README.md
```

Single flat crate. Modules provide the separation; no workspace needed for a focused CLI tool.

## Code Style

```rust
use clap::{Parser, Subcommand};
use crate::api::HtbClient;
use crate::error::HtbError;
use crate::output::OutputFormat;

#[derive(Subcommand)]
pub enum MachineCommand {
    List {
        #[arg(long)]
        retired: bool,
        #[arg(long)]
        os: Option<String>,
        #[arg(long)]
        difficulty: Option<String>,
    },
    Info {
        name_or_id: String,
    },
    Start {
        name_or_id: String,
    },
    Stop,
    Reset {
        name_or_id: String,
    },
    Submit {
        name_or_id: String,
        flag: String,
    },
    Active,
}

pub async fn handle(client: &HtbClient, cmd: MachineCommand, format: OutputFormat) -> anyhow::Result<()> {
    match cmd {
        MachineCommand::List { retired, os, difficulty } => {
            let machines = client.machines().list(retired).await?;
            // filter and output
        }
        // ...
    }
    Ok(())
}
```

- Derive macros for clap, serde
- One handler function per subcommand module
- `HtbClient` holds the reqwest client + base URL + auth token
- Domain errors via `HtbError` (thiserror), anyhow at the handler boundary

## API Client Design

```rust
pub struct HtbClient {
    http: reqwest::Client,
    base_url: String,
    token: String,
}

impl HtbClient {
    pub fn machines(&self) -> MachineApi<'_> { MachineApi(self) }
    pub fn challenges(&self) -> ChallengeApi<'_> { ChallengeApi(self) }
    pub fn seasons(&self) -> SeasonApi<'_> { SeasonApi(self) }
    pub fn sherlocks(&self) -> SherlockApi<'_> { SherlockApi(self) }
    pub fn vpn(&self) -> VpnApi<'_> { VpnApi(self) }
    pub fn user(&self) -> UserApi<'_> { UserApi(self) }
}
```

Sub-API structs borrow the client. Each method maps 1:1 to an endpoint. Base URL: `https://labs.hackthebox.com/api/v4` (some endpoints use `/api/v5`).

## Rate Limiting

The API returns rate limit headers on every response:
- `x-ratelimit-limit`: max requests per window
- `x-ratelimit-remaining`: requests left in current window

Limits vary by endpoint (15-60 per window). The client tracks remaining quota
from response headers and delays requests when nearing the limit. No need to
spam the API when we know the ceiling.

```rust
pub struct RateLimitState {
    remaining: AtomicU32,
    limit: AtomicU32,
}
```

On each response, update from headers. Before each request, check remaining > 0.
If exhausted, wait and retry with backoff (reuse the exponential backoff pattern
from ridgeline's AzureDevOpsClient). Surface the limit to the user:
`Rate limited (14/25 remaining)` in verbose mode.

## Config

`~/.htb-cli/config.toml`:

```toml
# Default output format (table or json)
output = "table"

# Default VPN server ID
vpn_server = 1

# Disable colored output
no_color = false
```

Token stored separately at `~/.htb-cli/.token` (plaintext, 0o600 permissions).

## Testing Strategy

- **Unit tests:** serde deserialization of API responses (use recorded JSON fixtures in `tests/fixtures/`)
- **Integration tests:** mock HTTP server (wiremock-rs) to test full request/response cycle
- **CLI tests:** assert-cmd for end-to-end command parsing and output format
- **No live API tests in CI** (requires auth token)

## CI: GitHub Actions

### ci.yml (on push + PR)

1. Matrix: stable + nightly Rust, ubuntu-latest + macos-latest
2. Steps: checkout, cache, fmt check, clippy, test
3. Fail on warnings

### release.yml (on workflow_dispatch or knope-triggered tag)

1. Build release binaries (linux-x86_64, linux-aarch64, macos-x86_64, macos-aarch64)
2. Create GitHub release with binaries attached
3. Generate/update CHANGELOG.md via knope

### Knope workflow

- Developers create changeset files in `.changeset/` describing changes
- `knope release` bumps version in Cargo.toml, updates CHANGELOG.md, tags, creates GitHub release
- GitHub Actions runs `knope release` on dispatch

## Boundaries

- **Always:** Run clippy + tests before commits. Validate API responses (don't unwrap). Handle rate limiting gracefully.
- **Ask first:** Adding new content types beyond v1.0 scope. Changing output format defaults. Adding interactive prompts.
- **Never:** Store tokens in config.toml. Log tokens or API responses containing tokens. Make destructive API calls without confirmation (machine reset gets a prompt).

## Success Criteria

- `htb machines list` returns results in < 2s on a normal connection
- `htb machines info <name>` shows machine details with difficulty, OS, own status, blood times
- `htb machines start <name>` spawns the machine and reports the IP
- `htb challenges submit <id> <flag>` submits and shows result
- `--json` flag works on every list/info command
- `htb auth login` stores token and `htb auth status` confirms it works
- CI passes on every PR (clippy clean, tests green, fmt clean)
- knope produces a valid CHANGELOG.md entry on release

## Open Questions

1. VPN connect/disconnect (managing the OpenVPN process) needs root/sudo on most systems. Should this shell out to `openvpn` or use a helper approach?
2. Should we support shell completions generation (`htb completions bash/zsh/fish`)?

---

# Feature Spec: CTF Platform Support

## Objective

Add support for the HTB CTF platform (`ctf.hackthebox.com`) alongside the existing labs platform. CTF players can list events, browse challenges within an event, start/stop containers, submit flags, download challenge files, view scoreboards, and check solves - all from the terminal.

The CTF platform runs on a separate domain with its own API (`/api/` prefix, no version), separate JWT auth (audience `"2"`), and a team-centric model where solves are attributed to teams.

Target users: CTF players who want to interact with HTB CTF events without switching to the browser during a competition.

## API Surface

Based on HAR analysis of `ctf.hackthebox.com`. All endpoints use Bearer JWT auth and the `/api/` prefix (no version).

### Discovery

| Method | Path | Returns |
|---|---|---|
| `GET` | `/api/ctfs` | Array of CTF events with status, dates, team info |
| `GET` | `/api/ctfs/details/{slug}` | Event details (description, prizes, player counts) |
| `GET` | `/api/ctfs/{id}` | Event + participating team + full challenge list |
| `GET` | `/api/ctfs/{id}/menu` | Permissions and nav info |
| `GET` | `/api/public/challenge-categories` | All 32 challenge categories |

### Scoreboard

| Method | Path | Returns |
|---|---|---|
| `GET` | `/api/ctfs/scores/{ctf_id}` | Top 100 teams + your team's rank |
| `GET` | `/api/ctfs/score-charts/{ctf_id}` | Time-series score progression (top 10), summary cards |
| `GET` | `/api/ctfs/solves/{ctf_id}` | Recent solves feed across the event |

### Challenge interaction

| Method | Path | Returns |
|---|---|---|
| `POST` | `/api/flags/own` | Submit flag (body: `{challenge_id, flag}`) |
| `GET` | `/api/challenges/{id}/solves` | Per-challenge team leaderboard |
| `GET` | `/api/challenges/{id}/download` | Download challenge ZIP (binary) |
| `GET` | `/api/challenges/{id}/download/link` | Signed download URL |
| `POST` | `/api/challenges/containers/start` | Start Docker instance (body: `{id}`) |
| `POST` | `/api/challenges/containers/stop` | Stop Docker instance (body: `{id}`) |

### Team coordination

| Method | Path | Returns |
|---|---|---|
| `GET` | `/api/challenges/{id}/associations` | Team member assignment list |
| `GET` | `/api/challenges/{id}/associate/{user_id}` | Assign/toggle member on challenge |
| `POST` | `/api/challenges/{id}/progress` | Set status: not started / in progress / need help |

### User

| Method | Path | Returns |
|---|---|---|
| `GET` | `/api/users/profile` | Current user (CTF context) |
| `GET` | `/api/users/profile/details/{id}` | Public profile with CTF stats |

### Not in scope for v1

- Team management (create/join/leave)
- Team chat (`/api/chat/`)
- CTF join/leave flow
- Notifications
- Organization settings

## Design

### Auth

CTF uses a separate JWT with audience `"2"`, obtained from the CTF platform login. Store alongside the labs token:

```
~/.htb-cli/.token        # labs token (existing)
~/.htb-cli/.ctf-token    # CTF platform token
```

`htb ctf auth login` prompts for the CTF token separately. The CTF token can be generated from the CTF platform's settings page, or the user pastes a JWT.

### Client reuse

`HtbClient` already supports configurable `base_url`. The CTF client is a second `HtbClient` instance with:
- `base_url`: `https://ctf.hackthebox.com`
- `token`: CTF-specific JWT
- Same rate limiting, caching, and error handling

A factory function builds it:

```rust
fn ctf_client(cache: Arc<Cache>) -> anyhow::Result<HtbClient> {
    let token = config::read_ctf_token()?;
    Ok(HtbClient::with_base_url_and_cache(
        token,
        "https://ctf.hackthebox.com".to_string(),
        cache,
    ))
}
```

### HtbClient additions

**`post_no_content`**: `POST /api/challenges/{id}/progress` returns 204 with no body. The existing `post` method tries to deserialize JSON and would fail. Add a `post_no_content` method that sends the request and returns `Ok(())` on 2xx without parsing a body.

**Multi-flag challenges**: Challenges can have multiple flags (`flagsInfo` array with `flag_id`, `identifier`, `question`, `solved`). The `submit` command needs to accept an optional `--flag-id` for multi-flag challenges. If omitted on a multi-flag challenge, list the available flags and prompt. Needs testing against a live multi-flag challenge to confirm whether `POST /api/flags/own` accepts a `flag_id` field.

### CLI structure

New top-level `Ctf` subcommand. Mirrors the existing pattern of subcommand modules.

```
htb ctf auth login                          # Save CTF API token
htb ctf auth status                         # Show CTF auth state
htb ctf auth logout                         # Remove CTF token

htb ctf events                              # List CTF events (ongoing, upcoming)
htb ctf events --all                        # Include past events
htb ctf info <slug>                         # Event details

htb ctf challenges <event-id>               # List challenges in event
htb ctf challenges <event-id> --category <cat>  # Filter by category

htb ctf download <event-id> <challenge-id>   # Download challenge files
htb ctf start <event-id> <challenge-id>     # Start challenge container
htb ctf stop <event-id> <challenge-id>      # Stop challenge container
htb ctf submit <challenge-id> <flag>        # Submit flag

htb ctf scoreboard <event-id>               # Top teams + your rank (checks hide_scoreboard)
htb ctf solves <event-id>                   # Recent solves feed
htb ctf challenge-solves <challenge-id>     # Per-challenge team leaderboard
```

Challenge commands take IDs directly (not slugs) because the CTF API uses numeric IDs and challenges are listed from an event context, so the user always sees the ID first.

Guards: `start` checks `hasDocker` before calling the API; `download` checks `filename` is present. Both produce a clear error if the challenge doesn't support the operation.

### Models

New module `src/models/ctf.rs` with CTF-specific types. The CTF API returns different shapes than the labs API, so separate models are cleaner than trying to unify:

```rust
pub struct CtfEvent { ... }          // from GET /api/ctfs
pub struct CtfEventDetail { ... }    // from GET /api/ctfs/details/{slug}
pub struct CtfEventData { ... }      // from GET /api/ctfs/{id}
pub struct CtfChallenge { ... }      // nested in CtfEventData
pub struct CtfFlagInfo { ... }       // nested in CtfChallenge
pub struct CtfScoreboard { ... }     // from GET /api/ctfs/scores/{id}
pub struct CtfTeamScore { ... }      // items in scoreboard
pub struct CtfSolve { ... }          // from GET /api/ctfs/solves/{id}
pub struct CtfChallengeSolve { ... } // from GET /api/challenges/{id}/solves
pub struct CtfFlagResult { ... }     // from POST /api/flags/own
pub struct CtfUserProfile { ... }    // from GET /api/users/profile
```

### Caching

Reuse the existing cache infrastructure. Cache keys must include the base URL to avoid collisions between labs and CTF (e.g. both have `/api/users/profile`). The `Cache::set`/`get` methods already receive the full URL including base, so the sanitized filename naturally separates them (`ctf.hackthebox.com_api_users_profile` vs `labs.hackthebox.com_api_users_profile`). The `ttl_for_path` method needs platform awareness; solved by matching on the base_url or the path prefix patterns.

CTF-specific TTL rules in `ttl_for_path`:

| Path pattern | TTL | Rationale |
|---|---|---|
| `/api/ctfs` (event list) | 5 min | Events don't change often |
| `/api/ctfs/details/*` | 5 min | Static event info |
| `/api/ctfs/{id}` (challenges) | 30 s | Challenge solve counts update live |
| `/api/ctfs/scores/*` | 30 s | Scoreboard changes during active CTF |
| `/api/ctfs/solves/*` | 30 s | Solves feed updates frequently |
| `/api/public/challenge-categories` | 30 min | Reference data |
| `/api/users/profile*` | 2 min | Profile data |
| POST endpoints, downloads | not cached | Mutations and binary data |

Invalidation after POST: `flags/own` and `containers/start`/`stop` clear `api_ctfs_` and `api_challenges_` prefixed cache entries.

### File structure

```
src/
  api/
    ctf.rs              # CtfApi struct with endpoint methods
  cli/
    ctf.rs              # CtfCommand enum, handle(), subcommand dispatch
  models/
    ctf.rs              # CTF response types + Tabular impls
  config.rs             # add read_ctf_token, save_ctf_token, ctf_token_path
  cli/mod.rs            # add Ctf variant to Command enum
  api/mod.rs            # add ctf module, TTL rules for CTF paths
  main.rs               # add Ctf dispatch arm
```

No new dependencies.

### Container status polling

The CTF API has no explicit container status endpoint. Container state (`docker_online`, `docker_ports`) is returned as fields on the challenge object within `GET /api/ctfs/{id}`. After `start`, poll `GET /api/ctfs/{id}` until `docker_online` is populated, with a timeout. Display the connection URL when ready.

```
$ htb ctf start 40790
Container starting...
Ready: 83.136.254.199:31337
```

### Edge cases

- **Hidden scoreboard**: Check `GET /api/ctfs/{id}/menu` for `userCanViewScoreboard` before hitting the scores endpoint. Show "Scoreboard is hidden for this event" if disabled.
- **Challenges without containers**: `hasDocker` is `0` or `null`. `htb ctf start` on these returns "This challenge doesn't use a container."
- **Challenges without downloads**: `filename` is `null`. `htb ctf download` returns "No files available for this challenge."
- **Multi-flag challenges**: `flagsInfo` has multiple entries. Show flag questions with `htb ctf challenges` output. Accept `--flag-id` on submit.

## Testing Strategy

- **Unit tests:** Deserialize CTF API responses from JSON fixtures in `tests/fixtures/ctf/`. One fixture per endpoint shape.
- **Integration tests:** wiremock-based tests for the full request/response cycle, following the existing `tests/cache_integration.rs` pattern.
- **No live API tests in CI.**

Fixtures derived from the HAR captures, with sensitive data (tokens, real usernames) scrubbed.

## Boundaries

- **Always:** Use the CTF token for CTF endpoints (never the labs token). Respect rate limits. Cache appropriately. Guard commands against unsupported challenge types.
- **Ask first:** Adding team management, chat, or CTF join/leave. Adding MCP tool exposure for CTF endpoints.
- **Never:** Mix labs and CTF auth tokens. Store CTF tokens in config.toml. Auto-join CTF events.

## Success Criteria

- `htb ctf events` lists ongoing and upcoming CTF events in < 2s
- `htb ctf challenges <event-id>` shows all challenges with category, points, difficulty, solved status
- `htb ctf submit <id> <flag>` submits and shows result with points earned
- `htb ctf scoreboard <event-id>` shows top teams and your team's rank/position
- `htb ctf scoreboard` on an event with `hide_scoreboard=1` shows a clear message
- `htb ctf start <id>` starts a container and reports the connection URL when ready
- `htb ctf start` on a non-Docker challenge produces a clear error
- `htb ctf download <id>` saves the challenge ZIP to the current directory
- `htb ctf download` on a challenge without files produces a clear error
- `--json` works on all CTF list/info/scoreboard commands
- All existing labs tests pass unchanged
- CTF auth is independent from labs auth

## Open Questions

1. Does `POST /api/flags/own` accept a `flag_id` field for multi-flag challenges, or does it match purely on the flag string? Needs testing against a live multi-flag challenge.
2. Is there a container status polling endpoint we haven't captured, or is `GET /api/ctfs/{id}` the only way to check `docker_online`/`docker_ports`?

---

# Feature Spec: HTTP Response Caching

## Objective

Reduce redundant API calls and speed up repeated CLI commands. A user running `htb machines info Bedside` then `htb machines start Bedside` hits the profile endpoint twice. With caching, the second call reads from disk.

The HTB API sends `cache-control: no-cache, private` on all endpoints with no ETags or Last-Modified. The frontend does no client-side caching either (no localStorage, no IndexedDB, no Pinia persistence). So this is purely client-side TTL caching. Evaluated `http-cache-reqwest`, `cacache`, and `moka` crates; none fit (RFC-compliant caching fights `no-cache` headers, and in-memory caches don't survive CLI process exits).

## Design

### Cache location

`~/.htb-cli/cache/` (follows existing `~/.htb-cli/` convention).

### Cache key

Sanitized URL path as filename. Strip the base URL, replace `/` with `_`, drop query string `?` and `&` to `_`. Example: `/api/v4/machine/profile/Bedside` becomes `api_v4_machine_profile_Bedside.json`. This makes files debuggable with `ls` and enables glob-based prefix invalidation.

### Cache entry format

```json
{
  "cached_at": 1784657660,
  "body": "..."
}
```

`body` stores the raw JSON response string. Deserialization happens after cache lookup, same as a live response.

### TTL tiers

| Endpoint pattern | TTL | Rationale |
|---|---|---|
| `/machine/profile/*`, `/challenge/info/*`, `/sherlocks/*` | 2 min | Profile data changes rarely within a session |
| `/machines?*`, `/challenges?*`, `/sherlocks?*` (list endpoints) | 5 min | Lists change infrequently |
| `/challenge/categories/list`, `/sherlocks/categories/list`, `/season/list`, `/tags/list` | 30 min | Reference data |
| `/user/info`, `/user/profile/*` | 2 min | Points/rank can change after submissions |
| Everything else | not cached | Active VM status, VPN, search, connections |

No user-configurable TTL override. The tiers are hardcoded; `--no-cache` and `cache.enabled = false` are sufficient controls.

### What is never cached

- POST requests (mutations: spawn, terminate, submit flag, etc.)
- `/virtual_machine/active` (must reflect real-time state)
- `/connection/status`, `/connections` (VPN state)
- `/search/*` (query-dependent, low repeat rate)
- Download URLs and binary responses

### Cache invalidation

- **TTL expiry**: stale entries are re-fetched and overwritten.
- **After mutations**: POST requests to machine/challenge endpoints clear related cache entries by glob prefix. `vm/spawn`, `vm/terminate`, `vm/reset` clear `api_v*_machine*` and `api_v*_machines*` (both profile and list). Challenge start/stop clears `api_v*_challenge*`.
- **Manual**: `htb cache clear` command wipes `~/.htb-cli/cache/`.
- **Auth change**: `htb auth login` and `htb auth logout` clear the cache directory.

### Disk cleanup

Lazy sweep: on every Nth cache write (e.g. every 10th), scan the cache directory and delete files with `cached_at` older than 1 hour. This handles long-lived MCP server sessions where the constructor sweep won't re-run.

### Atomicity and error handling

- **Atomic writes**: write to a tempfile in the cache directory, then `rename` into place. Prevents concurrent CLI invocations from reading half-written JSON.
- **Corrupt files**: if a cache file fails to parse, treat as a cache miss and delete the file.
- **Cache dir failure**: if `~/.htb-cli/cache/` can't be created (permissions, read-only FS), degrade to no-cache silently (log at debug level).
- **File permissions**: cache files get `0o600` on Unix, same as the token file.
- **Clock skew**: treat `cached_at` in the future as expired.

### Configuration

Add to `~/.htb-cli/config.toml`:

```toml
[cache]
enabled = true     # default: true
```

A `--no-cache` global CLI flag bypasses cache for a single invocation.

## Implementation

### New module: `src/cache.rs`

```rust
pub struct Cache {
    dir: PathBuf,
    enabled: bool,
    write_count: AtomicU32,
}

impl Cache {
    pub fn new(dir: PathBuf, enabled: bool) -> Self;
    pub fn get(&self, url: &str, max_age: Duration) -> Option<String>;
    pub fn set(&self, url: &str, body: &str);
    pub fn invalidate_pattern(&self, glob: &str);
    pub fn clear(&self);
}
```

Constructor takes a `PathBuf` so tests can pass a temp directory. All methods are infallible from the caller's perspective; errors log at debug and degrade to no-cache.

### Integration into HtbClient

Add a private `get_raw(&self, path: &str, max_age: Duration) -> Result<String>` method that checks cache before HTTP. The existing `get<T: DeserializeOwned>` delegates to `get_raw` then deserializes. No signature change to public API; all `src/api/*.rs` call sites remain unchanged.

`HtbClient` gains an `Option<Cache>` field. `None` means caching is disabled. A `ttl_for_path(path: &str) -> Option<Duration>` helper maps URL patterns to TTL tiers; returns `None` for uncached endpoints.

POST methods call `cache.invalidate_pattern()` after successful responses.

### Test constructor

Add `HtbClient::with_cache(token, base_url, cache)` for tests, following the existing `with_base_url` pattern.

### Files changed

```
src/cache.rs          # new: Cache struct, ~100 lines
src/api/mod.rs        # add get_raw, Option<Cache> field, ttl_for_path
src/cli/mod.rs        # add --no-cache flag, cache clear subcommand
src/cli/cache.rs      # new: cache subcommand handler
src/config.rs         # add CacheConfig
src/cli/auth.rs       # clear cache on login/logout
```

No new dependencies.

## Testing Strategy

- Unit tests for `Cache` in isolation using `tempfile::tempdir()` (already a transitive dep)
- Test TTL expiry by writing entries with `cached_at` in the past, not by sleeping
- Test atomic writes by verifying cache files are either absent or valid JSON
- Test corrupt file recovery by writing invalid JSON to a cache file
- Existing tests pass unchanged (cache is transparent; test clients use `with_base_url` which has no cache)

## Boundaries

- **Always**: respect `--no-cache` flag, never cache POST responses, atomic writes
- **Ask first**: changing default TTL values, adding new cached endpoints
- **Never**: cache auth tokens, serve stale data for active VM state

## Success Criteria

- `htb machines info X && htb machines start X` makes one profile HTTP request, not two
- `htb machines list` repeated within 5 minutes serves from cache
- `htb cache clear` removes all cached data
- `--no-cache` bypasses cache completely
- Spawning a machine invalidates both profile and list caches
- No behavioral change for mutating commands
- All existing tests pass without modification
