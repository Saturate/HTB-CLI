use clap::Subcommand;

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
