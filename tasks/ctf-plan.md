# Plan: CTF Platform Support

Implements the "Feature Spec: CTF Platform Support" section of SPEC.md.

## Dependency graph

```
config (CTF token CRUD)
    └── HtbClient additions (post_no_content)
            └── CTF models (structs + Tabular impls)
                    └── CTF API module (CtfApi)
                            ├── CLI: auth (login/status/logout)
                            ├── CLI: events + info
                            ├── CLI: challenges list
                            ├── CLI: submit, start, stop, download
                            └── CLI: scoreboard, solves
                                    └── Cache (TTL rules, key fix)
                                            └── Tests (fixtures + unit + integration)
```

---

## Task 1: Foundation - config and client additions ✅

**Description:** Add CTF token storage to config (read/save/remove, stored at `~/.htb-cli/.ctf-token` with 0o600 permissions), add `post_no_content` method to `HtbClient` for endpoints returning 204, and wire up the empty `Ctf` command variant in `cli/mod.rs`, `main.rs`, and `lib.rs`.

**Acceptance criteria:**
- [ ] `config::ctf_token_path()`, `read_ctf_token()`, `save_ctf_token()`, `remove_ctf_token()` functions exist
- [ ] `HtbClient::post_no_content` sends POST, calls `invalidate_after_post`, and returns `Ok(())` on 2xx without parsing body
- [ ] `Command::Ctf` variant exists in the CLI enum (can be a stub that prints "not yet implemented")

**Verification:**
- [ ] `cargo test` passes
- [ ] `cargo clippy` clean
- [ ] `cargo build` succeeds

**Dependencies:** None

**Files likely touched:**
- `src/config.rs`
- `src/api/mod.rs`
- `src/cli/mod.rs`
- `src/main.rs`

**Estimated scope:** M (4 files)

---

## Task 2: CTF auth commands

**Description:** Implement `htb ctf auth login`, `htb ctf auth status`, and `htb ctf auth logout`. Login prompts for a CTF API token and saves it. Status reads the token and calls `GET /api/users/profile` on the CTF platform to show the authenticated user. Logout removes the token. Clear cache on login/logout.

**Acceptance criteria:**
- [ ] `htb ctf auth login` prompts for token and saves to `~/.htb-cli/.ctf-token`
- [ ] `htb ctf auth status` shows CTF username and auth state
- [ ] `htb ctf auth logout` removes the CTF token
- [ ] Cache is cleared on login/logout

**Verification:**
- [ ] `cargo build` succeeds
- [ ] Manual: `htb ctf auth login` with a real token, then `htb ctf auth status` shows profile

**Dependencies:** Task 1

**Files likely touched:**
- `src/cli/ctf.rs` (new)
- `src/models/ctf.rs` (new, just `CtfUserProfile` for now)
- `src/api/ctf.rs` (new, just `profile()` for now)
- `src/api/mod.rs` (add ctf module)
- `src/models/mod.rs` (add ctf module)

**Estimated scope:** M (5 files, 3 new)

---

## Checkpoint: After Tasks 1-2
- [ ] All tests pass
- [ ] `htb ctf auth login` / `status` / `logout` works end-to-end
- [ ] CTF token stored separately from labs token

---

## Task 3: CTF events listing and event details

**Description:** Implement `htb ctf events` (list CTF events, default to ongoing+upcoming, `--all` for past), `htb ctf info <slug>` (event details). Add `CtfEvent` and `CtfEventDetail` models with Tabular impls. Add API methods for `GET /api/ctfs` and `GET /api/ctfs/details/{slug}`.

**Acceptance criteria:**
- [ ] `htb ctf events` shows a table with name, status, dates, team size, players
- [ ] `htb ctf events --all` includes past events
- [ ] `htb ctf info <slug>` shows event details (description, format, prizes, player/team counts)
- [ ] `--json` works on both commands

**Verification:**
- [ ] `cargo test` passes
- [ ] `cargo clippy` clean
- [ ] Manual: run against live CTF platform

**Dependencies:** Task 2

**Files likely touched:**
- `src/models/ctf.rs` (add CtfEvent, CtfEventDetail)
- `src/api/ctf.rs` (add events(), event_details())
- `src/cli/ctf.rs` (add Events, Info commands)

