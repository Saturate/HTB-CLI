use serde::{Deserialize, Serialize};

use crate::output::Tabular;

#[derive(Debug, Deserialize)]
pub struct ConnectionsResponse {
    pub data: Vec<Connection>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Connection {
    #[serde(rename = "type")]
    pub connection_type: String,
    #[serde(default)]
    pub location_type_friendly: Option<String>,
    #[serde(default)]
    pub assigned_server: Option<AssignedServer>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AssignedServer {
    pub id: u32,
    pub friendly_name: String,
    #[serde(default)]
    pub current_clients: u32,
    #[serde(default)]
    pub location: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VpnSwitchResponse {
    #[serde(default)]
    pub status: bool,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub data: Option<AssignedServer>,
}

impl Tabular for Connection {
    fn headers() -> Vec<&'static str> {
        vec!["Type", "Location", "Server", "Clients"]
    }

    fn row(&self) -> Vec<String> {
        let (server, clients) = match &self.assigned_server {
            Some(s) => (s.friendly_name.clone(), s.current_clients.to_string()),
            None => ("-".into(), "-".into()),
        };
        vec![
            self.connection_type.clone(),
            self.location_type_friendly.clone().unwrap_or_default(),
            server,
            clients,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_connections() {
        let json = include_str!("../../tests/fixtures/v5-connections.json");
        let result: ConnectionsResponse = serde_json::from_str(json).unwrap();
        assert!(!result.data.is_empty());
        let labs = result.data.iter().find(|c| c.connection_type == "labs");
        assert!(labs.is_some());
        assert!(labs.unwrap().assigned_server.is_some());
    }

    #[test]
    fn deserialize_connection_status_empty() {
        let json = include_str!("../../tests/fixtures/connection-status.json");
        let result: Vec<serde_json::Value> = serde_json::from_str(json).unwrap();
        assert!(result.is_empty());
    }
}
