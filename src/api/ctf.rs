use crate::error::HtbError;
use crate::models::ctf::{CtfEvent, CtfEventData, CtfEventDetail, CtfMenu, CtfUserProfile};

use super::HtbClient;

pub struct CtfApi<'a>(pub(crate) &'a HtbClient);

impl CtfApi<'_> {
    pub async fn profile(&self) -> Result<CtfUserProfile, HtbError> {
        self.0.get("/api/users/profile").await
    }

    pub async fn events(&self) -> Result<Vec<CtfEvent>, HtbError> {
        self.0.get("/api/ctfs").await
    }

    pub async fn event_details(&self, slug: &str) -> Result<CtfEventDetail, HtbError> {
        let encoded = super::encode_path(slug);
        self.0
            .get(&format!("/api/ctfs/details/{encoded}"))
            .await
    }

    pub async fn event_data(&self, event_id: u64) -> Result<CtfEventData, HtbError> {
        self.0.get(&format!("/api/ctfs/{event_id}")).await
    }

    pub async fn menu(&self, event_id: u64) -> Result<CtfMenu, HtbError> {
        self.0.get(&format!("/api/ctfs/{event_id}/menu")).await
    }
}