**Estimated scope:** M (3 files, CtfEvent has 20+ fields)

---

## Task 4: CTF challenges listing

**Description:** Implement `htb ctf challenges <event-id>` to list all challenges in a CTF event, with optional `--category` filter. Uses `GET /api/ctfs/{id}` which returns the event data including the full challenge list. Add `CtfEventData`, `CtfChallenge`, `CtfFlagInfo`, and `CtfParticipatingTeam` models.

**Acceptance criteria:**
- [ ] `htb ctf challenges <event-id>` shows a table with name, category, difficulty, points, solves, solved status, hasDocker, hasDownload
- [ ] `--category <cat>` filters by category name (case-insensitive)
- [ ] `--json` outputs the challenge list as JSON
- [ ] Multi-flag challenges show flag count in the table

**Verification:**
- [ ] `cargo test` passes
- [ ] Manual: run against live CTF event with challenges

**Dependencies:** Task 3

**Files likely touched:**
- `src/models/ctf.rs` (add CtfEventData, CtfChallenge, CtfFlagInfo, CtfParticipatingTeam)
- `src/api/ctf.rs` (add event_data())
- `src/cli/ctf.rs` (add Challenges command)

**Estimated scope:** M (3 files, models are the bulk)

---

## Checkpoint: After Tasks 3-4
- [ ] All tests pass
- [ ] Can browse CTF events and challenges end-to-end
- [ ] Table output looks clean with real data

---

## Task 5: Challenge interaction - submit, download, start, stop

**Description:** Implement flag submission (`htb ctf submit`), file download (`htb ctf download`), and container start/stop (`htb ctf start`, `htb ctf stop`). Download and start/stop take both event ID and challenge ID since the CTF API needs the event context for polling container status and for validating challenge capabilities. Start checks `hasDocker` via `GET /api/ctfs/{event_id}` challenge data. Download checks `filename`. Start polls `GET /api/ctfs/{event_id}` for `docker_online`/`docker_ports` after container launch, with timeout. Add `CtfFlagResult` model.

CLI shape: `htb ctf start <event-id> <challenge-id>`, `htb ctf stop <event-id> <challenge-id>`, `htb ctf download <event-id> <challenge-id>`. Submit only needs the challenge ID since it doesn't need event context: `htb ctf submit <challenge-id> <flag>`.

**Acceptance criteria:**
- [ ] `htb ctf submit <challenge-id> <flag>` submits flag and shows result with points
- [ ] `htb ctf download <event-id> <challenge-id>` downloads ZIP to current directory
- [ ] `htb ctf download` on challenge without files shows clear error
- [ ] `htb ctf start <event-id> <challenge-id>` starts container and shows connection URL when ready
- [ ] `htb ctf start` on non-Docker challenge shows clear error
- [ ] `htb ctf stop <event-id> <challenge-id>` stops the container
- [ ] Container start polls for ready state with timeout

**Verification:**
- [ ] `cargo test` passes
- [ ] Manual: submit a flag, download a challenge, start/stop a container

**Dependencies:** Task 4 (needs CtfChallenge model for guards, event_data() for polling)

**Files likely touched:**
- `src/models/ctf.rs` (add CtfFlagResult)
- `src/api/ctf.rs` (add submit_flag, download, download_link, container_start, container_stop)
- `src/cli/ctf.rs` (add Submit, Download, Start, Stop commands + polling logic)

**Estimated scope:** M (3 files)

---

## Task 6: Scoreboard and solves

**Description:** Implement `htb ctf scoreboard <event-id>` (top teams + your rank), `htb ctf solves <event-id>` (recent solves feed), and `htb ctf challenge-solves <challenge-id>` (per-challenge team leaderboard). Scoreboard checks `userCanViewScoreboard` from `GET /api/ctfs/{id}/menu` before fetching. Add remaining models.

**Acceptance criteria:**
- [ ] `htb ctf scoreboard <event-id>` shows top teams table with rank, name, points, flags, first bloods
- [ ] Scoreboard shows your team's position separately
- [ ] `htb ctf scoreboard` on hidden-scoreboard event shows clear message
- [ ] `htb ctf solves <event-id>` shows recent solves with team, challenge, category, time
- [ ] `htb ctf challenge-solves <challenge-id>` shows per-challenge team leaderboard
- [ ] `--json` works on all three

