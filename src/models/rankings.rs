use serde::{Deserialize, Serialize};

use crate::output::Tabular;

#[derive(Debug, Deserialize)]
pub struct RankingsUsersResponse {
    #[allow(dead_code)]
    pub status: bool,
    pub data: Vec<RankingUserEntry>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RankingUserEntry {
    #[serde(default)]
    pub rank: u32,
    #[serde(default)]
    pub points: u32,
    #[serde(default)]
    pub user_owns: u32,
    #[serde(default)]
    pub root_owns: u32,
    #[serde(default)]
    pub challenge_owns: u32,
    #[serde(default)]
    pub user_bloods: u32,
    #[serde(default)]
    pub root_bloods: u32,
    #[serde(default)]
    pub challenge_bloods: u32,
    #[serde(default)]
    pub fortress: u32,
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub country: Option<String>,
    #[serde(default)]
    pub level: Option<String>,
    #[serde(default)]
    pub ranks_diff: i32,
}

impl Tabular for RankingUserEntry {
    fn headers() -> Vec<&'static str> {
        vec![
            "#",
            "Name",
            "Level",
            "Points",
            "User Owns",
            "Root Owns",
            "Challenge Owns",
            "User Bloods",
            "Root Bloods",
            "Country",
        ]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.rank.to_string(),
            self.name.clone(),
            self.level.clone().unwrap_or_default(),
            self.points.to_string(),
            self.user_owns.to_string(),
            self.root_owns.to_string(),
            self.challenge_owns.to_string(),
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
    fn deserialize_rankings_users() {
        let json = include_str!("../../tests/fixtures/rankings-users.json");
        let resp: RankingsUsersResponse = serde_json::from_str(json).unwrap();
        assert!(resp.status);
        assert_eq!(resp.data.len(), 3);
        assert_eq!(resp.data[0].name, "topUser");
        assert_eq!(resp.data[0].rank, 1);
        assert_eq!(resp.data[1].country.as_deref(), Some("US"));
        assert!(resp.data[2].country.is_none());
        assert_eq!(resp.data[2].ranks_diff, 2);
    }
}
