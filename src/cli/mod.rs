pub mod auth;
pub mod challenges;
pub mod machines;
pub mod search;
pub mod seasons;
pub mod sherlocks;
pub mod user;
pub mod vpn;

use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "htb", version, about = "Hack The Box CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    #[arg(long, global = true, help = "Output as JSON")]
    pub json: bool,

    #[arg(long, global = true, help = "Disable colored output")]
    pub no_color: bool,

    #[arg(short, long, global = true, help = "Enable debug logging")]
    pub verbose: bool,

    #[arg(long, global = true, help = "Override config file path")]
    pub config: Option<PathBuf>,
}

#[derive(Subcommand)]
pub enum Command {
    /// Manage authentication
    Auth {
        #[command(subcommand)]
        command: auth::AuthCommand,
    },
    /// Browse and interact with machines
    Machines {
        #[command(subcommand)]
        command: machines::MachineCommand,
    },
    /// Browse and interact with challenges
    Challenges {
        #[command(subcommand)]
        command: challenges::ChallengeCommand,
    },
    /// View seasons and rankings
    Seasons {
        #[command(subcommand)]
        command: seasons::SeasonCommand,
    },
    /// Browse and interact with Sherlocks
    Sherlocks {
        #[command(subcommand)]
        command: sherlocks::SherlockCommand,
    },
    /// Manage VPN connections
    Vpn {
        #[command(subcommand)]
        command: vpn::VpnCommand,
    },
    /// View user profiles
    User {
        #[command(subcommand)]
        command: user::UserCommand,
    },
    /// Search across all content
    Search {
        /// Search query
        query: String,
    },
}
