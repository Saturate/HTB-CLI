# Plan: HTTP Response Caching

## Task 1: Cache module (`src/cache.rs`)

**Description:** Create the standalone `Cache` struct with filesystem-based TTL caching. Sanitized URL path filenames, atomic writes via tempfile+rename, lazy disk sweep, corrupt file recovery. All methods infallible from the caller's perspective (errors log at debug, degrade to no-cache).

**Acceptance criteria:**
- [ ] `Cache::new(dir, enabled)` creates the cache dir if missing, degrades silently on failure
- [ ] `Cache::get(url, max_age)` returns `Some(body)` on hit, `None` on miss/expired/corrupt
- [ ] `Cache::set(url, body)` writes atomically (tempfile + rename) with `0o600` permissions
- [ ] `Cache::invalidate_pattern(glob)` removes matching cache files by filename prefix
- [ ] `Cache::clear()` removes all files in the cache directory
- [ ] Lazy sweep runs every 10th write, deleting entries older than 1 hour
- [ ] `cached_at` in the future is treated as expired

**Verification:**
- [ ] Unit tests: hit, miss, expiry (forged timestamps), corrupt file recovery, invalidation, clear
- [ ] `cargo test cache`
- [ ] `cargo clippy -- -D warnings`

**Dependencies:** None

**Files likely touched:**
- `src/cache.rs` (new)
- `src/main.rs` (add `mod cache`)

**Estimated scope:** Medium (1 new file, ~100 lines + tests)

---

## Task 2: Config and CLI flags

**Description:** Add `CacheConfig` to the config system and `--no-cache` global flag plus `htb cache clear` subcommand to the CLI.

**Acceptance criteria:**
- [ ] `config.toml` supports `[cache] enabled = true/false` (defaults to `true`)
- [ ] `--no-cache` global CLI flag is parsed and available
- [ ] `htb cache clear` subcommand exists and calls `Cache::clear()`
- [ ] Existing config tests still pass

**Verification:**
- [ ] `cargo test config`
- [ ] `cargo test` (all pass)
- [ ] Manual: `htb cache clear` prints confirmation

**Dependencies:** Task 1

**Files likely touched:**
- `src/config.rs` (add `CacheConfig`)
- `src/cli/mod.rs` (add `--no-cache` flag, `Cache` subcommand variant)
- `src/cli/cache.rs` (new, handler)
- `src/main.rs` (wire up cache clear, pass no_cache flag)

**Estimated scope:** Small (3-4 files, small changes each)

---

## Task 3: Integrate cache into HtbClient

**Description:** Add `Option<Cache>` to `HtbClient`, refactor `get` to delegate through a private `get_raw` that checks cache first, and add `ttl_for_path` to map URL patterns to TTL tiers. Wire cache construction in `main.rs`. No API module call sites change.

**Acceptance criteria:**
- [ ] `HtbClient` has an `Option<Cache>` field
- [ ] `get_raw(path, max_age) -> Result<String>` checks cache, falls back to HTTP, stores result
- [ ] `get<T: DeserializeOwned>(path)` delegates to `get_raw` then deserializes
- [ ] `ttl_for_path(path) -> Option<Duration>` returns correct TTL tier or `None` for uncached endpoints
- [ ] `get` on uncached endpoints (active VM, VPN, search) bypasses cache entirely
- [ ] `authenticated_client()` in `main.rs` constructs `HtbClient` with cache when enabled
- [ ] `HtbClient::with_base_url` (test constructor) continues to work without cache
- [ ] All 34 existing tests pass unchanged

**Verification:**
- [ ] `cargo test` (all pass)
- [ ] `cargo clippy -- -D warnings`
- [ ] Manual: run `htb machines info Bedside` twice, second is instant; `ls ~/.htb-cli/cache/` shows the file

**Dependencies:** Task 1, Task 2

**Files likely touched:**
- `src/api/mod.rs` (add cache field, get_raw, ttl_for_path)
- `src/main.rs` (construct cache, pass to client)

**Estimated scope:** Medium (2 files, core logic)

---

## Checkpoint: After Tasks 1-3
- [ ] All tests pass
- [ ] `htb machines info X` caches on first call, serves from cache on second
- [ ] `htb cache clear` works
- [ ] `--no-cache` bypasses cache
- [ ] `ls ~/.htb-cli/cache/` shows readable filenames

---

## Task 4: Mutation-triggered invalidation

**Description:** After successful POST requests, invalidate related cache entries. Machine mutations clear machine profile and list caches. Challenge mutations clear challenge caches.

**Acceptance criteria:**
- [ ] `vm/spawn`, `vm/terminate`, `vm/reset` invalidate `api_v*_machine*` cache files
- [ ] `container/start`, `container/stop` invalidate `api_v*_challenge*` cache files
- [ ] `machine/own` invalidates machine caches; `challenge/own` invalidates challenge caches
- [ ] `machine/todo/update` invalidates machine todo cache

**Verification:**
- [ ] Manual: `htb machines info Bedside`, `htb machines start Bedside`, `ls ~/.htb-cli/cache/` shows no machine cache files
- [ ] `cargo test`

**Dependencies:** Task 3

**Files likely touched:**
- `src/api/mod.rs` (add invalidation call after post)
- `src/api/machines.rs` (possibly, if invalidation needs path context)

**Estimated scope:** Small (1-2 files)

---

## Task 5: Clear cache on auth changes

**Description:** Clear the entire cache directory when the user logs in or logs out, since cached data is tied to the authenticated user.

**Acceptance criteria:**
- [ ] `htb auth login` clears cache after successful authentication
- [ ] `htb auth logout` clears cache after removing token

**Verification:**
- [ ] Manual: populate cache, run `htb auth logout`, verify cache is empty
- [ ] `cargo test`

**Dependencies:** Task 1

**Files likely touched:**
- `src/cli/auth.rs` (add cache clear calls)

**Estimated scope:** Small (1 file, 2-3 lines each)

---

## Checkpoint: After Tasks 4-5
- [ ] All tests pass
- [ ] Full invalidation flow works: cache, mutate, cache is cleared
- [ ] Auth changes clear cache
- [ ] Ready for review and PR
