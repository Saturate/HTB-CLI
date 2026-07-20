use clap::Subcommand;

use crate::api::HtbClient;
use crate::output::{self, OutputFormat};

#[derive(Subcommand)]
pub enum UserCommand {
    /// Show your profile
    Me,
    /// Show another user's profile
    Info {
        /// Username or user ID
        user: String,
    },
}

pub async fn handle(
    client: &HtbClient,
    cmd: UserCommand,
    format: OutputFormat,
) -> anyhow::Result<()> {
    match cmd {
        UserCommand::Me => {
            let current = client.user().current().await?;
            let profile = client.user().profile(current.id).await?;
            let fields = vec![
                ("Username", profile.name.clone()),
                ("ID", profile.id.to_string()),
                ("Rank", profile.rank.clone().unwrap_or_default()),
                ("Points", profile.points.to_string()),
                (
                    "Ranking",
                    profile
                        .ranking
                        .map(|r| format!("#{r}"))
                        .unwrap_or_else(|| "-".into()),
                ),
                ("User Owns", profile.user_owns.to_string()),
                ("System Owns", profile.system_owns.to_string()),
                ("User Bloods", profile.user_bloods.to_string()),
                ("System Bloods", profile.system_bloods.to_string()),
                ("Country", profile.country_name.clone().unwrap_or_default()),
                ("Server", profile.server.clone().unwrap_or_default()),
            ];
            output::print_detail(&profile, format, &fields);
        }

        UserCommand::Info { user } => {
            let user_id: u64 = match user.parse() {
                Ok(id) => id,
                Err(_) => {
                    anyhow::bail!("Please provide a numeric user ID. Username lookup is not supported by the API.");
                }
            };
            let profile = client.user().profile(user_id).await?;
            let fields = vec![
                ("Username", profile.name.clone()),
                ("ID", profile.id.to_string()),
                ("Rank", profile.rank.clone().unwrap_or_default()),
                ("Points", profile.points.to_string()),
                (
                    "Ranking",
                    profile
                        .ranking
                        .map(|r| format!("#{r}"))
                        .unwrap_or_else(|| "-".into()),
                ),
                ("User Owns", profile.user_owns.to_string()),
                ("System Owns", profile.system_owns.to_string()),
                ("Country", profile.country_name.clone().unwrap_or_default()),
            ];
            output::print_detail(&profile, format, &fields);
        }
    }
    Ok(())
}
