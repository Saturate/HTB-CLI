use serde::{Deserialize, Serialize};

pub mod challenge;
pub mod machine;
pub mod season;
pub mod sherlock;
pub mod user;
pub mod vpn;

#[derive(Debug, Deserialize)]
pub struct Paginated<T> {
    pub data: Vec<T>,
    pub links: PaginationLinks,
    pub meta: PaginationMeta,
}

#[derive(Debug, Deserialize)]
pub struct PaginationLinks {
    pub first: Option<String>,
    pub last: Option<String>,
    pub prev: Option<String>,
    pub next: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PaginationMeta {
    pub current_page: u32,
    pub from: Option<u32>,
    pub last_page: u32,
    pub per_page: u32,
    pub to: Option<u32>,
    pub total: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ActionResponse {
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paginated_machines() {
        let json = include_str!("../../tests/fixtures/v5-machines-list.json");
        let result: Paginated<serde_json::Value> = serde_json::from_str(json).unwrap();
        assert_eq!(result.data.len(), 15);
        assert_eq!(result.meta.total, 545);
        assert_eq!(result.meta.last_page, 37);
        assert_eq!(result.meta.current_page, 1);
    }

    #[test]
    fn paginated_challenges() {
        let json = include_str!("../../tests/fixtures/challenges-list.json");
        let result: Paginated<serde_json::Value> = serde_json::from_str(json).unwrap();
        assert_eq!(result.data.len(), 15);
        assert_eq!(result.meta.total, 839);
    }

    #[test]
    fn paginated_sherlocks() {
        let json = include_str!("../../tests/fixtures/sherlocks-list.json");
        let result: Paginated<serde_json::Value> = serde_json::from_str(json).unwrap();
        assert!(!result.data.is_empty());
    }

    #[test]
    fn action_response_container_start() {
        let json = include_str!("../../tests/fixtures/container-start.json");
        let result: ActionResponse = serde_json::from_str(json).unwrap();
        assert_eq!(result.message, "Instance Created!");
    }

    #[test]
    fn action_response_flag_incorrect() {
        let json = include_str!("../../tests/fixtures/challenge-own-incorrect.json");
        let result: ActionResponse = serde_json::from_str(json).unwrap();
        assert_eq!(result.message, "Incorrect flag");
    }
}
