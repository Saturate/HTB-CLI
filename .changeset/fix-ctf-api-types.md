---
default: patch
---

#### Fix CTF API deserialization errors

`hasJoined`, `hide_scoreboard`, `docker_online`, and `docker_ports` now handle the types the API actually sends (`null`, integers, arrays) instead of failing on deserialization.
