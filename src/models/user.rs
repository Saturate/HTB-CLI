use serde::{Deserialize, Serialize};

use super::deserialize_bool_or_null;

#[derive(Debug, Deserialize)]
pub struct UserInfoResponse {
    pub info: UserInfo,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub timezone: Option<String>,
    #[serde(default, deserialize_with = "deserialize_bool_or_null")]
    pub is_vip: bool,
    #[serde(default, deserialize_with = "deserialize_bool_or_null")]
    pub is_moderator: bool,
    #[serde(default, deserialize_with = "deserialize_bool_or_null")]
    pub can_access_vip: bool,
    #[serde(default)]
    pub server_id: Option<u32>,
    #[serde(default)]
    pub rank_id: u32,
    #[serde(default, deserialize_with = "deserialize_bool_or_null")]
    pub verified: bool,
    #[serde(default)]
    pub subscription_type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UserProfileResponse {
    pub profile: UserProfile,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserProfile {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub system_owns: u32,
    #[serde(default)]
    pub user_owns: u32,
    #[serde(default)]
    pub user_bloods: u32,
    #[serde(default)]
    pub system_bloods: u32,
    #[serde(default)]
    pub rank: Option<String>,
    #[serde(default)]
    pub rank_id: u32,
    #[serde(default)]
    pub ranking: Option<u32>,
    #[serde(default)]
    pub points: u32,
    #[serde(default)]
    pub country_name: Option<String>,
    #[serde(default)]
    pub country_code: Option<String>,
    #[serde(default)]
    pub joined_date: Option<String>,
    #[serde(default)]
    pub server: Option<String>,
    #[serde(
        default,
        rename = "isVip",
        deserialize_with = "deserialize_bool_or_null"
    )]
    pub is_vip: bool,
    #[serde(
        default,
        rename = "isDedicatedVip",
        deserialize_with = "deserialize_bool_or_null"
    )]
    pub is_dedicated_vip: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_user_info() {
        let json = include_str!("../../tests/fixtures/user-info.json");
        let resp: UserInfoResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.info.name, "TestUser");
        assert!(!resp.info.is_vip);
        assert_eq!(resp.info.id, 1234567);
    }

    #[test]
    fn deserialize_user_profile() {
        let json = include_str!("../../tests/fixtures/user-profile-basic.json");
        let resp: UserProfileResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.profile.name, "TestUser");
        assert_eq!(resp.profile.system_owns, 10);
        assert_eq!(resp.profile.user_owns, 10);
        assert_eq!(resp.profile.rank.as_deref(), Some("Script Kiddie"));
        assert_eq!(resp.profile.country_name.as_deref(), Some("Denmark"));
    }
}
