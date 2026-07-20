use serde_json::json;

use crate::error::HtbError;
use crate::models::machine::{
    ActiveVmInfo, ActiveVmResponse, Machine, MachineProfileResponse, TodoListResponse,
};
use crate::models::{ActionResponse, Paginated};

use super::HtbClient;

pub struct MachineApi<'a>(pub(crate) &'a HtbClient);

impl MachineApi<'_> {
    pub async fn list(&self, page: u32, per_page: u32) -> Result<Paginated<Machine>, HtbError> {
        self.0
            .get(&format!("/api/v5/machines?per_page={per_page}&page={page}"))
            .await
    }

    pub async fn profile(&self, name_or_id: &str) -> Result<Machine, HtbError> {
        let resp: MachineProfileResponse = self
            .0
            .get(&format!("/api/v4/machine/profile/{name_or_id}"))
            .await?;
        Ok(resp.info)
    }

    pub async fn start(&self, machine_id: u64) -> Result<ActionResponse, HtbError> {
        self.0
            .post(&format!("/api/v4/machine/play/{machine_id}"), &json!({}))
            .await
    }

    pub async fn stop(&self) -> Result<ActionResponse, HtbError> {
        self.0.post("/api/v4/machine/stop", &json!({})).await
    }

    pub async fn reset(&self, machine_id: u64) -> Result<ActionResponse, HtbError> {
        self.0
            .post("/api/v4/vm/reset", &json!({"machine_id": machine_id}))
            .await
    }

    pub async fn submit_flag(
        &self,
        machine_id: u64,
        flag: &str,
        difficulty: u32,
    ) -> Result<ActionResponse, HtbError> {
        self.0
            .post(
                "/api/v4/machine/own",
                &json!({"flag": flag, "id": machine_id, "difficulty": difficulty}),
            )
            .await
    }

    pub async fn active(&self) -> Result<Option<ActiveVmInfo>, HtbError> {
        let resp: ActiveVmResponse = self.0.get("/api/v5/virtual_machine/active").await?;
        Ok(resp.info)
    }

    pub async fn todo_list(&self) -> Result<Vec<Machine>, HtbError> {
        let resp: TodoListResponse = self.0.get("/api/v4/machine/todo").await?;
        Ok(resp.info)
    }

    pub async fn todo_toggle(&self, machine_id: u64) -> Result<ActionResponse, HtbError> {
        self.0
            .post(
                &format!("/api/v4/machine/todo/update/{machine_id}"),
                &json!({}),
            )
            .await
    }
}
