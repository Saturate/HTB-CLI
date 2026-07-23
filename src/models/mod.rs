use serde::{Deserialize, Deserializer, Serialize};

pub mod challenge;
pub mod ctf;
pub mod machine;
pub mod season;
pub mod sherlock;
pub mod user;
pub mod vpn;

/// Deserialize a value that may be a string or an integer into `Option<String>`.
pub(crate) fn deserialize_string_or_int<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let v = Option::<serde_json::Value>::deserialize(deserializer)?;
    Ok(v.map(|v| match v {
        serde_json::Value::String(s) => s,
        serde_json::Value::Number(n) => n.to_string(),
        other => other.to_string(),
    }))
}

/// Deserialize a bool that may arrive as `null` from the API.
pub(crate) fn deserialize_bool_or_null<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Option::<bool>::deserialize(deserializer)?.unwrap_or(false))
}

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

    #[test]
    fn bool_or_null_handles_null() {
        #[derive(Deserialize)]
        struct T {
            #[serde(default, deserialize_with = "deserialize_bool_or_null")]
            flag: bool,
        }
        let null: T = serde_json::from_str(r#"{"flag": null}"#).unwrap();
        assert!(!null.flag);
        let t: T = serde_json::from_str(r#"{"flag": true}"#).unwrap();
        assert!(t.flag);
        let missing: T = serde_json::from_str(r#"{}"#).unwrap();
        assert!(!missing.flag);

        // Rejects types the API shouldn't send but might
        assert!(serde_json::from_str::<T>(r#"{"flag": 0}"#).is_err());
        assert!(serde_json::from_str::<T>(r#"{"flag": 1}"#).is_err());
        assert!(serde_json::from_str::<T>(r#"{"flag": "true"}"#).is_err());
    }

    #[test]
    fn string_or_int_handles_both() {
        #[derive(Deserialize)]
        struct T {
            #[serde(default, deserialize_with = "deserialize_string_or_int")]
            val: Option<String>,
        }
        let s: T = serde_json::from_str(r#"{"val": "50"}"#).unwrap();
        assert_eq!(s.val.as_deref(), Some("50"));
        let i: T = serde_json::from_str(r#"{"val": 0}"#).unwrap();
        assert_eq!(i.val.as_deref(), Some("0"));
        let n: T = serde_json::from_str(r#"{"val": null}"#).unwrap();
        assert!(n.val.is_none());
    }

    #[test]
    fn string_or_int_handles_bool() {
        #[derive(Deserialize)]
        struct T {
            #[serde(default, deserialize_with = "deserialize_string_or_int")]
            val: Option<String>,
        }
        let t: T = serde_json::from_str(r#"{"val": true}"#).unwrap();
        assert_eq!(t.val.as_deref(), Some("true"));
        let f: T = serde_json::from_str(r#"{"val": false}"#).unwrap();
        assert_eq!(f.val.as_deref(), Some("false"));
    }

    #[test]
    fn string_or_int_handles_float() {
        #[derive(Deserialize)]
        struct T {
            #[serde(default, deserialize_with = "deserialize_string_or_int")]
            val: Option<String>,
        }
        let f: T = serde_json::from_str(r#"{"val": 4.5}"#).unwrap();
        assert_eq!(f.val.as_deref(), Some("4.5"));
    }

    #[test]
    fn string_or_int_handles_empty_string() {
        #[derive(Deserialize)]
        struct T {
            #[serde(default, deserialize_with = "deserialize_string_or_int")]
            val: Option<String>,
        }
        let e: T = serde_json::from_str(r#"{"val": ""}"#).unwrap();
        assert_eq!(e.val.as_deref(), Some(""));
    }
}
