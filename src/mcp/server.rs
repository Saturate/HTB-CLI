use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{Implementation, ServerCapabilities, ServerInfo};
use rmcp::{schemars, tool, tool_handler, tool_router, ServerHandler};

use crate::api::HtbClient;

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ListMachinesParams {
    #[schemars(description = "Filter by OS (e.g. 'linux', 'windows'). Omit to list all.")]
    pub os: Option<String>,
    #[schemars(
        description = "Filter by difficulty (e.g. 'easy', 'medium', 'hard', 'insane'). Omit to list all."
    )]
    pub difficulty: Option<String>,
    #[schemars(description = "Page number (default 1)")]
    pub page: Option<u32>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct MachineInfoParams {
    #[schemars(description = "Machine name or numeric ID")]
    pub name_or_id: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ListChallengesParams {
    #[schemars(
        description = "Filter by category (e.g. 'Web', 'Crypto', 'Reversing'). Omit to list all."
    )]
    pub category: Option<String>,
    #[schemars(description = "Page number (default 1)")]
    pub page: Option<u32>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ChallengeInfoParams {
    #[schemars(description = "Challenge slug (URL name)")]
    pub slug: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SubmitChallengeFlagParams {
    #[schemars(description = "Challenge ID (numeric)")]
    pub challenge_id: u64,
    #[schemars(description = "The flag string (e.g. 'HTB{...}')")]
    pub flag: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ChallengeIdParams {
    #[schemars(description = "Challenge ID (numeric)")]
    pub challenge_id: u64,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SubmitMachineFlagParams {
    #[schemars(description = "Machine name or numeric ID")]
    pub name_or_id: String,
    #[schemars(description = "The flag hash string")]
    pub flag: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SearchParams {
    #[schemars(description = "Search query")]
    pub query: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ListSherlocksParams {
    #[schemars(description = "Page number (default 1)")]
    pub page: Option<u32>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SherlockInfoParams {
    #[schemars(description = "Sherlock slug (URL name)")]
    pub slug: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SubmitSherlockFlagParams {
    #[schemars(description = "Sherlock ID (numeric)")]
    pub sherlock_id: u64,
    #[schemars(description = "Task ID within the Sherlock (numeric)")]
    pub task_id: u64,
    #[schemars(description = "The answer string")]
    pub flag: String,
}

#[derive(Clone)]
#[allow(dead_code)]
pub struct HtbMcp {
    client: HtbClient,
    tool_router: ToolRouter<Self>,
}

impl std::fmt::Debug for HtbMcp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HtbMcp").finish()
    }
}

impl HtbMcp {
    pub fn new(client: HtbClient) -> Self {
        Self {
            client,
            tool_router: Self::tool_router(),
        }
    }
}

fn to_json<T: serde::Serialize>(value: &T) -> Result<String, String> {
    serde_json::to_string_pretty(value).map_err(|e| e.to_string())
}

#[tool_router]
impl HtbMcp {
    #[tool(
        description = "Get the current authenticated user's profile info including rank, owns, and points."
    )]
    async fn get_user_profile(&self) -> Result<String, String> {
        let info = self
            .client
            .user()
            .current()
            .await
            .map_err(|e| e.to_string())?;
        let profile = self
            .client
            .user()
            .profile(info.id)
            .await
            .map_err(|e| e.to_string())?;
        to_json(&profile)
    }

    #[tool(
        description = "List machines on Hack The Box. Optionally filter by OS and difficulty. Returns name, OS, difficulty, rating, state, and own status."
    )]
    async fn list_machines(
        &self,
        Parameters(params): Parameters<ListMachinesParams>,
    ) -> Result<String, String> {
        let page = params.page.unwrap_or(1);
        let result = self
            .client
            .machines()
            .list(page, 100)
            .await
            .map_err(|e| e.to_string())?;
        let mut machines = result.data;

        if let Some(ref os) = params.os {
            machines.retain(|m| m.os.eq_ignore_ascii_case(os));
        }
        if let Some(ref diff) = params.difficulty {
            machines.retain(|m| {
                m.difficulty_text
                    .as_ref()
                    .is_some_and(|d| d.eq_ignore_ascii_case(diff))
            });
        }

        let summary: Vec<serde_json::Value> = machines
            .iter()
            .map(|m| {
                serde_json::json!({
                    "id": m.id,
                    "name": m.name,
                    "os": m.os,
                    "difficulty": m.difficulty_text,
                    "rating": m.rating,
                    "points": m.points,
                    "state": m.state,
                    "user_owned": m.auth_user_in_user_owns,
                    "root_owned": m.auth_user_in_root_owns,
                })
            })
            .collect();

        to_json(&serde_json::json!({
            "machines": summary,
            "page": result.meta.current_page,
            "total_pages": result.meta.last_page,
            "total": result.meta.total,
        }))
    }

    #[tool(description = "Get detailed information about a specific machine by name or ID.")]
    async fn get_machine_info(
        &self,
        Parameters(params): Parameters<MachineInfoParams>,
    ) -> Result<String, String> {
        let machine = self
            .client
            .machines()
            .profile(&params.name_or_id)
            .await
            .map_err(|e| e.to_string())?;
        to_json(&machine)
    }

    #[tool(
        description = "List challenges on Hack The Box. Optionally filter by category. Returns name, difficulty, category, solves, and own status."
    )]
    async fn list_challenges(
        &self,
        Parameters(params): Parameters<ListChallengesParams>,
    ) -> Result<String, String> {
        let page = params.page.unwrap_or(1);
        let result = self
            .client
            .challenges()
            .list(page, 100, None)
            .await
            .map_err(|e| e.to_string())?;
        let mut challenges = result.data;

        if let Some(ref cat) = params.category {
            challenges.retain(|c| {
                c.category_name
                    .as_ref()
                    .is_some_and(|cn| cn.eq_ignore_ascii_case(cat))
            });
        }

        let summary: Vec<serde_json::Value> = challenges
            .iter()
            .map(|c| {
                serde_json::json!({
                    "id": c.id,
                    "name": c.name,
                    "difficulty": c.difficulty,
                    "category": c.category_name,
                    "solves": c.solves,
                    "rating": c.rating,
                    "owned": c.is_owned,
                    "state": c.state,
                })
            })
            .collect();

        to_json(&serde_json::json!({
            "challenges": summary,
            "page": result.meta.current_page,
            "total_pages": result.meta.last_page,
            "total": result.meta.total,
        }))
    }

    #[tool(description = "Get detailed information about a specific challenge by slug name.")]
    async fn get_challenge_info(
        &self,
        Parameters(params): Parameters<ChallengeInfoParams>,
    ) -> Result<String, String> {
        let detail = self
            .client
            .challenges()
            .info(&params.slug)
            .await
            .map_err(|e| e.to_string())?;
        to_json(&detail)
    }

    #[tool(description = "Start a challenge container instance. Returns the instance ID.")]
    async fn start_challenge(
        &self,
        Parameters(params): Parameters<ChallengeIdParams>,
    ) -> Result<String, String> {
        let resp = self
            .client
            .challenges()
            .start(params.challenge_id)
            .await
            .map_err(|e| e.to_string())?;
        to_json(&resp)
    }

    #[tool(description = "Stop a running challenge container instance.")]
    async fn stop_challenge(
        &self,
        Parameters(params): Parameters<ChallengeIdParams>,
    ) -> Result<String, String> {
        let resp = self
            .client
            .challenges()
            .stop(params.challenge_id)
            .await
            .map_err(|e| e.to_string())?;
        to_json(&resp)
    }

    #[tool(description = "Submit a flag for a challenge. Returns whether the flag was correct.")]
    async fn submit_challenge_flag(
        &self,
        Parameters(params): Parameters<SubmitChallengeFlagParams>,
    ) -> Result<String, String> {
        let resp = self
            .client
            .challenges()
            .submit_flag(params.challenge_id, &params.flag)
            .await
            .map_err(|e| e.to_string())?;
        to_json(&resp)
    }

    #[tool(
        description = "Submit a flag for a machine (user or root). Resolves machine by name or ID."
    )]
    async fn submit_machine_flag(
        &self,
        Parameters(params): Parameters<SubmitMachineFlagParams>,
    ) -> Result<String, String> {
        let machine = self
            .client
            .machines()
            .profile(&params.name_or_id)
            .await
            .map_err(|e| e.to_string())?;
        let resp = self
            .client
            .machines()
            .submit_flag(machine.id, &params.flag)
            .await
            .map_err(|e| e.to_string())?;
        to_json(&resp)
    }

    #[tool(description = "Spawn/start a machine instance. Resolves machine by name or ID.")]
    async fn start_machine(
        &self,
        Parameters(params): Parameters<MachineInfoParams>,
    ) -> Result<String, String> {
        let machine = self
            .client
            .machines()
            .profile(&params.name_or_id)
            .await
            .map_err(|e| e.to_string())?;
        let resp = self
            .client
            .machines()
            .start(machine.id)
            .await
            .map_err(|e| e.to_string())?;
        to_json(&resp)
    }

    #[tool(
        description = "Stop a running machine instance. Resolves machine by name or ID, or stops the active machine if omitted."
    )]
    async fn stop_machine(
        &self,
        Parameters(params): Parameters<MachineInfoParams>,
    ) -> Result<String, String> {
        let machine = self
            .client
            .machines()
            .profile(&params.name_or_id)
            .await
            .map_err(|e| e.to_string())?;
        let resp = self
            .client
            .machines()
            .stop(machine.id)
            .await
            .map_err(|e| e.to_string())?;
        to_json(&resp)
    }

    #[tool(description = "Reset a machine instance. Resolves machine by name or ID.")]
    async fn reset_machine(
        &self,
        Parameters(params): Parameters<MachineInfoParams>,
    ) -> Result<String, String> {
        let machine = self
            .client
            .machines()
            .profile(&params.name_or_id)
            .await
            .map_err(|e| e.to_string())?;
        let resp = self
            .client
            .machines()
            .reset(machine.id)
            .await
            .map_err(|e| e.to_string())?;
        to_json(&resp)
    }

    #[tool(
        description = "Extend the expiration timer on a running machine. Resolves by name or ID."
    )]
    async fn extend_machine(
        &self,
        Parameters(params): Parameters<MachineInfoParams>,
    ) -> Result<String, String> {
        let machine = self
            .client
            .machines()
            .profile(&params.name_or_id)
            .await
            .map_err(|e| e.to_string())?;
        let resp = self
            .client
            .machines()
            .extend(machine.id)
            .await
            .map_err(|e| e.to_string())?;
        to_json(&resp)
    }

    #[tool(description = "Show the currently active/spawned machine, if any.")]
    async fn get_active_machine(&self) -> Result<String, String> {
        let active = self
            .client
            .machines()
            .active()
            .await
            .map_err(|e| e.to_string())?;
        match active {
            Some(vm) => to_json(&vm),
            None => Ok(r#"{"active": false, "message": "No active machine"}"#.to_string()),
        }
    }

    #[tool(
        description = "List Sherlocks (blue-team investigation labs). Returns name, difficulty, category, solves, and completion status."
    )]
    async fn list_sherlocks(
        &self,
        Parameters(params): Parameters<ListSherlocksParams>,
    ) -> Result<String, String> {
        let page = params.page.unwrap_or(1);
        let result = self
            .client
            .sherlocks()
            .list(page, 100)
            .await
            .map_err(|e| e.to_string())?;

        let summary: Vec<serde_json::Value> = result
            .data
            .iter()
            .map(|s| {
                serde_json::json!({
                    "id": s.id,
                    "name": s.name,
                    "difficulty": s.difficulty,
                    "category": s.category_name,
                    "solves": s.solves,
                    "rating": s.rating,
                    "progress": s.progress,
                    "owned": s.is_owned,
                    "state": s.state,
                })
            })
            .collect();

        to_json(&serde_json::json!({
            "sherlocks": summary,
            "page": result.meta.current_page,
            "total_pages": result.meta.last_page,
            "total": result.meta.total,
        }))
    }

    #[tool(description = "Get detailed information about a Sherlock by slug name.")]
    async fn get_sherlock_info(
        &self,
        Parameters(params): Parameters<SherlockInfoParams>,
    ) -> Result<String, String> {
        let sherlock = self
            .client
            .sherlocks()
            .info(&params.slug)
            .await
            .map_err(|e| e.to_string())?;
        to_json(&sherlock)
    }

    #[tool(description = "Submit an answer for a Sherlock task.")]
    async fn submit_sherlock_flag(
        &self,
        Parameters(params): Parameters<SubmitSherlockFlagParams>,
    ) -> Result<String, String> {
        let resp = self
            .client
            .sherlocks()
            .submit_flag(params.sherlock_id, params.task_id, &params.flag)
            .await
            .map_err(|e| e.to_string())?;
        to_json(&resp)
    }

    #[tool(description = "List all seasons and show which is currently active.")]
    async fn list_seasons(&self) -> Result<String, String> {
        let seasons = self
            .client
            .seasons()
            .list()
            .await
            .map_err(|e| e.to_string())?;
        to_json(&seasons)
    }

    #[tool(description = "Show VPN connection status for the authenticated user.")]
    async fn vpn_status(&self) -> Result<String, String> {
        let connections = self
            .client
            .vpn()
            .connections()
            .await
            .map_err(|e| e.to_string())?;
        to_json(&connections)
    }

    #[tool(description = "Search across machines, challenges, and users on Hack The Box.")]
    async fn search(&self, Parameters(params): Parameters<SearchParams>) -> Result<String, String> {
        let results = self
            .client
            .search()
            .fetch(&params.query)
            .await
            .map_err(|e| e.to_string())?;
        to_json(&results)
    }
}

#[tool_handler]
impl ServerHandler for HtbMcp {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::new("htb-cli", env!("CARGO_PKG_VERSION")))
            .with_instructions(
                "HTB-CLI provides access to the Hack The Box platform. \
                 Use get_user_profile to check auth. \
                 Machines: list_machines, get_machine_info, start_machine, stop_machine, \
                 reset_machine, extend_machine, get_active_machine, submit_machine_flag. \
                 Challenges: list_challenges, get_challenge_info, start_challenge, \
                 stop_challenge, submit_challenge_flag. \
                 Sherlocks: list_sherlocks, get_sherlock_info, submit_sherlock_flag. \
                 Other: list_seasons, vpn_status, search.",
            )
    }
}
