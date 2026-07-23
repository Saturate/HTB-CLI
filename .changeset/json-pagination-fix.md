---
htb-cli: patch
---

#### Fix JSON output broken by pagination footer

`--json` output on paginated commands (machines list, challenges list, sherlocks list) had a plaintext "Page X of Y" line appended after the JSON, breaking parsers. The pagination footer is now suppressed when output format is JSON.
