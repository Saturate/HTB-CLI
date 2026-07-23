use crate::error::HtbError;
use crate::models::ctf::CtfUserProfile;

use super::HtbClient;

pub struct CtfApi<'a>(pub(crate) &'a HtbClient);

impl CtfApi<'_> {
    pub async fn profile(&self) -> Result<CtfUserProfile, HtbError> {
        self.0.get("/api/users/profile").await
    }
}
