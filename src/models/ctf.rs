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

#[cfg(test)]
mod tests {
    use super::*;

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
