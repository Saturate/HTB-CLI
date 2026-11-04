---
default: patch
---

#### Update dependencies

Bump rpassword from v5 to v7 and update lockfile to resolve 12 security
advisories in transitive dependencies (quinn-proto, rustls, rustls-webpki,
webpki-roots, tokio, chacha20).
