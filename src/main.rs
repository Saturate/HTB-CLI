pub mod api;
pub mod cli;
pub mod config;
pub mod error;
pub mod models;
pub mod output;

use clap::Parser;
use tracing_subscriber::EnvFilter;

use cli::{Cli, Command};
use output::OutputFormat;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let filter = if cli.verbose {
        EnvFilter::new("debug")
    } else {
        EnvFilter::from_default_env()
    };

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .init();

    let format = if cli.json {
        OutputFormat::Json
    } else {
        OutputFormat::Table
    };

    if let Err(e) = run(cli.command, format).await {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

async fn run(command: Command, format: OutputFormat) -> anyhow::Result<()> {
    match command {
        Command::Auth { command } => cli::auth::handle(command, format).await,

        Command::Machines { command } => {
            let client = authenticated_client()?;
            cli::machines::handle(&client, command, format).await
        }

        Command::Challenges { command: _ } => {
            anyhow::bail!("Challenges commands not yet implemented")
        }

        Command::Seasons { command: _ } => {
            anyhow::bail!("Seasons commands not yet implemented")
        }

        Command::Sherlocks { command: _ } => {
            anyhow::bail!("Sherlocks commands not yet implemented")
        }

        Command::Vpn { command: _ } => {
            anyhow::bail!("VPN commands not yet implemented")
        }

        Command::User { command: _ } => {
            anyhow::bail!("User commands not yet implemented")
        }

        Command::Search { query: _ } => {
            anyhow::bail!("Search not yet implemented")
        }
    }
}

fn authenticated_client() -> anyhow::Result<api::HtbClient> {
    let token = config::read_token()?;
    Ok(api::HtbClient::new(token))
}
