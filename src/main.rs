use clap::{CommandFactory, Parser};
use tracing_subscriber::EnvFilter;

use htb_cli::cache;
use htb_cli::cli::{self, Cli, Command};
use htb_cli::config;
use htb_cli::mcp;
use htb_cli::output::OutputFormat;

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
    let app_cache = std::sync::Arc::new(cache::Cache::new(config::cache_dir(), cache_enabled));

    if let Err(e) = run(cli.command, format, app_cache).await {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

async fn run(
    command: Command,
    format: OutputFormat,
    app_cache: std::sync::Arc<cache::Cache>,
) -> anyhow::Result<()> {
    match command {
        Command::Auth { command } => cli::auth::handle(command, format, &app_cache).await,

        Command::Machines { command } => {
            let client = authenticated_client(app_cache)?;
            cli::machines::handle(&client, command, format).await
        }

        Command::Challenges { command } => {
            let client = authenticated_client(app_cache)?;
            cli::challenges::handle(&client, command, format).await
        }

        Command::Seasons { command } => {
            let client = authenticated_client(app_cache)?;
            cli::seasons::handle(&client, command, format).await
        }

        Command::Sherlocks { command } => {
            let client = authenticated_client(app_cache)?;
            cli::sherlocks::handle(&client, command, format).await
        }

        Command::Vpn { command } => {
            let client = authenticated_client(app_cache)?;
            cli::vpn::handle(&client, command, format).await
        }

        Command::User { command } => {
            let client = authenticated_client(app_cache)?;
            cli::user::handle(&client, command, format).await
        }

        Command::Pwnbox { command } => {
            let client = authenticated_client(app_cache)?;
            cli::pwnbox::handle(&client, command, format).await
        }

        Command::Ctf { command } => cli::ctf::handle(command, format, &app_cache).await,

        Command::Search { query } => {
            let client = authenticated_client(app_cache)?;
            cli::search::handle(&client, &query).await
        }

        Command::Cache { command } => {
            cli::cache::handle(command, &app_cache);
            Ok(())
        }

        Command::Completions { shell } => {
            clap_complete::generate(shell, &mut Cli::command(), "htb", &mut std::io::stdout());
            Ok(())
        }
    }
}

fn authenticated_client(
    cache: std::sync::Arc<cache::Cache>,
) -> anyhow::Result<htb_cli::api::HtbClient> {
    let token = config::read_token()?;
    Ok(htb_cli::api::HtbClient::with_cache_arc(token, cache))
}
