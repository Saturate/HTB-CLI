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
        vec!["ID", "Name", "Status", "Format", "Players", "Team Size", "Joined"]
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

#[cfg(test)]
mod tests {
    use super::*;

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
