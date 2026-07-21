use std::sync::Arc;

use htb_cli::cache::Cache;

use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn temp_cache(name: &str) -> Cache {
    let dir = std::env::temp_dir().join(format!("htb-cache-integ-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    Cache::new(dir, true)
}

#[tokio::test]
async fn cached_get_hits_server_once() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v4/machine/profile/TestBox"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "info": {"id": 1, "name": "TestBox", "os": "Linux",
                     "points": 30, "userOwnsCount": 0, "rootOwnsCount": 0}
        })))
        .expect(1)
        .mount(&server)
        .await;

    let cache = Arc::new(temp_cache("hit-once"));
    let client =
        htb_cli::api::HtbClient::with_base_url_and_cache("test-token".into(), server.uri(), cache);

    let result: serde_json::Value = client.get("/api/v4/machine/profile/TestBox").await.unwrap();
    assert_eq!(result["info"]["name"], "TestBox");

    let cached: serde_json::Value = client.get("/api/v4/machine/profile/TestBox").await.unwrap();
    assert_eq!(cached["info"]["name"], "TestBox");
}

#[tokio::test]
async fn invalidation_after_post_clears_machine_cache() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v4/machine/profile/Box"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "info": {"id": 927, "name": "Box", "os": "Linux",
                     "points": 30, "userOwnsCount": 0, "rootOwnsCount": 0}
        })))
        .expect(2)
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/api/v4/vm/spawn"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({"message": "Spawned", "success": true})),
        )
        .mount(&server)
        .await;

    let cache = Arc::new(temp_cache("invalidation"));
    let client =
        htb_cli::api::HtbClient::with_base_url_and_cache("test-token".into(), server.uri(), cache);

    let _: serde_json::Value = client.get("/api/v4/machine/profile/Box").await.unwrap();

    let _: serde_json::Value = client
        .post("/api/v4/vm/spawn", &serde_json::json!({"machine_id": 927}))
        .await
        .unwrap();

    let _: serde_json::Value = client.get("/api/v4/machine/profile/Box").await.unwrap();
}

#[tokio::test]
async fn uncached_endpoints_always_hit_server() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v5/virtual_machine/active"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({"info": null})))
        .expect(2)
        .mount(&server)
        .await;

    let cache = Arc::new(temp_cache("uncached"));
    let client =
        htb_cli::api::HtbClient::with_base_url_and_cache("test-token".into(), server.uri(), cache);

    let _: serde_json::Value = client.get("/api/v5/virtual_machine/active").await.unwrap();
    let _: serde_json::Value = client.get("/api/v5/virtual_machine/active").await.unwrap();
}

#[tokio::test]
async fn disabled_cache_always_hits_server() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v4/machine/profile/NoCacheBox"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "info": {"id": 1, "name": "NoCacheBox", "os": "Linux",
                     "points": 30, "userOwnsCount": 0, "rootOwnsCount": 0}
        })))
        .expect(2)
        .mount(&server)
        .await;

    let dir = std::env::temp_dir().join(format!("htb-cache-integ-disabled-{}", std::process::id()));
    let cache = Arc::new(Cache::new(dir, false));
    let client =
        htb_cli::api::HtbClient::with_base_url_and_cache("test-token".into(), server.uri(), cache);

    let _: serde_json::Value = client
        .get("/api/v4/machine/profile/NoCacheBox")
        .await
        .unwrap();
    let _: serde_json::Value = client
        .get("/api/v4/machine/profile/NoCacheBox")
        .await
        .unwrap();
}
