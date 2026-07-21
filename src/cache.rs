use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

const SWEEP_INTERVAL: u32 = 10;
const SWEEP_MAX_AGE: Duration = Duration::from_secs(3600);

#[derive(Serialize, Deserialize)]
struct CacheEntry {
    cached_at: u64,
    body: String,
}

pub struct Cache {
    dir: PathBuf,
    enabled: bool,
    write_count: AtomicU32,
}

impl Cache {
    pub fn new(dir: PathBuf, enabled: bool) -> Self {
        if enabled {
            if let Err(e) = fs::create_dir_all(&dir) {
                tracing::debug!("cache dir creation failed, caching disabled: {e}");
                return Self {
                    dir,
                    enabled: false,
                    write_count: AtomicU32::new(0),
                };
            }
        }
        Self {
            dir,
            enabled,
            write_count: AtomicU32::new(0),
        }
    }

    pub fn get(&self, url: &str, max_age: Duration) -> Option<String> {
        if !self.enabled {
            return None;
        }
        let path = self.path_for(url);
        let data = fs::read_to_string(&path).ok()?;
        let entry: CacheEntry = match serde_json::from_str(&data) {
            Ok(e) => e,
            Err(_) => {
                tracing::debug!("corrupt cache file, removing: {}", path.display());
                let _ = fs::remove_file(&path);
                return None;
            }
        };
        let now = now_secs();
        if entry.cached_at > now {
            tracing::debug!("cache entry has future timestamp, treating as expired");
            let _ = fs::remove_file(&path);
            return None;
        }
        if now - entry.cached_at > max_age.as_secs() {
            return None;
        }
        tracing::debug!("cache hit: {url}");
        Some(entry.body)
    }

    pub fn set(&self, url: &str, body: &str) {
        if !self.enabled {
            return;
        }
        let path = self.path_for(url);
        let entry = CacheEntry {
            cached_at: now_secs(),
            body: body.to_string(),
        };
        let Ok(data) = serde_json::to_string(&entry) else {
            return;
        };

        let tmp = path.with_extension(format!("{}.tmp", std::process::id()));
        if write_atomic(&tmp, &path, data.as_bytes()).is_err() {
            tracing::debug!("cache write failed: {}", path.display());
        }

        let count = self.write_count.fetch_add(1, Ordering::Relaxed);
        if count > 0 && count.is_multiple_of(SWEEP_INTERVAL) {
            self.sweep();
        }
    }

    pub fn invalidate_pattern(&self, prefix: &str) {
        if !self.enabled {
            return;
        }
        let entries = match fs::read_dir(&self.dir) {
            Ok(e) => e,
            Err(_) => return,
        };
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if name.starts_with(prefix) && name.ends_with(".json") {
                let _ = fs::remove_file(entry.path());
            }
        }
    }

    pub fn clear(&self) {
        let entries = match fs::read_dir(&self.dir) {
            Ok(e) => e,
            Err(_) => return,
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let ext = path.extension().and_then(|e| e.to_str());
            if ext == Some("json") || ext == Some("tmp") {
                let _ = fs::remove_file(path);
            }
        }
    }

    fn path_for(&self, url: &str) -> PathBuf {
        self.dir.join(format!("{}.json", sanitize_url(url)))
    }

    fn sweep(&self) {
        let entries = match fs::read_dir(&self.dir) {
            Ok(e) => e,
            Err(_) => return,
        };
        let now = now_secs();
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            let data = match fs::read_to_string(&path) {
                Ok(d) => d,
                Err(_) => continue,
            };
            if let Ok(entry) = serde_json::from_str::<CacheEntry>(&data) {
                if now.saturating_sub(entry.cached_at) > SWEEP_MAX_AGE.as_secs() {
                    let _ = fs::remove_file(&path);
                }
            } else {
                let _ = fs::remove_file(&path);
            }
        }
    }
}

