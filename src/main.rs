pub mod api;
pub mod cli;
pub mod config;
pub mod error;
pub mod models;
pub mod output;

use std::path::Path;

use clap::Parser;
use tracing_subscriber::EnvFilter;

pub fn sanitize_filename(raw: &str, fallback: &str) -> String {
    Path::new(raw)
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| fallback.to_string())
}

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
        .with_ansi(!cli.no_color)
        .init();

    let cfg = config::HtbConfig::load(cli.config.as_deref()).unwrap_or_default();

    let format = if cli.json {
        OutputFormat::Json
    } else {
        match cfg.output.as_str() {
            "json" => OutputFormat::Json,
            _ => OutputFormat::Table,
        }
    };

    if cli.no_color || cfg.no_color {
        std::env::set_var("NO_COLOR", "1");
    }

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

        Command::Challenges { command } => {
            let client = authenticated_client()?;
            cli::challenges::handle(&client, command, format).await
        }

        Command::Seasons { command } => {
            let client = authenticated_client()?;
            cli::seasons::handle(&client, command, format).await
        }

        Command::Sherlocks { command } => {
            let client = authenticated_client()?;
            cli::sherlocks::handle(&client, command, format).await
        }

        Command::Vpn { command } => {
            let client = authenticated_client()?;
            cli::vpn::handle(&client, command, format).await
        }

        Command::User { command } => {
            let client = authenticated_client()?;
            cli::user::handle(&client, command, format).await
        }

        Command::Search { query } => {
            let client = authenticated_client()?;
            cli::search::handle(&client, &query).await
        }
    }
}

fn authenticated_client() -> anyhow::Result<api::HtbClient> {
    let token = config::read_token()?;
    Ok(api::HtbClient::new(token))
}
