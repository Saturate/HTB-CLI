use serde_json::json;

use crate::error::HtbError;
use crate::models::sherlock::{Sherlock, SherlockCategoriesResponse, SherlockCategory};
use crate::models::{ActionResponse, Paginated};

use super::HtbClient;

pub struct SherlockApi<'a>(pub(crate) &'a HtbClient);

impl SherlockApi<'_> {
    pub async fn list(&self, page: u32, per_page: u32) -> Result<Paginated<Sherlock>, HtbError> {
        self.0
            .get(&format!(
                "/api/v4/sherlocks?per_page={per_page}&page={page}"
            ))
            .await
    }

    pub async fn categories(&self) -> Result<Vec<SherlockCategory>, HtbError> {
        let resp: SherlockCategoriesResponse =
            self.0.get("/api/v4/sherlocks/categories/list").await?;
        Ok(resp.info)
    }

    pub async fn info(&self, slug: &str) -> Result<Sherlock, HtbError> {
        let encoded = super::encode_path(slug);
        self.0.get(&format!("/api/v4/sherlocks/{encoded}")).await
    }

    pub async fn download_link(&self, sherlock_id: u64) -> Result<String, HtbError> {
        let resp: crate::models::challenge::ChallengeDownloadResponse = self
            .0
            .get(&format!("/api/v4/sherlocks/{sherlock_id}/download_link"))
            .await?;
        Ok(resp.url)
    }

    pub async fn submit_flag(
        &self,
        sherlock_id: u64,
        task_id: u64,
        flag: &str,
    ) -> Result<ActionResponse, HtbError> {
        self.0
            .post(
                &format!("/api/v4/sherlocks/{sherlock_id}/tasks/{task_id}/flag"),
                &json!({"flag": flag}),
            )
            .await
    }
}
