use crate::error::HtbError;
use crate::models::season::{
    Season, SeasonLeaderboardEntry, SeasonLeaderboardResponse, SeasonListResponse, SeasonMachine,
    SeasonMachinesResponse, SeasonUserRank, SeasonUserRanksResponse,
};

use super::HtbClient;

pub struct SeasonApi<'a>(pub(crate) &'a HtbClient);

impl SeasonApi<'_> {
    pub async fn machines(&self, season_id: u32) -> Result<Vec<SeasonMachine>, HtbError> {
        let resp: SeasonMachinesResponse = self
            .0
            .get(&format!("/api/v4/season/machines/{season_id}"))
            .await?;
        Ok(resp.data)
    }

    pub async fn leaderboard(
        &self,
        season_id: u32,
    ) -> Result<Vec<SeasonLeaderboardEntry>, HtbError> {
        let resp: SeasonLeaderboardResponse = self
            .0
            .get(&format!("/api/v4/season/leaderboard?season_id={season_id}"))
            .await?;
        Ok(resp.data)
    }

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
