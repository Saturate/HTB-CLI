# htb-cli

A terminal client for [Hack The Box](https://hackthebox.com). Browse machines, challenges, Sherlocks, and seasons. Spawn instances, submit flags, manage VPN connections.

Works with any HTB account (free or VIP).

## Install

```bash
cargo install htb-cli
```

Or download a binary from [releases](https://github.com/Saturate/HTB-CLI/releases).

## Setup

```bash
# Save your API token (from https://app.hackthebox.com/account-settings)
htb auth login

# Verify it works
htb auth status
```

## Usage

```bash
# Machines
htb machines list --os linux --difficulty easy
htb machines info Bedside
htb machines start Bedside
htb machines submit Bedside 'HTB{flag_here}'
htb machines active

# Challenges
htb challenges list --category Web
htb challenges categories
htb challenges info Poly
htb challenges start Poly
htb challenges submit 112 'HTB{flag_here}'
htb challenges download Poly

# Sherlocks
htb sherlocks list --category DFIR
htb sherlocks info Brutus

# Seasons
htb seasons list
htb seasons rank

# VPN
htb vpn status
htb vpn list
htb vpn switch 1
htb vpn download

# User
htb user me
htb user info 1234567

# Search
htb search nmap
```

## Output

Table output by default. Use `--json` on any command for machine-readable output:

```bash
htb machines list --json
htb auth status --json
```

## MCP Server

Run as an [MCP](https://modelcontextprotocol.io) server for AI agent integration:

```bash
htb --mcp-stdio
```

Exposes tools: `list_machines`, `get_machine_info`, `list_challenges`, `get_challenge_info`, `start_challenge`, `submit_challenge_flag`, `get_active_machine`, `list_seasons`, `search`, `get_user_profile`.

### Claude Code config

```json
{
  "mcpServers": {
    "htb": {
      "command": "htb",
      "args": ["--mcp-stdio"]
    }
  }
}
```

## Config

Optional config at `~/.htb-cli/config.toml`:

```toml
output = "table"   # or "json"
vpn_server = 1     # default VPN server ID
no_color = false
```

Token stored at `~/.htb-cli/.token` (0600 permissions).

## License

MIT
