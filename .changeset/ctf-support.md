---
htb-cli: minor
---

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