fn sanitize_url(url: &str) -> String {
    let path = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))
        .unwrap_or(url);
    // Strip the host portion
    let path = match path.find('/') {
        Some(i) => &path[i + 1..],
        None => path,
    };
    path.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == '.' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn write_atomic(tmp: &std::path::Path, dest: &std::path::Path, data: &[u8]) -> std::io::Result<()> {
    #[cfg(unix)]
    {
        use std::io::Write;
        use std::os::unix::fs::OpenOptionsExt;
        let mut f = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .mode(0o600)
            .open(tmp)?;
        f.write_all(data)?;
    }
    #[cfg(not(unix))]
    {
        fs::write(tmp, data)?;
    }
    fs::rename(tmp, dest)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_cache(name: &str) -> (Cache, PathBuf) {
        let dir =
            std::env::temp_dir().join(format!("htb-cache-test-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        let cache = Cache::new(dir.clone(), true);
        (cache, dir)
    }

    #[test]
    fn cache_hit_and_miss() {
        let (cache, _dir) = temp_cache("hit-miss");
        let url = "https://labs.hackthebox.com/api/v4/machine/profile/Test";

        assert!(cache.get(url, Duration::from_secs(120)).is_none());

        cache.set(url, r#"{"info":{"id":1}}"#);

        let hit = cache.get(url, Duration::from_secs(120));
        assert_eq!(hit.as_deref(), Some(r#"{"info":{"id":1}}"#));
    }

    #[test]
    fn cache_expiry() {
        let (cache, _dir) = temp_cache("expiry");
        let url = "https://labs.hackthebox.com/api/v4/machine/profile/Old";

        // Write an entry with a timestamp in the past
        let path = cache.path_for(url);
        let old_entry = CacheEntry {
            cached_at: now_secs() - 300,
            body: r#"{"stale":true}"#.to_string(),
        };
        fs::write(&path, serde_json::to_string(&old_entry).unwrap()).unwrap();

        assert!(cache.get(url, Duration::from_secs(120)).is_none());
    }

    #[test]
    fn cache_future_timestamp_treated_as_expired() {
        let (cache, _dir) = temp_cache("future-ts");
        let url = "https://labs.hackthebox.com/api/v4/test";

        let path = cache.path_for(url);
        let future_entry = CacheEntry {
            cached_at: now_secs() + 9999,
            body: r#"{"future":true}"#.to_string(),
        };
        fs::write(&path, serde_json::to_string(&future_entry).unwrap()).unwrap();

        assert!(cache.get(url, Duration::from_secs(120)).is_none());
        assert!(!path.exists());
    }

    #[test]
    fn corrupt_file_treated_as_miss() {
        let (cache, _dir) = temp_cache("corrupt");
        let url = "https://labs.hackthebox.com/api/v4/corrupt";

        let path = cache.path_for(url);
        fs::write(&path, "not valid json{{{").unwrap();

        assert!(cache.get(url, Duration::from_secs(120)).is_none());
        assert!(!path.exists());
    }

    #[test]
    fn invalidate_pattern() {
        let (cache, _dir) = temp_cache("invalidate");
        let url1 = "https://labs.hackthebox.com/api/v4/machine/profile/A";
        let url2 = "https://labs.hackthebox.com/api/v4/machine/profile/B";
        let url3 = "https://labs.hackthebox.com/api/v4/challenge/info/C";

        cache.set(url1, "a");
        cache.set(url2, "b");
        cache.set(url3, "c");

        cache.invalidate_pattern("api_v4_machine");

        assert!(cache.get(url1, Duration::from_secs(300)).is_none());
        assert!(cache.get(url2, Duration::from_secs(300)).is_none());
        assert!(cache.get(url3, Duration::from_secs(300)).is_some());
    }

    #[test]
    fn clear_removes_all() {
        let (cache, _dir) = temp_cache("clear");
        cache.set("https://example.com/api/v4/a", "1");
        cache.set("https://example.com/api/v4/b", "2");

        cache.clear();

        assert!(cache
            .get("https://example.com/api/v4/a", Duration::from_secs(300))
            .is_none());
        assert!(cache
            .get("https://example.com/api/v4/b", Duration::from_secs(300))
            .is_none());
    }

    #[test]
    fn disabled_cache_is_noop() {
        let dir = std::env::temp_dir().join(format!("htb-cache-disabled-{}", std::process::id()));
        let cache = Cache::new(dir, false);
        let url = "https://example.com/api/v4/test";

        cache.set(url, "data");
        assert!(cache.get(url, Duration::from_secs(300)).is_none());
    }

    #[test]
    fn sanitize_url_produces_safe_filenames() {
        assert_eq!(
            sanitize_url("https://labs.hackthebox.com/api/v4/machine/profile/Bedside"),
            "api_v4_machine_profile_Bedside"
        );
        assert_eq!(
            sanitize_url("https://labs.hackthebox.com/api/v5/machines?per_page=100&page=1"),
            "api_v5_machines_per_page_100_page_1"
        );
    }
}
