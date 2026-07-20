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

Sub-API structs borrow the client. Each method maps 1:1 to an endpoint. Base URL: `https://labs.hackthebox.com/api/v4`.

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

1. Should `htb machines list` show paginated results or fetch all? The API supports `machine/paginated?per_page=100` but also non-paginated endpoints.
2. VPN connect/disconnect (managing the OpenVPN process) needs root/sudo on most systems. Should this shell out to `openvpn` or use a helper approach?
3. Should we support shell completions generation (`htb completions bash/zsh/fish`)?
4. The API base URL might differ between `www.hackthebox.com/api/v4` and `labs.hackthebox.com/api/v4`. Need to confirm which is current from your browser calls.
5. Are there any season-specific endpoints you use that aren't covered above?
