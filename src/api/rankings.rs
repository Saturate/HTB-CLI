use crate::error::HtbError;
use crate::models::rankings::{RankingUserEntry, RankingsUsersResponse};

use super::HtbClient;

pub struct RankingsApi<'a>(pub(crate) &'a HtbClient);

impl RankingsApi<'_> {
    pub async fn users(&self) -> Result<Vec<RankingUserEntry>, HtbError> {
        let resp: RankingsUsersResponse = self.0.get("/api/v4/rankings/users").await?;
        Ok(resp.data)
    }
}
