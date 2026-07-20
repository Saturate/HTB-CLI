use serde_json::json;

use crate::error::HtbError;
use crate::models::vpn::{Connection, ConnectionsResponse, VpnSwitchResponse};

use super::HtbClient;

pub struct VpnApi<'a>(pub(crate) &'a HtbClient);

impl VpnApi<'_> {
    pub async fn connections(&self) -> Result<Vec<Connection>, HtbError> {
        let resp: ConnectionsResponse = self.0.get("/api/v5/connections").await?;
        Ok(resp.data)
    }

    pub async fn status(&self) -> Result<Vec<serde_json::Value>, HtbError> {
        self.0.get("/api/v4/connection/status").await
    }

    pub async fn switch(&self, server_id: u32) -> Result<VpnSwitchResponse, HtbError> {
        self.0
            .post(
                &format!("/api/v4/connections/servers/switch/{server_id}"),
                &json!({}),
            )
            .await
    }

    pub async fn download_ovpn(&self, server_id: u32) -> Result<Vec<u8>, HtbError> {
        self.0
            .get_bytes(&format!("/api/v4/access/ovpnfile/{server_id}/0"))
            .await
    }
}
