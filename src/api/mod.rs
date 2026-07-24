pub mod challenges;
pub mod ctf;
pub mod machines;
pub mod search;
pub mod seasons;
pub mod sherlocks;
pub mod user;
pub mod vpn;

pub fn encode_path(s: &str) -> String {
    s.bytes()
        .flat_map(|b| {
            if b.is_ascii_alphanumeric() || b == b'-' || b == b'_' || b == b'.' || b == b'~' {
                vec![b as char]
            } else {
                format!("%{b:02X}").chars().collect()
            }
        })
        .collect()
}

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Duration;

use reqwest::header::HeaderMap;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::cache::Cache;
use crate::error::{ApiErrorBody, HtbError};

fn extract_error_message(body: &str, status: u16) -> String {
    if let Ok(e) = serde_json::from_str::<ApiErrorBody>(body) {
        return e.message;
    }

    let trimmed = body.trim_start();
    if trimmed.starts_with("<!") || trimmed.starts_with("<html") || trimmed.starts_with("<HTML") {
        let lower = trimmed.to_lowercase();
        if let Some(start) = lower.find("<title>") {
            let after = &trimmed[start + 7..];
            if let Some(end) = after.to_lowercase().find("</title>") {
                let title = after[..end].trim();
                if !title.is_empty() {
                    return title.to_string();
                }
            }
        }
        return format!("Server returned an error (HTTP {status})");
    }

    body.to_string()
}

const BASE_URL: &str = "https://labs.hackthebox.com";
const USER_AGENT: &str = concat!("htb-cli/", env!("CARGO_PKG_VERSION"));

#[derive(Clone)]
pub struct HtbClient {
    http: reqwest::Client,
    base_url: String,
    token: String,
    rate_limit: Arc<RateLimitState>,
    cache: Option<Arc<Cache>>,
}

struct RateLimitState {
    remaining: AtomicU32,
    limit: AtomicU32,
}

impl RateLimitState {
    fn new() -> Self {
        Self {
            remaining: AtomicU32::new(u32::MAX),
            limit: AtomicU32::new(u32::MAX),
        }
    }

    fn update(&self, headers: &HeaderMap) {
        if let Some(limit) = headers
            .get("x-ratelimit-limit")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse::<u32>().ok())
        {
            self.limit.store(limit, Ordering::Relaxed);
        }

        if let Some(remaining) = headers
            .get("x-ratelimit-remaining")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse::<u32>().ok())
        {
            self.remaining.store(remaining, Ordering::Relaxed);
        }
    }

    fn remaining(&self) -> u32 {
        self.remaining.load(Ordering::Relaxed)
    }

    fn limit(&self) -> u32 {
        self.limit.load(Ordering::Relaxed)
    }
}

impl HtbClient {
    pub fn new(token: String) -> Self {
        Self::build(token, BASE_URL.to_string(), None)
    }

    pub fn with_cache_arc(token: String, cache: Arc<Cache>) -> Self {
        Self::build(token, BASE_URL.to_string(), Some(cache))
    }

    pub fn with_base_url(token: String, base_url: String) -> Self {
        Self::build(token, base_url, None)
    }

    pub fn with_base_url_and_cache(token: String, base_url: String, cache: Arc<Cache>) -> Self {
        Self::build(token, base_url, Some(cache))
    }

    fn build(token: String, base_url: String, cache: Option<Arc<Cache>>) -> Self {
        let http = reqwest::Client::builder()
            .user_agent(USER_AGENT)
            .timeout(Duration::from_secs(30))
            .build()
            .expect("failed to build HTTP client");

        Self {
            http,
            base_url,
            token,
            rate_limit: Arc::new(RateLimitState::new()),
            cache,
        }
    }

    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, HtbError> {
        let url = format!("{}{}", self.base_url, path);
        let ttl = self.ttl_for_path(path);

        if let Some(max_age) = ttl {
            if let Some(cache) = &self.cache {
                if let Some(body) = cache.get(&url, max_age) {
                    match serde_json::from_str(&body) {
                        Ok(parsed) => return Ok(parsed),
                        Err(e) => {
                            tracing::debug!(
                                "cached response failed to deserialize, refetching: {e}"
                            );
                        }
                    }
                }
            }
        }

        self.wait_for_rate_limit().await;
        tracing::debug!(url = %url, "GET");
        let resp = self.http.get(&url).bearer_auth(&self.token).send().await?;
        let body = self.handle_response_raw(resp).await?;

        if let Some(max_age) = ttl {
            if max_age > Duration::ZERO {
                if let Some(cache) = &self.cache {
                    cache.set(&url, &body);
                }
            }
        }

        Ok(serde_json::from_str(&body)?)
    }

