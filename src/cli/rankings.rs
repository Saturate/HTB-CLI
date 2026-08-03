use clap::Subcommand;

use crate::api::HtbClient;
use crate::output::{self, OutputFormat};

#[derive(Subcommand)]
pub enum RankingsCommand {
    /// Show global user rankings
    Users,
}

pub async fn handle(
    client: &HtbClient,
    cmd: RankingsCommand,
    format: OutputFormat,
) -> anyhow::Result<()> {
    match cmd {
        RankingsCommand::Users => {
            let entries = client.rankings().users().await?;
            if entries.is_empty() {
                output::print_message("No rankings data available.");
            } else {
                output::print_list(&entries, format);
            }
        }
    }
    Ok(())
}
