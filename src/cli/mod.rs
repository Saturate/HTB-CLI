pub mod auth;
pub mod cache;
pub mod challenges;
pub mod ctf;
pub mod machines;
pub mod pwnbox;
pub mod search;
pub mod seasons;
pub mod sherlocks;
pub mod user;
pub mod vpn;

use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "htb",
    version,
    about = "Hack The Box CLI",
    long_about = "Hack The Box CLI - interact with the HTB platform from your terminal.\n\nQuery machines, challenges, Sherlocks, and seasons. Spawn instances, submit flags, manage VPN connections.",
    after_help = "Examples:\n  htb auth login                              Save your API token\n  htb machines list --os linux --difficulty easy   Filter machines\n  htb machines start Bedside                  Spawn a machine\n  htb challenges list --category Web          Browse web challenges\n  htb challenges submit 1018 'HTB{flag}'      Submit a challenge flag\n  htb vpn list                                Show VPN servers\n  htb user me                                 View your profile\n  htb search nmap                             Search across all content"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,

    #[arg(long, help = "Run as MCP server over stdin/stdout")]
    pub mcp_stdio: bool,

    #[arg(long, global = true, help = "Output as JSON")]
    pub json: bool,

    #[arg(long, global = true, help = "Disable colored output")]
    pub no_color: bool,

    #[arg(short, long, global = true, help = "Enable debug logging")]
    pub verbose: bool,

    #[arg(long, global = true, help = "Override config file path")]
    pub config: Option<PathBuf>,

    #[arg(long, global = true, help = "Bypass response cache")]
    pub no_cache: bool,
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
    /// Check PwnBox status and usage
    Pwnbox {
        #[command(subcommand)]
        command: pwnbox::PwnboxCommand,
    },
    /// Interact with CTF events and challenges
    Ctf {
        #[command(subcommand)]
        command: ctf::CtfCommand,
    },
    /// Search across all content
    Search {
        /// Search query
        query: String,
    },
    /// Manage response cache
    Cache {
        #[command(subcommand)]
        command: cache::CacheCommand,
    },
}
