# Changelog
## 0.1.2 (2026-07-21)

### Fixes

- merge duplicate with blocks in release workflow
- use correct Saturate CI variable/secret names
- handle missing home directory and deprecated set_var (#5)
- use origin check for auth token in byte downloads (#3)
- respect output format for status command (#4)
- verify todo state before toggling (#6)
- handle type-inconsistent fields from HTB API (#7)
- add required path field to knope asset config
- match knope 0.23.0 dry-run output format
- split knope command step to avoid shell parsing issue
- configure git identity for knope release commit
- push prepare commit before creating release
