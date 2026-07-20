use clap::Subcommand;

use crate::api::HtbClient;
use crate::output::OutputFormat;

#[derive(Subcommand)]
pub enum MachineCommand {
    /// List machines
    List {
        #[arg(long, help = "Include retired machines")]
        retired: bool,
        #[arg(long, help = "Filter by OS (linux, windows)")]
        os: Option<String>,
        #[arg(long, help = "Filter by difficulty (easy, medium, hard, insane)")]
        difficulty: Option<String>,
        #[arg(long, help = "Page number")]
        page: Option<u32>,
        #[arg(long, help = "Fetch all pages")]
        all: bool,
    },
    /// Show machine details
    Info {
        /// Machine name or ID
        name_or_id: String,
    },
    /// Spawn a machine
    Start {
        /// Machine name or ID
        name_or_id: String,
    },
    /// Stop the active machine
    Stop,
    /// Reset a machine
    Reset {
        /// Machine name or ID
        name_or_id: String,
    },
    /// Submit a flag
    Submit {
        /// Machine name or ID
        name_or_id: String,
        /// The flag
        flag: String,
    },
    /// Show currently active machine
    Active,
    /// Manage todo list
    Todo {
        #[command(subcommand)]
        command: Option<TodoCommand>,
    },
}

#[derive(Subcommand)]
pub enum TodoCommand {
    /// Add a machine to todo list
    Add {
        /// Machine name or ID
        name_or_id: String,
    },
    /// Remove a machine from todo list
    Remove {
        /// Machine name or ID
        name_or_id: String,
    },
}

pub async fn handle(
    _client: &HtbClient,
    cmd: MachineCommand,
    _format: OutputFormat,
) -> anyhow::Result<()> {
    let _ = (cmd, _format, _client);
    anyhow::bail!("Machines commands not yet implemented")
}
