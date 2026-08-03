---
default: minor
---

#### Add seasons machines/leaderboard, sherlocks tasks, user info subcommands

- `seasons machines <id>` — list machines in a season
- `seasons leaderboard <id>` — show season leaderboard
- `sherlocks tasks <slug>` — list tasks for a sherlock
- `user info <username>` — look up another user's profile by name or ID
- `rankings users` — show global user leaderboard

Also fixed pre-existing bugs discovered via live API testing:
- sherlocks info/tasks: wrapped in `{"data": ...}`, field names were wrong
- seasons leaderboard: wrong API path and model fields
- user info search: API returns `"value"` not `"name"`