**Verification:**
- [ ] `cargo test` passes
- [ ] Manual: run scoreboard and solves against a live event

**Dependencies:** Task 4 (needs event_data for menu check)

**Files likely touched:**
- `src/models/ctf.rs` (add CtfScoreboard, CtfTeamScore, CtfSolve, CtfChallengeSolve, CtfMenu)
- `src/api/ctf.rs` (add scoreboard, score_charts, solves, challenge_solves, menu)
- `src/cli/ctf.rs` (add Scoreboard, Solves, ChallengeSolves commands)

**Estimated scope:** M (3 files)

---

## Checkpoint: After Tasks 5-6
- [ ] All tests pass
- [ ] Full CTF workflow works: list events, browse challenges, submit flags, view scoreboard
- [ ] Edge cases handled (no docker, no download, hidden scoreboard)

---

## Task 7: Cache integration for CTF

**Description:** Fix cache key collision: `sanitize_url()` in `cache.rs` currently strips the hostname, so `labs.hackthebox.com/api/users/profile` and `ctf.hackthebox.com/api/users/profile` collide. Change `sanitize_url` to include the hostname in the cache key. Add CTF-specific TTL rules to `ttl_for_path` - branch on `self.base_url` to distinguish labs vs CTF paths. Add cache invalidation after CTF mutations (flag submit, container start/stop). Update existing cache tests for the new key format.

**Acceptance criteria:**
- [ ] `sanitize_url` includes hostname in cache filename
- [ ] Labs `/api/users/profile` and CTF `/api/users/profile` produce different cache files
- [ ] CTF event list cached for 5 min, event details for 5 min
- [ ] CTF challenges/scoreboard/solves cached for 30 s
- [ ] CTF categories cached for 30 min, profile for 2 min
- [ ] `POST /api/flags/own` invalidates CTF challenge/score caches
- [ ] Container start/stop invalidates CTF challenge caches
- [ ] Existing cache tests updated and passing

**Verification:**
- [ ] `cargo test` passes (including updated cache tests)
- [ ] Manual: repeated `htb ctf events` is faster on second call; `htb ctf submit` invalidates challenge cache

**Dependencies:** Task 5

**Files likely touched:**
- `src/cache.rs` (fix sanitize_url to include hostname)
- `src/api/mod.rs` (extend ttl_for_path with CTF rules, extend invalidate_after_post)
- `tests/cache_integration.rs` (update expected cache key filenames)

**Estimated scope:** M (3 files)

---

## Task 8: Test fixtures and tests

**Description:** Create JSON fixtures from HAR data (scrubbed of sensitive data) for all CTF model types. Add unit tests for deserialization of every CTF model. Add wiremock integration tests for key API flows (events list, challenges, submit flag, container start).

**Acceptance criteria:**
- [ ] Fixtures exist for: ctf events list, event details, event data (with challenges), scoreboard, solves, flag result, user profile, challenge solves, menu
- [ ] Unit tests verify deserialization of all CTF model structs
- [ ] Integration tests cover: list events, list challenges, submit flag, container start/stop
- [ ] All tests pass in CI (no live API dependency)

**Verification:**
- [ ] `cargo test` passes with all new tests
- [ ] `cargo clippy` clean

**Dependencies:** Task 6

**Files likely touched:**
- `tests/fixtures/ctf/` (new directory, ~9 fixture files)
- `src/models/ctf.rs` (add #[cfg(test)] mod tests)
- `tests/ctf_integration.rs` (new)

**Estimated scope:** M (multiple new files)

---

## Final Checkpoint
- [ ] `cargo test` - all tests pass (existing + new)
- [ ] `cargo clippy` - clean
- [ ] `cargo fmt --check` - clean
- [ ] `cargo build --release` - succeeds
- [ ] Full manual walkthrough: auth, events, challenges, submit, scoreboard
- [ ] Labs commands still work unchanged
- [ ] Changeset written for the feature
