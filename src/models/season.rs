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
    fn deserialize_unranked_season() {
        let json = include_str!("../../tests/fixtures/season-user-ranks.json");
        let result: SeasonUserRanksResponse = serde_json::from_str(json).unwrap();
        let unranked = result.data.iter().find(|r| r.league.is_none());
        assert!(unranked.is_some());
        assert!(unranked.unwrap().rank.is_none());
    }
}
