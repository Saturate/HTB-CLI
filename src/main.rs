pub mod api;
pub mod cache;
pub mod cli;
pub mod config;
pub mod error;
pub mod mcp;
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

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Check for --mcp-stdio before clap parsing (it's a mode switch, not a subcommand)
    if std::env::args().any(|a| a == "--mcp-stdio") {
        if let Err(e) = mcp::run_stdio().await {
            eprintln!("MCP error: {e}");
            std::process::exit(1);
        }
        return;
    }

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

    let cfg = match config::HtbConfig::load(cli.config.as_deref()) {
        Ok(c) => c,
        Err(e) => {
            if cli.config.is_some() {
                eprintln!("Error: failed to load config: {e}");
                std::process::exit(1);
            }
            tracing::debug!("Using default config: {e}");
            config::HtbConfig::default()
        }
    };

    let format = if cli.json {
        OutputFormat::Json
    } else {
        match cfg.output.as_str() {
            "json" => OutputFormat::Json,
            _ => OutputFormat::Table,
        }
    };

    if cli.no_color || cfg.no_color {
        // SAFETY: called before any concurrent work; single-threaded tokio runtime
        unsafe { std::env::set_var("NO_COLOR", "1") };
    }

    let cache_enabled = cfg.cache.enabled && !cli.no_cache;
    let cache_dir = config::config_dir()
        .map(|d| d.join("cache"))
        .unwrap_or_else(|_| std::env::temp_dir().join("htb-cli-cache"));
    let app_cache = cache::Cache::new(cache_dir, cache_enabled);

    if let Err(e) = run(cli.command, format, &app_cache).await {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

async fn run(command: Command, format: OutputFormat, cache: &cache::Cache) -> anyhow::Result<()> {
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

        Command::Cache { command } => {
            cli::cache::handle(command, cache);
            Ok(())
        }
    }
}

fn authenticated_client() -> anyhow::Result<api::HtbClient> {
    let token = config::read_token()?;
    Ok(api::HtbClient::new(token))
}
