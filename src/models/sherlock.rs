use serde::{Deserialize, Serialize};

use crate::output::Tabular;

#[derive(Debug, Deserialize, Serialize)]
pub struct Sherlock {
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
    pub progress: Option<u32>,
    #[serde(default)]
    pub release_date: Option<String>,
    #[serde(default)]
    pub play_methods: Vec<String>,
    #[serde(default)]
    pub labels: Vec<super::machine::Label>,
}

#[derive(Debug, Deserialize)]
pub struct SherlockCategoriesResponse {
    pub info: Vec<SherlockCategory>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SherlockCategory {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub icon: Option<String>,
}

impl Tabular for Sherlock {
    fn headers() -> Vec<&'static str> {
        vec![
            "ID",
            "Name",
            "Difficulty",
            "Category",
            "Solves",
            "Rating",
            "Progress",
            "State",
        ]
    }

    fn row(&self) -> Vec<String> {
        let progress = self.progress.map(|p| format!("{p}%")).unwrap_or_default();
        vec![
            self.id.to_string(),
            self.name.clone(),
            self.difficulty.clone().unwrap_or_default(),
            self.category_name.clone().unwrap_or_default(),
            self.solves.to_string(),
            self.rating.map(|r| format!("{r:.1}")).unwrap_or_default(),
            progress,
            self.state.clone().unwrap_or_default(),
        ]
    }
}

impl Tabular for SherlockCategory {
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
    fn deserialize_sherlocks_list() {
        let json = include_str!("../../tests/fixtures/sherlocks-list.json");
        let result: Paginated<Sherlock> = serde_json::from_str(json).unwrap();
        assert!(!result.data.is_empty());
        assert_eq!(result.data[0].name, "Brutus");
        assert_eq!(result.data[0].category_name.as_deref(), Some("DFIR"));
    }

    #[test]
    fn deserialize_sherlock_categories() {
        let json = include_str!("../../tests/fixtures/sherlocks-categories.json");
        let result: SherlockCategoriesResponse = serde_json::from_str(json).unwrap();
        assert!(!result.info.is_empty());
        assert!(result.info.iter().any(|c| c.name == "DFIR"));
    }
}
