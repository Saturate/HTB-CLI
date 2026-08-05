---
default: patch
---

#### Lower cache TTL for detail endpoints

Machine, challenge, and sherlock detail endpoints now cache for 30 seconds
instead of 60 minutes so solve counts, ratings, and active player data stay
fresh.
