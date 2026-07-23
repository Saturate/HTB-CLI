---
default: patch
---

#### Suppress team summary in JSON output and add jq examples

`ctf challenges --json` and `ctf scoreboard --json` no longer print the team summary line before the JSON array. Added jq examples to the README showing how to filter and reshape JSON output.