    pub async fn post<B: Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, HtbError> {
        self.wait_for_rate_limit().await;

        let url = format!("{}{}", self.base_url, path);
        tracing::debug!(url = %url, "POST");

        let resp = self
            .http
            .post(&url)
            .bearer_auth(&self.token)
            .json(body)
            .send()
            .await?;

        let result = self.handle_response(resp).await?;
        self.invalidate_after_post(path);
        Ok(result)
    }

    pub async fn post_no_content<B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<(), HtbError> {
        self.wait_for_rate_limit().await;

        let url = format!("{}{}", self.base_url, path);
        tracing::debug!(url = %url, "POST (no content)");

        let resp = self
            .http
            .post(&url)
            .bearer_auth(&self.token)
            .json(body)
            .send()
            .await?;

        self.rate_limit.update(resp.headers());
        self.log_rate_limit();

        let status = resp.status();
        if status == 401 {
            return Err(HtbError::NotAuthenticated);
        }
        if status == 429 {
            return Err(HtbError::RateLimited);
        }
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(HtbError::Api {
                status: status.as_u16(),
                message: extract_error_message(&body, status.as_u16()),
            });
        }

        self.invalidate_after_post(path);
        Ok(())
    }

    fn invalidate_after_post(&self, path: &str) {
        let Some(cache) = &self.cache else { return };
        if path.contains("/vm/spawn")
            || path.contains("/vm/terminate")
            || path.contains("/vm/reset")
            || path.contains("/machine/own")
            || path.contains("/machine/todo")
        {
            cache.invalidate_pattern("api_v4_machine");
            cache.invalidate_pattern("api_v5_machine");
        }
        if path.contains("/container/start")
            || path.contains("/container/stop")
            || path.contains("/challenge/own")
        {
            cache.invalidate_pattern("api_v4_challenge");
        }
        if path.contains("/sherlocks/") && path.contains("/flag") {
            cache.invalidate_pattern("api_v4_sherlock");
        }
        // CTF mutations
        if path.contains("/flags/own")
            || path.contains("/challenges/containers/start")
            || path.contains("/challenges/containers/stop")
        {
            cache.invalidate_pattern("ctf.hackthebox.com_api_ctfs_");
            cache.invalidate_pattern("ctf.hackthebox.com_api_challenges_");
        }
    }

    pub async fn get_bytes(&self, url_or_path: &str) -> Result<Vec<u8>, HtbError> {
        self.wait_for_rate_limit().await;

        let is_absolute = url_or_path.starts_with("http://") || url_or_path.starts_with("https://");
        let url = if is_absolute {
            url_or_path.to_string()
        } else {
            format!("{}{}", self.base_url, url_or_path)
        };
        tracing::debug!(url = %url, "GET (bytes)");

        let same_origin = url.starts_with(&format!("{}/", self.base_url));
        let req = self.http.get(&url);
        let req = if same_origin {
            req.bearer_auth(&self.token)
        } else {
            req
        };
        let resp = req.send().await?;

        self.rate_limit.update(resp.headers());
        self.log_rate_limit();

        let status = resp.status();
        if status == 401 {
            return Err(HtbError::NotAuthenticated);
        }
        if status == 429 {
            return Err(HtbError::RateLimited);
        }
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(HtbError::Api {
                status: status.as_u16(),
                message: extract_error_message(&body, status.as_u16()),
            });
        }

        Ok(resp.bytes().await?.to_vec())
    }

    async fn handle_response<T: DeserializeOwned>(
        &self,
        resp: reqwest::Response,
    ) -> Result<T, HtbError> {
        let body = self.handle_response_raw(resp).await?;
        Ok(serde_json::from_str(&body)?)
    }

    async fn handle_response_raw(&self, resp: reqwest::Response) -> Result<String, HtbError> {
        self.rate_limit.update(resp.headers());
        self.log_rate_limit();

        let status = resp.status();

        if status == 401 {
            return Err(HtbError::NotAuthenticated);
        }

        if status == 429 {
            return Err(HtbError::RateLimited);
        }

        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(HtbError::Api {
                status: status.as_u16(),
                message: extract_error_message(&body, status.as_u16()),
            });
        }

        Ok(resp.text().await?)
    }

    async fn wait_for_rate_limit(&self) {
        let remaining = self.rate_limit.remaining();
        if remaining == 0 && self.rate_limit.limit() != u32::MAX {
            // Single-threaded runtime; no concurrent task will update the atomic
            // while we sleep. Wait once and let the next response refresh the state.
            tracing::warn!("Rate limit exhausted, waiting 5s before next request");
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }

    fn log_rate_limit(&self) {
        let remaining = self.rate_limit.remaining();
        let limit = self.rate_limit.limit();
        if limit != u32::MAX {
            tracing::debug!(remaining, limit, "rate limit");
        }
    }

    pub fn ctf(&self) -> ctf::CtfApi<'_> {
        ctf::CtfApi(self)
    }

    pub fn user(&self) -> user::UserApi<'_> {
        user::UserApi(self)
    }

    pub fn machines(&self) -> machines::MachineApi<'_> {
        machines::MachineApi(self)
    }

    pub fn challenges(&self) -> challenges::ChallengeApi<'_> {
        challenges::ChallengeApi(self)
    }

    pub fn sherlocks(&self) -> sherlocks::SherlockApi<'_> {
        sherlocks::SherlockApi(self)
    }

    pub fn seasons(&self) -> seasons::SeasonApi<'_> {
        seasons::SeasonApi(self)
    }

    pub fn vpn(&self) -> vpn::VpnApi<'_> {
        vpn::VpnApi(self)
    }

    pub fn search(&self) -> search::SearchApi<'_> {
        search::SearchApi(self)
    }

    fn ttl_for_path(&self, path: &str) -> Option<Duration> {
        if path.contains("/download") {
            return None;
        }

        let is_ctf = self.base_url.contains("ctf.hackthebox.com");
        if is_ctf {
            return self.ttl_for_ctf_path(path);
        }

        // Labs: reference data (30 min)
        if path.contains("/categories/list")
            || path.contains("/season/list")
            || path.contains("/tags/list")
        {
            return Some(Duration::from_secs(1800));
        }
        // Labs: lists (5 min)
        if path.starts_with("/api/v5/machines")
            || path.starts_with("/api/v4/challenges?")
            || path.starts_with("/api/v4/sherlocks?")
        {
            return Some(Duration::from_secs(300));
        }
        // Labs: challenge/machine/sherlock details (60 min, mostly static)
        if path.contains("/challenge/info/")
            || path.contains("/machine/profile/")
            || path.contains("/sherlocks/")
        {
            return Some(Duration::from_secs(3600));
        }
        // Labs: user profiles (2 min, points/rank change after submissions)
        if path.contains("/user/info") || path.contains("/user/profile/") {
            return Some(Duration::from_secs(120));
        }
        None
    }

    fn ttl_for_ctf_path(&self, path: &str) -> Option<Duration> {
        // Reference data (30 min)
        if path.starts_with("/api/public/challenge-categories") {
            return Some(Duration::from_secs(1800));
        }
        // Event list and details (5 min)
        if path == "/api/ctfs" || path.starts_with("/api/ctfs/details/") {
            return Some(Duration::from_secs(300));
        }
        // Profiles (2 min)
        if path.starts_with("/api/users/profile") {
            return Some(Duration::from_secs(120));
        }
        // Live data: challenges, scoreboard, solves (30 s)
        if path.starts_with("/api/ctfs/scores/")
            || path.starts_with("/api/ctfs/solves/")
            || path.starts_with("/api/ctfs/score-charts/")
            || path.starts_with("/api/challenges/")
        {
            return Some(Duration::from_secs(30));
        }
        // Event data with challenges (30 s)
        if path.starts_with("/api/ctfs/") {
            return Some(Duration::from_secs(30));
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rate_limit_state_parses_headers() {
        let state = RateLimitState::new();
        let mut headers = HeaderMap::new();
        headers.insert("x-ratelimit-limit", "25".parse().unwrap());
        headers.insert("x-ratelimit-remaining", "14".parse().unwrap());

        state.update(&headers);
        assert_eq!(state.limit(), 25);
        assert_eq!(state.remaining(), 14);
    }

    #[test]
    fn rate_limit_state_ignores_missing_headers() {
        let state = RateLimitState::new();
        let headers = HeaderMap::new();

        state.update(&headers);
        assert_eq!(state.limit(), u32::MAX);
        assert_eq!(state.remaining(), u32::MAX);
    }

    #[test]
    fn rate_limit_state_ignores_garbage() {
        let state = RateLimitState::new();
        let mut headers = HeaderMap::new();
        headers.insert("x-ratelimit-limit", "not-a-number".parse().unwrap());
        headers.insert("x-ratelimit-remaining", "".parse().unwrap());

        state.update(&headers);
        assert_eq!(state.limit(), u32::MAX);
        assert_eq!(state.remaining(), u32::MAX);
    }

    #[test]
    fn extract_error_message_json() {
        let body = r#"{"message":"Incorrect flag"}"#;
        assert_eq!(extract_error_message(body, 403), "Incorrect flag");
    }

    #[test]
    fn extract_error_message_html_with_title() {
        let body = r#"<!DOCTYPE html>
<html><head><title>403 Forbidden</title></head>
<body><h1>403 Forbidden</h1></body></html>"#;
        assert_eq!(extract_error_message(body, 403), "403 Forbidden");
    }

    #[test]
    fn extract_error_message_html_no_title() {
        let body = "<!DOCTYPE html><html><body>error</body></html>";
        assert_eq!(
            extract_error_message(body, 403),
            "Server returned an error (HTTP 403)"
        );
    }

    #[test]
    fn extract_error_message_plain_text() {
        let body = "something went wrong";
        assert_eq!(extract_error_message(body, 500), "something went wrong");
    }
}
