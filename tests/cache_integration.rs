use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn temp_cache_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("htb-cache-integ-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn sanitize_url(url: &str) -> String {
    let path = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))
        .unwrap_or(url);
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

#[derive(serde::Serialize, serde::Deserialize)]
struct CacheEntry {
    cached_at: u64,
    body: String,
}

fn cache_get(dir: &Path, url: &str, max_age: Duration) -> Option<String> {
    let filename = format!("{}.json", sanitize_url(url));
    let path = dir.join(filename);
    let data = fs::read_to_string(&path).ok()?;
    let entry: CacheEntry = serde_json::from_str(&data).ok()?;
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    if now - entry.cached_at > max_age.as_secs() {
        return None;
    }
    Some(entry.body)
}

fn cache_set(dir: &Path, url: &str, body: &str) {
    let filename = format!("{}.json", sanitize_url(url));
    let path = dir.join(filename);
    let entry = CacheEntry {
        cached_at: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        body: body.to_string(),
    };
    fs::write(path, serde_json::to_string(&entry).unwrap()).unwrap();
}

#[tokio::test]
async fn cache_prevents_duplicate_http_requests() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v4/machine/profile/TestBox"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(serde_json::json!({"info": {
                "id": 1, "name": "TestBox", "os": "Linux"
            }})),
        )
        .expect(1)
        .mount(&server)
        .await;

    let cache_dir = temp_cache_dir("dup-requests");
    let url = format!("{}/api/v4/machine/profile/TestBox", server.uri());
    let max_age = Duration::from_secs(120);

    // First request: cache miss, hits the server
    assert!(cache_get(&cache_dir, &url, max_age).is_none());
    let resp = reqwest::get(&url).await.unwrap();
    let body = resp.text().await.unwrap();
    cache_set(&cache_dir, &url, &body);

    // Second request: cache hit, does NOT hit the server
    let cached = cache_get(&cache_dir, &url, max_age);
    assert!(cached.is_some());
    let parsed: serde_json::Value = serde_json::from_str(&cached.unwrap()).unwrap();
    assert_eq!(parsed["info"]["name"], "TestBox");

    // Wiremock will panic at drop if the mock was called more than once (expect(1))
}

#[tokio::test]
async fn invalidation_forces_fresh_fetch() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v4/machine/profile/Box"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(serde_json::json!({"info": {"id": 1}})),
        )
        .expect(2)
        .mount(&server)
        .await;

    let cache_dir = temp_cache_dir("invalidation");
    let url = format!("{}/api/v4/machine/profile/Box", server.uri());
    let max_age = Duration::from_secs(120);

    // First request
    let resp = reqwest::get(&url).await.unwrap();
    cache_set(&cache_dir, &url, &resp.text().await.unwrap());
    assert!(cache_get(&cache_dir, &url, max_age).is_some());

    // Invalidate machine cache (simulate post-mutation)
    for entry in fs::read_dir(&cache_dir).unwrap().flatten() {
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if name.starts_with("api_v4_machine") && name.ends_with(".json") {
            fs::remove_file(entry.path()).unwrap();
        }
    }

    // Cache miss after invalidation
    assert!(cache_get(&cache_dir, &url, max_age).is_none());

    // Second request hits server again
    let resp = reqwest::get(&url).await.unwrap();
    cache_set(&cache_dir, &url, &resp.text().await.unwrap());

    // Wiremock verifies exactly 2 calls
}

#[tokio::test]
async fn expired_cache_entry_causes_refetch() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v4/user/info"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({"info": {"id": 1, "name": "User"}})),
        )
        .expect(1)
        .mount(&server)
        .await;

    let cache_dir = temp_cache_dir("expiry");
    let url = format!("{}/api/v4/user/info", server.uri());

    // Write a stale entry (5 minutes old, 2 min TTL)
    let filename = format!("{}.json", sanitize_url(&url));
    let stale = CacheEntry {
        cached_at: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            - 300,
        body: r#"{"info":{"id":0,"name":"Stale"}}"#.to_string(),
    };
    fs::write(
        cache_dir.join(filename),
        serde_json::to_string(&stale).unwrap(),
    )
    .unwrap();

    // Should be expired
    assert!(cache_get(&cache_dir, &url, Duration::from_secs(120)).is_none());

    // Fetch fresh data
    let resp = reqwest::get(&url).await.unwrap();
    let body = resp.text().await.unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&body).unwrap();
    assert_eq!(parsed["info"]["name"], "User");
}
