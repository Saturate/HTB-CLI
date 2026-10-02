---
default: minor
---

#### MCP server: add missing machine, sherlock, and VPN tools

Added submit_machine_flag, start_machine, stop_machine, reset_machine, extend_machine,
stop_challenge, list_sherlocks, get_sherlock_info, submit_sherlock_flag, and vpn_status
to the MCP server. Agents using the HTB MCP server can now submit machine flags and
manage machine lifecycle without falling back to raw API calls.

#### Fix season machine listings crashing on unrevealed machines

Season endpoints return placeholder entries for machines not yet revealed (no id or name).
These are now filtered out instead of causing a deserialization error.

#### Security: update h2 and rustls

h2 0.4.15 -> 0.4.19 (RUSTSEC-2026-0258), rustls 0.23.42 -> 0.23.45 (RUSTSEC-2026-0285).
