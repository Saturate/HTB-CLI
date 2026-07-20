use clap::Subcommand;

use crate::api::HtbClient;
use crate::output::{self, OutputFormat};

#[derive(Subcommand)]
pub enum VpnCommand {
    /// Show current VPN connection status
    Status,
    /// List available VPN servers
    List,
    /// Switch VPN server
    Switch {
        /// Server ID
        server_id: u32,
    },
    /// Download .ovpn file
    Download {
        /// Server ID (uses default if omitted)
        server_id: Option<u32>,
    },
}

pub async fn handle(
    client: &HtbClient,
    cmd: VpnCommand,
    format: OutputFormat,
) -> anyhow::Result<()> {
    match cmd {
        VpnCommand::Status => {
            let status = client.vpn().status().await?;
            if status.is_empty() {
                output::print_message("Not connected to VPN.");
            } else {
                output::json::print_json(&status);
            }
        }

        VpnCommand::List => {
            let connections = client.vpn().connections().await?;
            output::print_list(&connections, format);
        }

        VpnCommand::Switch { server_id } => {
            let resp = client.vpn().switch(server_id).await?;
            if let Some(msg) = &resp.message {
                output::print_message(msg);
            }
            if let Some(server) = &resp.data {
                output::print_message(&format!(
                    "Switched to {} ({} clients)",
                    server.friendly_name, server.current_clients
                ));
            }
        }

        VpnCommand::Download { server_id } => {
            let sid = server_id.unwrap_or(1);
            let bytes = client.vpn().download_ovpn(sid).await?;
            let filename = format!("lab-vpn-{sid}.ovpn");
            std::fs::write(&filename, &bytes)?;
            output::print_message(&format!("Downloaded {} ({} bytes)", filename, bytes.len()));
        }
    }
    Ok(())
}
