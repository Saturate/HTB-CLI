---
default: minor
---

#### HTTP response caching

Cache API responses to disk with TTL-based expiration. Reduces repeated network calls for data that changes infrequently (user profile, machine lists, challenge lists).

- Tiered TTLs: 5 min for profile data, 2 min for lists, 30 s for active/seasonal data
- Automatic invalidation on mutations (start/stop machine, submit flag)
- Cache cleared on login/logout
- `htb cache clear` command and `--no-cache` flag for bypass
- Configurable via `~/.htb-cli/config.toml`
