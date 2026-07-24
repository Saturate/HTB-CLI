use std::sync::Arc;

use htb_cli::cache::Cache;
use htb_cli::error::HtbError;
use htb_cli::models::ctf::{CtfEvent, CtfEventData, CtfFlagResult, CtfScoreboard, CtfSolve};

use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn temp_cache(name: &str) -> Cache {
    let dir = std::env::temp_dir().join(format!("htb-ctf-integ-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    Cache::new(dir, false)
}

#[tokio::test]
async fn list_ctf_events() {
    let server = MockServer::start().await;
    let body: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/ctf/events.json")).unwrap();

    Mock::given(method("GET"))
        .and(path("/api/ctfs"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&body))
        .expect(1)
        .mount(&server)
        .await;

    let cache = Arc::new(temp_cache("events"));
    let client =
        htb_cli::api::HtbClient::with_base_url_and_cache("test-token".into(), server.uri(), cache);

    let events: Vec<CtfEvent> = client.get("/api/ctfs").await.unwrap();
    assert_eq!(events.len(), 2);
    assert_eq!(events[0].name, "CTF Try Out");
}

#[tokio::test]
async fn get_event_data_with_challenges() {
    let server = MockServer::start().await;
    let body: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/ctf/event-data.json")).unwrap();

    Mock::given(method("GET"))
        .and(path("/api/ctfs/1434"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&body))
        .expect(1)
        .mount(&server)
        .await;

    let cache = Arc::new(temp_cache("event-data"));
    let client =
        htb_cli::api::HtbClient::with_base_url_and_cache("test-token".into(), server.uri(), cache);

    let data: CtfEventData = client.get("/api/ctfs/1434").await.unwrap();
    assert_eq!(data.challenges.len(), 2);
    assert!(data.challenges[0].solved);
    assert_eq!(data.challenges[1].has_docker, Some(1));
}

#[tokio::test]
async fn submit_flag() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/api/flags/own"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({"message": "Correct flag!", "points": 725})),
        )
        .expect(1)
        .mount(&server)
        .await;

    let cache = Arc::new(temp_cache("submit"));
    let client =
        htb_cli::api::HtbClient::with_base_url_and_cache("test-token".into(), server.uri(), cache);

    let result: CtfFlagResult = client
        .post(
            "/api/flags/own",
            &serde_json::json!({"challenge_id": 31855, "flag": "HTB{test}"}),
        )
        .await
        .unwrap();

    assert_eq!(result.message, "Correct flag!");
    assert_eq!(result.points, Some(725));
}

#[tokio::test]
async fn get_scoreboard() {
    let server = MockServer::start().await;
    let body: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/ctf/scoreboard.json")).unwrap();

    Mock::given(method("GET"))
        .and(path("/api/ctfs/scores/1434"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&body))
        .expect(1)
        .mount(&server)
        .await;

    let cache = Arc::new(temp_cache("scoreboard"));
    let client =
        htb_cli::api::HtbClient::with_base_url_and_cache("test-token".into(), server.uri(), cache);

    let sb: CtfScoreboard = client.get("/api/ctfs/scores/1434").await.unwrap();
    assert_eq!(sb.scores.len(), 2);
    assert_eq!(sb.participating_team.unwrap().position, Some(42));
}

#[tokio::test]
async fn get_solves_feed() {
    let server = MockServer::start().await;
    let body: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/ctf/solves.json")).unwrap();

    Mock::given(method("GET"))
        .and(path("/api/ctfs/solves/1434"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&body))
        .expect(1)
        .mount(&server)
        .await;

    let cache = Arc::new(temp_cache("solves"));
    let client =
        htb_cli::api::HtbClient::with_base_url_and_cache("test-token".into(), server.uri(), cache);

    let solves: Vec<CtfSolve> = client.get("/api/ctfs/solves/1434").await.unwrap();
    assert_eq!(solves.len(), 2);
    assert_eq!(solves[0].challenge_name.as_deref(), Some("LootStash"));
}

#[tokio::test]
async fn html_403_gives_friendly_error() {
    let server = MockServer::start().await;
    let html = r#"<!DOCTYPE html>
<html><head><title>403 Forbidden</title></head>
<body><h1>403 Forbidden</h1></body></html>"#;

    Mock::given(method("POST"))
        .and(path("/api/challenges/containers/start"))
        .respond_with(ResponseTemplate::new(403).set_body_string(html))
        .expect(1)
        .mount(&server)
        .await;

    let cache = Arc::new(temp_cache("html-403"));
    let client =
        htb_cli::api::HtbClient::with_base_url_and_cache("test-token".into(), server.uri(), cache);

    let err = client.ctf().container_start(12345).await.unwrap_err();

    match err {
        HtbError::Api { status, message } => {
            assert_eq!(status, 403);
            assert_eq!(message, "403 Forbidden");
            assert!(!message.contains('<'));
        }
        other => panic!("expected HtbError::Api, got: {other:?}"),
    }
}
