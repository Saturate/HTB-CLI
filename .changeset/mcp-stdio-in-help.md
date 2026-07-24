---
default: patch
---

#### Show --mcp-stdio in help output

Moved `--mcp-stdio` from a pre-parse arg check to a proper clap flag so it
appears in `htb --help`.
