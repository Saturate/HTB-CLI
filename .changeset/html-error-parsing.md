---
default: patch
---

#### Parse HTML error responses into friendly messages

When the API returns HTML instead of JSON on errors (e.g. a 403 from nginx),
the CLI now extracts the `<title>` text instead of dumping raw HTML.
