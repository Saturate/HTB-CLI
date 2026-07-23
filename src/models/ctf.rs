use serde::{Deserialize, Serialize};

use crate::output::Tabular;

#[derive(Debug, Deserialize, Serialize)]
pub struct CtfUserProfile {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub full_name: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub timezone: Option<String>,
    #[serde(default, rename = "hasAnyTeam")]
    pub has_any_team: bool,
    #[serde(default)]
    pub avatar: Option<String>,
}

impl Tabular for CtfUserProfile {
    fn headers() -> Vec<&'static str> {
        vec!["ID", "Name", "Email", "Timezone"]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.id.to_string(),
            self.name.clone(),
            self.email.clone().unwrap_or_default(),
            self.timezone.clone().unwrap_or_default(),
        ]
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CtfEvent {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub slug: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub starts_at: Option<String>,
    #[serde(default)]
    pub ends_at: Option<String>,
    #[serde(default)]
    pub format: Option<String>,
    #[serde(default)]
    pub team_size: Option<u32>,
    #[serde(default)]
    pub players: Option<u32>,
    #[serde(default)]
    pub org_name: Option<String>,
    #[serde(default, rename = "hasJoined")]
    pub has_joined: u32,
    #[serde(default, rename = "joinedTeam")]
    pub joined_team: Option<String>,
    #[serde(default, rename = "membersJoined")]
    pub members_joined: Option<String>,
}

impl Tabular for CtfEvent {
    fn headers() -> Vec<&'static str> {
        vec![
            "ID",
            "Name",
            "Status",
            "Format",
            "Players",
            "Team Size",
            "Joined",
        ]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.id.to_string(),
            self.name.clone(),
            self.status.clone().unwrap_or_default(),
            self.format.clone().unwrap_or_default(),
            self.players.map(|p| p.to_string()).unwrap_or_default(),
            self.team_size.map(|t| t.to_string()).unwrap_or_default(),
            if self.has_joined > 0 {
                self.joined_team.clone().unwrap_or_else(|| "Yes".into())
            } else {
                String::new()
            },
        ]
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CtfEventDetail {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub slug: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default, rename = "startDate")]
    pub start_date: Option<String>,
    #[serde(default, rename = "endDate")]
    pub end_date: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default, rename = "type")]
    pub event_type: Option<String>,
    #[serde(default)]
    pub format: Option<String>,
    #[serde(default)]
    pub location: Option<String>,
    #[serde(default, rename = "playersJoined")]
    pub players_joined: Option<u32>,
    #[serde(default, rename = "teamsJoined")]
    pub teams_joined: Option<u32>,
    #[serde(default)]
    pub challenges: Option<u32>,
    #[serde(default, rename = "challengeCategories")]
    pub challenge_categories: Option<u32>,
    #[serde(default, rename = "maxTeamSize")]
    pub max_team_size: Option<u32>,
    #[serde(default, rename = "prizePool")]
    pub prize_pool: Option<String>,
    #[serde(default)]
    pub featured: bool,
    #[serde(default)]
    pub ai_usage_policy: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CtfEventData {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub hide_scoreboard: u32,
    #[serde(default)]
    pub participating_team: Option<CtfParticipatingTeam>,
    #[serde(default)]
    pub challenges: Vec<CtfChallenge>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CtfParticipatingTeam {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub points: Option<u32>,
    #[serde(default)]
    pub solved_challenges: Option<u32>,
    #[serde(default)]
    pub total_challenges: Option<u32>,
    #[serde(default)]
    pub owned_flags: Option<u32>,
    #[serde(default)]
    pub total_flags: Option<u32>,
    #[serde(default)]
    pub rank: Option<u32>,
    #[serde(default, rename = "isCaptain")]
    pub is_captain: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CtfChallenge {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub creator: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub challenge_category_id: Option<u64>,
    #[serde(default)]
    pub difficulty: Option<String>,
    #[serde(default)]
    pub filename: Option<String>,
    #[serde(default, rename = "hasDocker")]
    pub has_docker: Option<u32>,
    #[serde(default)]
    pub docker_online: Option<String>,
    #[serde(default)]
    pub docker_ports: Option<String>,
    #[serde(default)]
    pub points: Option<u32>,
    #[serde(default)]
    pub solves: Option<u32>,
    #[serde(default)]
    pub solved: bool,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default, rename = "flagsInfo")]
    pub flags_info: Vec<CtfFlagInfo>,
}

