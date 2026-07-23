use serde::{Deserialize, Serialize};

use super::deserialize_bool_or_null;
use crate::output::Tabular;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Machine {
    pub id: u64,
    pub name: String,
    pub os: String,
    #[serde(default)]
    pub points: u32,
    #[serde(default)]
    pub rating: Option<f64>,
    #[serde(default)]
    pub difficulty: Option<u32>,
    #[serde(default)]
    pub difficulty_text: Option<String>,
    #[serde(default)]
    pub user_owns_count: u32,
    #[serde(default)]
    pub root_owns_count: u32,
    #[serde(default)]
    pub auth_user_in_user_owns: Option<bool>,
    #[serde(default)]
    pub auth_user_in_root_owns: Option<bool>,
    #[serde(default, deserialize_with = "deserialize_bool_or_null")]
    pub todo: bool,
    #[serde(default)]
    pub state: Option<String>,
    #[serde(default)]
    pub ip: Option<String>,
    #[serde(default, deserialize_with = "deserialize_bool_or_null")]
    pub free: bool,
    #[serde(default)]
    pub release_date: Option<String>,
    #[serde(default)]
    pub play_info: Option<MachinePlayInfo>,
    #[serde(default)]
    pub first_creator: Option<Creator>,
    #[serde(default)]
    pub labels: Vec<Label>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MachinePlayInfo {
    #[serde(default)]
    pub is_spawned: Option<bool>,
    #[serde(default)]
    pub is_spawning: Option<bool>,
    #[serde(default, deserialize_with = "deserialize_bool_or_null")]
    pub is_active: bool,
    #[serde(default)]
    pub active_player_count: Option<u32>,
    #[serde(default)]
    pub expires_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Creator {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub avatar: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Label {
    pub color: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct ActiveVmResponse {
    pub info: Option<ActiveVmInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveVmInfo {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub avatar: Option<String>,
    #[serde(default)]
    pub expires_at: Option<String>,
    #[serde(default, rename = "type")]
    pub vm_type: Option<String>,
    #[serde(default)]
    pub lab_server: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MachineProfileResponse {
    pub info: Machine,
}

#[derive(Debug, Deserialize)]
pub struct TodoListResponse {
    pub info: Vec<Machine>,
}

impl Tabular for Machine {
    fn headers() -> Vec<&'static str> {
        vec![
            "ID",
            "Name",
            "OS",
            "Difficulty",
            "Rating",
            "Points",
            "State",
            "User",
            "Root",
        ]
    }

    fn row(&self) -> Vec<String> {
        let user_own = match self.auth_user_in_user_owns {
            Some(true) => "✓",
            _ => "",
        };
        let root_own = match self.auth_user_in_root_owns {
            Some(true) => "✓",
            _ => "",
        };
        vec![
            self.id.to_string(),
            self.name.clone(),
            self.os.clone(),
            self.difficulty_text.clone().unwrap_or_default(),
            self.rating.map(|r| format!("{r:.1}")).unwrap_or_default(),
            self.points.to_string(),
            self.state.clone().unwrap_or_default(),
            user_own.to_string(),
            root_own.to_string(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Paginated;

    #[test]
    fn deserialize_machines_list() {
        let json = include_str!("../../tests/fixtures/v5-machines-list.json");
        let result: Paginated<Machine> = serde_json::from_str(json).unwrap();
        assert_eq!(result.data.len(), 15);
        let cap = &result.data[0];
        assert_eq!(cap.name, "Cap");
        assert_eq!(cap.os, "Linux");
        assert_eq!(cap.difficulty_text.as_deref(), Some("Easy"));
        assert_eq!(cap.auth_user_in_user_owns, Some(true));
    }

    #[test]
    fn deserialize_machines_list_live() {
        let json = include_str!("../../tests/fixtures/v5-machines-list-live.json");
        let result: Paginated<Machine> = serde_json::from_str(json).unwrap();
        assert_eq!(result.data.len(), 3);
        assert_eq!(result.data[0].name, "TestMachine-1");
        assert_eq!(result.data[0].os, "Linux");
        assert_eq!(result.data[0].difficulty, Some(28));
        assert!(result.data[0].free);
        assert_eq!(result.data[0].state.as_deref(), Some("retired_free"));
        assert!(result.data[0].labels.iter().any(|l| l.name == "STAFF PICK"));
        assert!(result.data[0].first_creator.is_some());
    }

    #[test]
    fn deserialize_active_vm_null() {
        let json = include_str!("../../tests/fixtures/v5-virtual-machine-active.json");
        let result: ActiveVmResponse = serde_json::from_str(json).unwrap();
        assert!(result.info.is_none());
    }
}
