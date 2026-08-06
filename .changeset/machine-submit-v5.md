---
default: minor
---

#### Use v5 endpoint for machine flag submission

Switched from `POST /api/v4/machine/{id}/flag` to `POST /api/v5/machine/own`,
matching the current HTB web app. The old endpoint didn't work for seasonal
machines. The `difficulty` parameter is no longer sent.
