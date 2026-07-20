use crate::error::HtbError;
use crate::models::season::{Season, SeasonListResponse, SeasonUserRank, SeasonUserRanksResponse};

use super::HtbClient;

pub struct SeasonApi<'a>(pub(crate) &'a HtbClient);

impl SeasonApi<'_> {
    pub async fn list(&self) -> Result<Vec<Season>, HtbError> {
        let resp: SeasonListResponse = self.0.get("/api/v4/season/list").await?;
        Ok(resp.data)
    }

    pub async fn user_ranks(&self, user_id: u64) -> Result<Vec<SeasonUserRank>, HtbError> {
        let resp: SeasonUserRanksResponse = self
            .0
            .get(&format!("/api/v4/season/user/{user_id}/ranks"))
            .await?;
        Ok(resp.data)
    }
}
