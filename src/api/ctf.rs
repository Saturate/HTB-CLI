use serde_json::json;

use crate::error::HtbError;
use crate::models::ctf::{
    CtfChallengeSolve, CtfEvent, CtfEventData, CtfEventDetail, CtfFlagResult, CtfMenu,
    CtfScoreboard, CtfSolve, CtfTeamOverview, CtfUserProfile,
};
use crate::models::ActionResponse;

#[derive(serde::Serialize)]
struct ProgressBody {
    status: Option<String>,
}

use super::HtbClient;

pub struct CtfApi<'a>(pub(crate) &'a HtbClient);

impl CtfApi<'_> {
    pub async fn profile(&self) -> Result<CtfUserProfile, HtbError> {
        self.0.get("/api/users/profile").await
    }

    pub async fn events(&self) -> Result<Vec<CtfEvent>, HtbError> {
        self.0.get("/api/ctfs").await
    }

    pub async fn event_details(&self, slug: &str) -> Result<CtfEventDetail, HtbError> {
        let encoded = super::encode_path(slug);
        self.0.get(&format!("/api/ctfs/details/{encoded}")).await
    }

    pub async fn event_data(&self, event_id: u64) -> Result<CtfEventData, HtbError> {
        self.0.get(&format!("/api/ctfs/{event_id}")).await
    }

    pub async fn menu(&self, event_id: u64) -> Result<CtfMenu, HtbError> {
        self.0.get(&format!("/api/ctfs/{event_id}/menu")).await
    }

    pub async fn submit_flag(
        &self,
        challenge_id: u64,
        flag: &str,
    ) -> Result<CtfFlagResult, HtbError> {
        self.0
            .post(
                "/api/flags/own",
                &json!({"challenge_id": challenge_id, "flag": flag}),
            )
            .await
    }

    pub async fn download_file(&self, challenge_id: u64) -> Result<Vec<u8>, HtbError> {
        self.0
            .get_bytes(&format!("/api/challenges/{challenge_id}/download"))
            .await
    }

    pub async fn container_start(&self, challenge_id: u64) -> Result<ActionResponse, HtbError> {
        self.0
            .post(
                "/api/challenges/containers/start",
                &json!({"id": challenge_id}),
            )
            .await
    }

    pub async fn container_stop(&self, challenge_id: u64) -> Result<ActionResponse, HtbError> {
        self.0
            .post(
                "/api/challenges/containers/stop",
                &json!({"id": challenge_id}),
            )
            .await
    }

    pub async fn scoreboard(&self, event_id: u64) -> Result<CtfScoreboard, HtbError> {
        self.0.get(&format!("/api/ctfs/scores/{event_id}")).await
    }

    pub async fn solves(&self, event_id: u64) -> Result<Vec<CtfSolve>, HtbError> {
        self.0.get(&format!("/api/ctfs/solves/{event_id}")).await
    }

    pub async fn challenge_solves(
        &self,
        challenge_id: u64,
    ) -> Result<Vec<CtfChallengeSolve>, HtbError> {
        self.0
            .get(&format!("/api/challenges/{challenge_id}/solves"))
            .await
    }

    pub async fn team_overview(&self, team_id: u64) -> Result<CtfTeamOverview, HtbError> {
        self.0.get(&format!("/api/teams/{team_id}/overview")).await
    }

    pub async fn associate_challenge(
        &self,
        challenge_id: u64,
        user_id: u64,
    ) -> Result<ActionResponse, HtbError> {
        self.0
            .get(&format!(
                "/api/challenges/{challenge_id}/associate/{user_id}"
            ))
            .await
    }

    pub async fn disassociate_challenge(
        &self,
        challenge_id: u64,
        user_id: u64,
    ) -> Result<ActionResponse, HtbError> {
        self.0
            .get(&format!(
                "/api/challenges/{challenge_id}/disassociate/{user_id}"
            ))
            .await
    }

    pub async fn set_challenge_progress(
        &self,
        challenge_id: u64,
        status: Option<&str>,
    ) -> Result<(), HtbError> {
        self.0
            .post_no_content(
                &format!("/api/challenges/{challenge_id}/progress"),
                &ProgressBody {
                    status: status.map(|s| s.to_string()),
                },
            )
            .await
    }
}
