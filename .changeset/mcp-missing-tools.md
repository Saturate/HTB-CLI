---
default: minor
---

#### MCP server: add missing machine, sherlock, and VPN tools

Added submit_machine_flag, start_machine, stop_machine, reset_machine, extend_machine,
stop_challenge, list_sherlocks, get_sherlock_info, submit_sherlock_flag, and vpn_status
to the MCP server. Agents using the HTB MCP server can now submit machine flags and
manage machine lifecycle without falling back to raw API calls.
