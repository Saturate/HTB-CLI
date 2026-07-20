use crate::error::HtbError;

use super::HtbClient;

pub struct SearchApi<'a>(pub(crate) &'a HtbClient);

impl SearchApi<'_> {
    pub async fn fetch(&self, query: &str) -> Result<serde_json::Value, HtbError> {
        let encoded = super::encode_path(query);
        self.0
            .get(&format!("/api/v4/search/fetch?query={encoded}"))
            .await
    }
}
