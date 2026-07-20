use serde_json::json;

use crate::error::HtbError;
use crate::models::challenge::{
    Challenge, ChallengeCategoriesResponse, ChallengeCategory, ChallengeDetail,
    ChallengeDetailResponse, ChallengeDownloadResponse, ChallengeOwnResponse,
};
use crate::models::{ActionResponse, Paginated};

use super::HtbClient;

pub struct ChallengeApi<'a>(pub(crate) &'a HtbClient);

impl ChallengeApi<'_> {
    pub async fn list(&self, page: u32, per_page: u32) -> Result<Paginated<Challenge>, HtbError> {
        self.0
            .get(&format!(
                "/api/v4/challenges?per_page={per_page}&page={page}"
            ))
            .await
    }

    pub async fn categories(&self) -> Result<Vec<ChallengeCategory>, HtbError> {
        let resp: ChallengeCategoriesResponse =
            self.0.get("/api/v4/challenge/categories/list").await?;
        Ok(resp.info)
    }

    pub async fn info(&self, slug: &str) -> Result<ChallengeDetail, HtbError> {
        let resp: ChallengeDetailResponse = self
            .0
            .get(&format!("/api/v4/challenge/info/{slug}"))
            .await?;
        Ok(resp.challenge)
    }

    pub async fn start(&self, challenge_id: u64) -> Result<ActionResponse, HtbError> {
        self.0
            .post(
                "/api/v4/container/start",
                &json!({"containerable_id": challenge_id}),
            )
            .await
    }

    pub async fn stop(&self, challenge_id: u64) -> Result<ActionResponse, HtbError> {
        self.0
            .post(
                "/api/v4/container/stop",
                &json!({"containerable_id": challenge_id}),
            )
            .await
    }

    pub async fn submit_flag(
        &self,
        challenge_id: u64,
        flag: &str,
    ) -> Result<ChallengeOwnResponse, HtbError> {
        self.0
            .post(
                "/api/v4/challenge/own",
                &json!({"challenge_id": challenge_id, "flag": flag}),
            )
            .await
    }

    pub async fn download_link(&self, challenge_id: u64) -> Result<String, HtbError> {
        let resp: ChallengeDownloadResponse = self
            .0
            .get(&format!("/api/v4/challenges/{challenge_id}/download_link"))
            .await?;
        Ok(resp.url)
    }

    pub async fn download_file(&self, url: &str) -> Result<Vec<u8>, HtbError> {
        self.0.get_bytes(url).await
    }
}
