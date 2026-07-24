---
default: minor
---

#### Sticky CTF event selection

Added `htb ctf use <event_id>` to persist the active CTF event to config.
All event-scoped commands now accept the event ID as optional, falling back
to the stored value.
