use crate::error::HtbError;
use crate::models::user::{
    ActivityEntry, UserActivityResponse, UserInfo, UserInfoResponse, UserProfile,
    UserProfileResponse,
};

use super::HtbClient;

pub struct UserApi<'a>(pub(crate) &'a HtbClient);

impl UserApi<'_> {
    pub async fn current(&self) -> Result<UserInfo, HtbError> {
        let resp: UserInfoResponse = self.0.get("/api/v4/user/info").await?;
        Ok(resp.info)
    }

    pub async fn profile(&self, user_id: u64) -> Result<UserProfile, HtbError> {
        let resp: UserProfileResponse = self
            .0
            .get(&format!("/api/v4/user/profile/basic/{user_id}"))
            .await?;
        Ok(resp.profile)
    }

    pub async fn activity(&self, user_id: u64) -> Result<Vec<ActivityEntry>, HtbError> {
        let resp: UserActivityResponse = self
            .0
            .get(&format!("/api/v5/user/profile/activity/{user_id}"))
            .await?;
        Ok(resp.data)
    }
}