impl Tabular for CtfChallenge {
    fn headers() -> Vec<&'static str> {
        vec![
            "ID",
            "Name",
            "Difficulty",
            "Points",
            "Solves",
            "Docker",
            "Download",
            "Flags",
            "Solved",
        ]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.id.to_string(),
            self.name.clone(),
            self.difficulty.clone().unwrap_or_default(),
            self.points.map(|p| p.to_string()).unwrap_or_default(),
            self.solves.map(|s| s.to_string()).unwrap_or_default(),
            if self.has_docker.unwrap_or(0) > 0 {
                "Yes"
            } else {
                ""
            }
            .to_string(),
            if self.filename.is_some() { "Yes" } else { "" }.to_string(),
            self.flags_info.len().to_string(),
            if self.solved { "Yes" } else { "" }.to_string(),
        ]
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CtfFlagInfo {
    pub flag_id: u64,
    #[serde(default)]
    pub identifier: Option<String>,
    #[serde(default)]
    pub question: Option<String>,
    #[serde(default)]
    pub solved: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CtfMenu {
    pub id: u64,
    #[serde(default, rename = "userCanViewScoreboard")]
    pub user_can_view_scoreboard: Option<u32>,
    #[serde(default)]
    pub status: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CtfScoreboard {
    #[serde(default)]
    pub ctf_name: Option<String>,
    #[serde(default)]
    pub ctf_teams: Option<u32>,
    #[serde(default)]
    pub ctf_players: Option<u32>,
    #[serde(default)]
    pub is_ended: bool,
    #[serde(default)]
    pub participating_team: Option<CtfScoreboardTeam>,
    #[serde(default)]
    pub scores: Vec<CtfTeamScore>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CtfScoreboardTeam {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub position: Option<u32>,
    #[serde(default)]
    pub points: Option<u32>,
    #[serde(default)]
    pub solved_challenges: Option<u32>,
    #[serde(default)]
    pub total_challenges: Option<u32>,
    #[serde(default)]
    pub owned_flags: Option<u32>,
    #[serde(default)]
    pub first_bloods: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CtfTeamScore {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub country_code: Option<String>,
    #[serde(default)]
    pub points: Option<u32>,
    #[serde(default)]
    pub owned_flags: Option<u32>,
    #[serde(default)]
    pub first_bloods: Option<u32>,
}

impl Tabular for CtfTeamScore {
    fn headers() -> Vec<&'static str> {
        vec!["#", "Team", "Country", "Points", "Flags", "Bloods"]
    }

    fn row(&self) -> Vec<String> {
        vec![
            String::new(), // rank filled by display
            self.name.clone(),
            self.country_code.clone().unwrap_or_default(),
            self.points.map(|p| p.to_string()).unwrap_or_default(),
            self.owned_flags.map(|f| f.to_string()).unwrap_or_default(),
            self.first_bloods.map(|b| b.to_string()).unwrap_or_default(),
        ]
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CtfSolve {
    #[serde(default)]
    pub challenge_name: Option<String>,
    #[serde(default)]
    pub challenge_category: Option<String>,
    #[serde(default)]
    pub team_name: Option<String>,
    #[serde(default)]
    pub user_name: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
}

impl Tabular for CtfSolve {
    fn headers() -> Vec<&'static str> {
        vec!["Challenge", "Category", "Team", "Player", "Time"]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.challenge_name.clone().unwrap_or_default(),
            self.challenge_category.clone().unwrap_or_default(),
            self.team_name.clone().unwrap_or_default(),
            self.user_name.clone().unwrap_or_default(),
            self.created_at.clone().unwrap_or_default(),
        ]
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CtfChallengeSolve {
    #[serde(default)]
    pub team_id: Option<u64>,
    #[serde(default)]
    pub team_name: Option<String>,
    #[serde(default)]
    pub date_of_latest_own: Option<String>,
    #[serde(default)]
    pub user_of_latest_own: Option<CtfSolveUser>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CtfSolveUser {
    pub name: String,
}

impl Tabular for CtfChallengeSolve {
    fn headers() -> Vec<&'static str> {
        vec!["Team", "Player", "Time"]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.team_name.clone().unwrap_or_default(),
            self.user_of_latest_own
                .as_ref()
                .map(|u| u.name.clone())
                .unwrap_or_default(),
            self.date_of_latest_own.clone().unwrap_or_default(),
        ]
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CtfFlagResult {
    pub message: String,
    #[serde(default)]
    pub points: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_ctf_events_fixture() {
        let json = include_str!("../../tests/fixtures/ctf/events.json");
        let events: Vec<CtfEvent> = serde_json::from_str(json).unwrap();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].name, "CTF Try Out");
        assert_eq!(events[0].has_joined, 1);
        assert_eq!(events[1].status.as_deref(), Some("Upcoming"));
    }

    #[test]
    fn deserialize_ctf_event_detail_fixture() {
        let json = include_str!("../../tests/fixtures/ctf/event-detail.json");
        let detail: CtfEventDetail = serde_json::from_str(json).unwrap();
        assert_eq!(detail.id, 1434);
        assert_eq!(detail.players_joined, Some(1200));
        assert_eq!(detail.challenges, Some(50));
    }

    #[test]
    fn deserialize_ctf_event_data_fixture() {
        let json = include_str!("../../tests/fixtures/ctf/event-data.json");
        let data: CtfEventData = serde_json::from_str(json).unwrap();
        assert_eq!(data.challenges.len(), 2);
        assert_eq!(data.challenges[0].name, "An unusual sighting");
        assert!(data.challenges[0].solved);
        assert!(!data.challenges[1].solved);
        assert_eq!(data.challenges[1].has_docker, Some(1));
        let team = data.participating_team.unwrap();
        assert_eq!(team.rank, Some(42));
    }

    #[test]
    fn deserialize_ctf_scoreboard_fixture() {
        let json = include_str!("../../tests/fixtures/ctf/scoreboard.json");
        let sb: CtfScoreboard = serde_json::from_str(json).unwrap();
        assert_eq!(sb.scores.len(), 2);
        assert_eq!(sb.scores[0].points, Some(15000));
        let team = sb.participating_team.unwrap();
        assert_eq!(team.position, Some(42));
    }

    #[test]
    fn deserialize_ctf_solves_fixture() {
        let json = include_str!("../../tests/fixtures/ctf/solves.json");
        let solves: Vec<CtfSolve> = serde_json::from_str(json).unwrap();
        assert_eq!(solves.len(), 2);
        assert_eq!(solves[0].challenge_name.as_deref(), Some("LootStash"));
    }

    #[test]
    fn deserialize_ctf_user_profile_fixture() {
        let json = include_str!("../../tests/fixtures/ctf/user-profile.json");
        let profile: CtfUserProfile = serde_json::from_str(json).unwrap();
        assert_eq!(profile.name, "LANGSOMT");
        assert!(profile.has_any_team);
    }

    #[test]
    fn deserialize_ctf_menu_fixture() {
        let json = include_str!("../../tests/fixtures/ctf/menu.json");
        let menu: CtfMenu = serde_json::from_str(json).unwrap();
        assert_eq!(menu.id, 1434);
        assert_eq!(menu.user_can_view_scoreboard, Some(1));
    }

    #[test]
    fn deserialize_ctf_event() {
        let json = r#"{
            "id": 1434,
            "name": "CTF Try Out",
            "slug": "ctf-try-out-1434",
            "status": "Ongoing",
            "starts_at": "2026-01-01T00:00:00.000000Z",
            "ends_at": "2026-12-31T12:00:00.000000Z",
            "format": "Jeopardy",
            "team_mode": "user_organized",
            "team_size": 5,
            "players": 1200,
            "private": 0,
            "university": false,
            "isBusiness": false,
            "featured": false,
            "banner": "https://example.com/banner.png",
            "org_name": "Hack The Box",
            "org_logo": "https://example.com/logo.png",
            "canPlay": true,
            "canJoin": true,
            "hasJoined": 1,
            "joinedTeam": "v1olet",
            "membersJoined": "4/5",
            "hide_scoreboard": 0,
            "mcp_access_mode": "no_mcp"
        }"#;
        let event: CtfEvent = serde_json::from_str(json).unwrap();
        assert_eq!(event.id, 1434);
        assert_eq!(event.name, "CTF Try Out");
        assert_eq!(event.status.as_deref(), Some("Ongoing"));
        assert_eq!(event.has_joined, 1);
        assert_eq!(event.joined_team.as_deref(), Some("v1olet"));
    }

    #[test]
    fn deserialize_ctf_event_detail() {
        let json = r#"{
            "id": 1434,
            "name": "CTF Try Out",
            "slug": "ctf-try-out-1434",
            "status": "Ongoing",
            "startDate": "01 Jan 2026, 00:00 AM UTC",
            "endDate": "31 Dec 2026, 12:00 PM UTC",
            "description": "<p>A practice CTF</p>",
            "type": "Public",
            "format": "Jeopardy",
            "location": "Online",
            "playersJoined": 1200,
            "teamsJoined": 300,
            "challenges": 50,
            "challengeCategories": 10,
            "maxTeamSize": 5,
            "prizePool": null,
            "featured": false,
            "ai_usage_policy": null
        }"#;
        let detail: CtfEventDetail = serde_json::from_str(json).unwrap();
        assert_eq!(detail.id, 1434);
        assert_eq!(detail.players_joined, Some(1200));
        assert_eq!(detail.teams_joined, Some(300));
        assert_eq!(detail.challenges, Some(50));
    }

    #[test]
    fn deserialize_ctf_event_data_with_challenges() {
        let json = r#"{
            "id": 1434,
            "name": "CTF Try Out",
            "status": "Ongoing",
            "hide_scoreboard": 0,
            "participating_team": {
                "id": 321013,
                "name": "v1olet",
                "points": 5000,
                "solved_challenges": 10,
                "total_challenges": 50,
                "owned_flags": 10,
                "total_flags": 60,
                "rank": 42,
                "isCaptain": false
            },
            "challenges": [{
                "id": 31855,
                "name": "Test Challenge",
                "creator": "author",
                "description": "A test",
                "challenge_category_id": 2,
                "difficulty": "Easy",
                "filename": "test.zip",
                "hasDocker": 1,
                "docker_online": null,
                "docker_ports": null,
                "points": 500,
                "solves": 42,
                "solved": true,
                "status": "in progress",
                "flagsInfo": [
                    {"flag_id": 1, "identifier": null, "question": null, "solved": true}
                ],
                "machine": null,
                "team_solves": []
            }]
        }"#;
        let data: CtfEventData = serde_json::from_str(json).unwrap();
        assert_eq!(data.id, 1434);
        assert_eq!(data.challenges.len(), 1);
        assert_eq!(data.challenges[0].name, "Test Challenge");
        assert!(data.challenges[0].solved);
        assert_eq!(data.challenges[0].flags_info.len(), 1);
        let team = data.participating_team.unwrap();
        assert_eq!(team.rank, Some(42));
        assert_eq!(team.points, Some(5000));
    }

    #[test]
    fn deserialize_ctf_scoreboard() {
        let json = r#"{
            "ctf_name": "CTF Try Out",
            "ctf_teams": 300,
            "ctf_players": 1200,
            "is_ended": false,
            "participating_team": {
                "id": 321013,
                "name": "v1olet",
                "position": 42,
                "points": 5000,
                "solved_challenges": 10,
                "total_challenges": 50,
                "owned_flags": 10,
                "first_bloods": 2
            },
            "scores": [{
                "id": 1,
                "name": "Top Team",
                "country_code": "US",
                "points": 10000,
                "owned_flags": 50,
                "first_bloods": 5
            }]
        }"#;
        let sb: CtfScoreboard = serde_json::from_str(json).unwrap();
        assert_eq!(sb.ctf_teams, Some(300));
        assert_eq!(sb.scores.len(), 1);
        assert_eq!(sb.scores[0].points, Some(10000));
        let team = sb.participating_team.unwrap();
        assert_eq!(team.position, Some(42));
    }

    #[test]
    fn deserialize_ctf_solve() {
        let json = r#"{
            "challenge_name": "LootStash",
            "challenge_category": "Reversing",
            "team_name": "v1olet",
            "user_name": "LANGSOMT",
            "created_at": "2026-07-22T11:12:05.000000Z"
        }"#;
        let solve: CtfSolve = serde_json::from_str(json).unwrap();
        assert_eq!(solve.challenge_name.as_deref(), Some("LootStash"));
        assert_eq!(solve.team_name.as_deref(), Some("v1olet"));
    }

    #[test]
    fn deserialize_ctf_flag_result() {
        let json = r#"{"message": "Correct flag!", "points": 725}"#;
        let result: CtfFlagResult = serde_json::from_str(json).unwrap();
        assert_eq!(result.message, "Correct flag!");
        assert_eq!(result.points, Some(725));
    }

    #[test]
    fn deserialize_ctf_user_profile() {
        let json = r#"{
            "id": 1084837,
            "name": "LANGSOMT",
            "full_name": "Allan Kimmer Jensen",
            "email": "test@example.com",
            "timezone": "Europe/London",
            "hasAnyTeam": true,
            "hasNormalTeam": true,
            "avatar": null,
            "device_id": "abc"
        }"#;
        let profile: CtfUserProfile = serde_json::from_str(json).unwrap();
        assert_eq!(profile.id, 1084837);
        assert_eq!(profile.name, "LANGSOMT");
        assert!(profile.has_any_team);
    }
}
