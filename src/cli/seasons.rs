use clap::Subcommand;

use crate::api::HtbClient;
use crate::output::{self, OutputFormat};

#[derive(Subcommand)]
pub enum SeasonCommand {
    /// List all seasons
    List,
    /// Show your rank in the current season
    Rank,
}

pub async fn handle(
    client: &HtbClient,
    cmd: SeasonCommand,
    format: OutputFormat,
) -> anyhow::Result<()> {
    match cmd {
        SeasonCommand::List => {
            let seasons = client.seasons().list().await?;
            output::print_list(&seasons, format);
        }

        SeasonCommand::Rank => {
            let user = client.user().current().await?;
            let ranks = client.seasons().user_ranks(user.id).await?;

            if ranks.is_empty() {
                output::print_message("No season data found.");
            } else {
                output::print_list(&ranks, format);
            }
        }
    }
    Ok(())
}
