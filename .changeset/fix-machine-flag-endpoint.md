---
default: patch
---

#### Fix machine flag submission endpoint

Use `POST /api/v4/machine/{id}/flag` instead of `/api/v4/machine/own` for submitting machine flags. The old endpoint fails for Release Arena machines.
