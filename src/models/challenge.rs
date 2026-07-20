use serde::{Deserialize, Serialize};

use crate::output::Tabular;

#[derive(Debug, Deserialize, Serialize)]
pub struct Challenge {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub difficulty: Option<String>,
    #[serde(default)]
    pub state: Option<String>,
    #[serde(default)]
    pub category_id: Option<u64>,
    #[serde(default)]
    pub category_name: Option<String>,
    #[serde(default)]
    pub solves: u32,
    #[serde(default)]
    pub is_owned: bool,
    #[serde(default)]
    pub rating: Option<f64>,
    #[serde(default)]
    pub rating_count: Option<u32>,
    #[serde(default)]
    pub release_date: Option<String>,
    #[serde(default)]
    pub play_methods: Vec<String>,
    #[serde(default)]
    pub labels: Vec<super::machine::Label>,
}

#[derive(Debug, Deserialize)]
pub struct ChallengeDetailResponse {
    pub challenge: ChallengeDetail,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChallengeDetail {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub difficulty: Option<String>,
    #[serde(default)]
    pub points: Option<String>,
    #[serde(default)]
    pub state: Option<String>,
    #[serde(default)]
    pub solves: u32,
    #[serde(default)]
    pub stars: Option<f64>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub category_name: Option<String>,
    #[serde(default)]
    pub first_blood_user: Option<String>,
    #[serde(default)]
    pub first_blood_time: Option<String>,
    #[serde(default)]
    pub creator_name: Option<String>,
    #[serde(default)]
    pub download: bool,
    #[serde(default)]
    pub file_name: Option<String>,
    #[serde(default)]
    pub file_size: Option<String>,
    #[serde(default)]
    pub play_info: Option<ChallengePlayInfo>,
    #[serde(default)]
    pub play_methods: Vec<String>,
    #[serde(default)]
    pub release_date: Option<String>,
    #[serde(default, rename = "authUserSolve")]
    pub auth_user_solve: bool,
    #[serde(default)]
    pub experience_points: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChallengePlayInfo {
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub expires_at: Option<String>,
    #[serde(default)]
    pub ip: Option<String>,
    #[serde(default)]
    pub ports: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChallengeCategory {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub icon: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ChallengeCategoriesResponse {
    pub info: Vec<ChallengeCategory>,
}

#[derive(Debug, Deserialize)]
pub struct ChallengeDownloadResponse {
    pub url: String,
    #[serde(default)]
    pub expires_in: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct ChallengeOwnResponse {
    pub message: String,
    #[serde(default)]
    pub user_rank: Option<ChallengeOwnRank>,
}

#[derive(Debug, Deserialize)]
pub struct ChallengeOwnRank {
    #[serde(default)]
    pub changed: bool,
}

impl Tabular for Challenge {
    fn headers() -> Vec<&'static str> {
        vec![
            "ID",
            "Name",
            "Difficulty",
            "Category",
            "Solves",
            "Rating",
            "Owned",
            "State",
        ]
    }

    fn row(&self) -> Vec<String> {
        vec![
            self.id.to_string(),
            self.name.clone(),
            self.difficulty.clone().unwrap_or_default(),
            self.category_name.clone().unwrap_or_default(),
            self.solves.to_string(),
            self.rating.map(|r| format!("{r:.1}")).unwrap_or_default(),
            if self.is_owned { "✓" } else { "" }.to_string(),
            self.state.clone().unwrap_or_default(),
        ]
    }
}

impl Tabular for ChallengeCategory {
    fn headers() -> Vec<&'static str> {
        vec!["ID", "Name"]
    }

    fn row(&self) -> Vec<String> {
        vec![self.id.to_string(), self.name.clone()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Paginated;

    #[test]
    fn deserialize_challenges_list() {
        let json = include_str!("../../tests/fixtures/challenges-list.json");
        let result: Paginated<Challenge> = serde_json::from_str(json).unwrap();
        assert_eq!(result.data.len(), 15);
        assert_eq!(result.data[0].name, "SpookyPass");
        assert_eq!(result.data[0].category_name.as_deref(), Some("Reversing"));
    }

    #[test]
    fn deserialize_challenge_detail() {
        let json = include_str!("../../tests/fixtures/challenge-info-poly.json");
        let result: ChallengeDetailResponse = serde_json::from_str(json).unwrap();
        assert_eq!(result.challenge.name, "Poly");
        assert_eq!(result.challenge.difficulty.as_deref(), Some("Insane"));
        assert_eq!(result.challenge.category_name.as_deref(), Some("Reversing"));
    }

    #[test]
    fn deserialize_challenge_categories() {
        let json = include_str!("../../tests/fixtures/challenge-categories.json");
        let result: ChallengeCategoriesResponse = serde_json::from_str(json).unwrap();
        assert!(!result.info.is_empty());
        assert!(result.info.iter().any(|c| c.name == "Web"));
    }

    #[test]
    fn deserialize_challenge_own_success() {
        let json = include_str!("../../tests/fixtures/challenge-own-success.json");
        let result: ChallengeOwnResponse = serde_json::from_str(json).unwrap();
        assert_eq!(result.message, "Congratulations!");
    }
}
