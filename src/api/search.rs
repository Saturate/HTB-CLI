use crate::error::HtbError;

use super::HtbClient;

pub struct SearchApi<'a>(pub(crate) &'a HtbClient);

impl SearchApi<'_> {
    pub async fn fetch(&self, query: &str) -> Result<serde_json::Value, HtbError> {
        let encoded: String = query
            .bytes()
            .flat_map(|b| {
                if b.is_ascii_alphanumeric() || b == b'-' || b == b'_' || b == b'.' || b == b'~' {
                    vec![b as char]
                } else {
                    format!("%{b:02X}").chars().collect()
                }
            })
            .collect();
        self.0
            .get(&format!("/api/v4/search/fetch?query={encoded}"))
            .await
    }
}
