use serde::{Deserialize, Serialize};

use crate::output::Tabular;

#[derive(Debug, Deserialize)]
pub struct SeasonListResponse {
    pub data: Vec<Season>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Season {
    pub id: u32,
    pub name: String,
    #[serde(default)]
    pub subtitle: Option<String>,
    #[serde(default)]
    pub start_date: Option<String>,
    #[serde(default)]
    pub end_date: Option<String>,
    #[serde(default)]
    pub state: Option<String>,
    #[serde(default)]
    pub active: bool,
    #[serde(default)]
    pub weeks: Option<u32>,
    #[serde(default)]
    pub current_week: Option<u32>,
    #[serde(default)]
    pub players: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct SeasonUserRanksResponse {
    pub data: Vec<SeasonUserRank>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SeasonUserRank {
    #[serde(default)]
    pub league: Option<String>,
    #[serde(default)]
    pub rank: Option<u32>,
    #[serde(default)]
    pub total_ranks: Option<u32>,
    #[serde(default)]
    pub rank_suffix: Option<String>,
    #[serde(default)]
    pub total_season_points: u32,
    #[serde(default)]
    pub total_season_bloods: u32,
    #[serde(default)]
    pub user_owns: Option<u32>,
    #[serde(default)]
    pub root_owns: Option<u32>,
    #[serde(default)]
    pub season_id: u32,
    #[serde(default)]
    pub season_name: Option<String>,
}

impl Tabular for Season {
    fn headers() -> Vec<&'static str> {
        vec!["ID", "Name", "State", "Weeks", "Players", "Active"]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.id.to_string(),
            self.name.clone(),
            self.state.clone().unwrap_or_default(),
            self.weeks.map(|w| w.to_string()).unwrap_or_default(),
            self.players.map(|p| p.to_string()).unwrap_or_default(),
            if self.active { "✓" } else { "" }.to_string(),
        ]
    }
}

#[derive(Debug, Deserialize)]
pub struct SeasonMachinesResponse {
    pub data: Vec<SeasonMachine>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SeasonMachine {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub os: Option<String>,
    #[serde(default)]
    pub difficulty: Option<u32>,
    #[serde(default)]
    pub difficulty_text: Option<String>,
    #[serde(default)]
    pub points: u32,
    #[serde(default)]
    pub user_owns_count: u32,
    #[serde(default)]
    pub root_owns_count: u32,
    #[serde(default)]
    pub auth_user_in_user_owns: Option<bool>,
    #[serde(default)]
    pub auth_user_in_root_owns: Option<bool>,
    #[serde(default)]
    pub ip: Option<String>,
    #[serde(default)]
    pub state: Option<String>,
}

impl Tabular for SeasonMachine {
    fn headers() -> Vec<&'static str> {
        vec![
            "ID",
            "Name",
            "OS",
            "Difficulty",
            "Points",
            "User Owns",
            "Root Owns",
            "State",
        ]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.id.to_string(),
            self.name.clone(),
            self.os.clone().unwrap_or_default(),
            self.difficulty_text
                .clone()
                .unwrap_or_else(|| self.difficulty.map(|d| d.to_string()).unwrap_or_default()),
            self.points.to_string(),
            self.user_owns_count.to_string(),
            self.root_owns_count.to_string(),
            self.state.clone().unwrap_or_default(),
        ]
    }
}

impl Tabular for SeasonUserRank {
    fn headers() -> Vec<&'static str> {
        vec![
            "Season",
            "League",
            "Rank",
            "Points",
            "User Owns",
            "Root Owns",
        ]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.season_name.clone().unwrap_or_default(),
            self.league.clone().unwrap_or_else(|| "-".into()),
            self.rank
                .map(|r| r.to_string())
                .unwrap_or_else(|| "-".into()),
            self.total_season_points.to_string(),
            self.user_owns
                .map(|o| o.to_string())
                .unwrap_or_else(|| "-".into()),
            self.root_owns
                .map(|o| o.to_string())
                .unwrap_or_else(|| "-".into()),
        ]
    }
}

#[derive(Debug, Deserialize)]
pub struct SeasonLeaderboardResponse {
    pub data: Vec<SeasonLeaderboardEntry>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SeasonLeaderboardEntry {
    #[serde(default)]
    pub resource_id: u64,
    pub name: String,
    #[serde(default)]
    pub rank: u32,
    #[serde(default)]
    pub league_rank: Option<String>,
    #[serde(default)]
    pub points: u32,
    #[serde(default)]
    pub user_owns: u32,
    #[serde(default)]
    pub root_owns: u32,
    #[serde(default)]
    pub user_bloods: u32,
    #[serde(default)]
    pub root_bloods: u32,
    #[serde(default)]
    pub country: Option<String>,
    #[serde(default)]
    pub country_name: Option<String>,
    #[serde(default)]
    pub last_own: Option<String>,
}

impl Tabular for SeasonLeaderboardEntry {
    fn headers() -> Vec<&'static str> {
        vec![
            "#",
            "Name",
            "Points",
            "User Owns",
            "Root Owns",
            "User Bloods",
            "Root Bloods",
            "Country",
        ]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.rank.to_string(),
            self.name.clone(),
            self.points.to_string(),
            self.user_owns.to_string(),
            self.root_owns.to_string(),
            self.user_bloods.to_string(),
            self.root_bloods.to_string(),
            self.country.clone().unwrap_or_default(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_season_list() {
        let json = include_str!("../../tests/fixtures/season-list.json");
        let result: SeasonListResponse = serde_json::from_str(json).unwrap();
        assert!(!result.data.is_empty());
        let active = result.data.iter().find(|s| s.active);
        assert!(active.is_some());
        assert_eq!(active.unwrap().name, "Season 11");
    }

    #[test]
    fn deserialize_season_user_ranks() {
        let json = include_str!("../../tests/fixtures/season-user-ranks.json");
        let result: SeasonUserRanksResponse = serde_json::from_str(json).unwrap();
        assert!(!result.data.is_empty());
        let first = &result.data[0];
        assert_eq!(first.league.as_deref(), Some("Silver"));
        assert_eq!(first.total_season_points, 110);
    }

    #[test]
    fn deserialize_season_machines() {
        let json = include_str!("../../tests/fixtures/season-machines.json");
        let resp: SeasonMachinesResponse = serde_json::from_str(json).unwrap();
        assert!(!resp.data.is_empty());
        assert_eq!(resp.data[0].os.as_deref(), Some("Linux"));
    }

    #[test]
    fn deserialize_season_leaderboard() {
        let json = include_str!("../../tests/fixtures/season-leaderboard.json");
        let resp: SeasonLeaderboardResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.data.len(), 3);
        assert_eq!(resp.data[0].name, "ultimatekristency");
        assert_eq!(resp.data[0].rank, 1);
        assert_eq!(resp.data[0].resource_id, 2751322);
        assert_eq!(resp.data[0].league_rank.as_deref(), Some("Platinum"));
        assert_eq!(resp.data[1].country.as_deref(), Some("US"));
        assert!(resp.data[2].country.is_none());
    }

    #[test]
    fn deserialize_unranked_season() {
        let json = include_str!("../../tests/fixtures/season-user-ranks.json");
        let result: SeasonUserRanksResponse = serde_json::from_str(json).unwrap();
        let unranked = result.data.iter().find(|r| r.league.is_none());
        assert!(unranked.is_some());
        assert!(unranked.unwrap().rank.is_none());
    }
}
