---
default: patch
---

#### Add missing serde defaults on optional fields

Added `#[serde(default)]` to optional fields in `ActiveVmResponse`,
`PaginationLinks`, and `PaginationMeta` that were missing the attribute.
Prevents deserialization failures when the HTB API omits a key entirely
instead of returning null.
