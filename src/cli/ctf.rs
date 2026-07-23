use std::sync::Arc;

use clap::Subcommand;

use crate::api::HtbClient;
use crate::cache::Cache;
use crate::output::OutputFormat;

const CTF_BASE_URL: &str = "https://ctf.hackthebox.com";

#[derive(Subcommand)]
#[command(
    after_help = "Examples:\n  htb ctf auth login                     Save CTF API token\n  htb ctf events                         List CTF events\n  htb ctf info ctf-try-out-1434          Event details\n  htb ctf challenges 1434                Challenges in event\n  htb ctf submit 31855 'HTB{flag}'       Submit a flag\n  htb ctf scoreboard 1434                Event scoreboard"
)]
pub enum CtfCommand {
    /// Manage CTF authentication
    Auth {
        #[command(subcommand)]
        command: CtfAuthCommand,
    },
    /// List CTF events
    Events {
        #[arg(long, help = "Include past events")]
        all: bool,
    },
    /// Show CTF event details
    Info {
        /// Event slug (e.g. ctf-try-out-1434)
        slug: String,
    },
}

#[derive(Subcommand)]
pub enum CtfAuthCommand {
    /// Save your CTF API token
    Login,
    /// Show CTF auth status
    Status,
    /// Remove stored CTF token
    Logout,
}

pub async fn handle(
    cmd: CtfCommand,
    format: OutputFormat,
    cache: &Arc<Cache>,
) -> anyhow::Result<()> {
    match cmd {
        CtfCommand::Auth { command } => handle_auth(command, format, cache).await,
        CtfCommand::Events { all } => events(all, format, cache).await,
        CtfCommand::Info { slug } => info(&slug, format, cache).await,
    }
}

fn ctf_client(cache: &Arc<Cache>) -> anyhow::Result<HtbClient> {
    let token = crate::config::read_ctf_token()?;
    Ok(HtbClient::with_base_url_and_cache(
        token,
        CTF_BASE_URL.to_string(),
        Arc::clone(cache),
    ))
}

async fn handle_auth(
    cmd: CtfAuthCommand,
    format: OutputFormat,
    cache: &Arc<Cache>,
) -> anyhow::Result<()> {
    match cmd {
        CtfAuthCommand::Login => login(cache).await,
        CtfAuthCommand::Status => status(format).await,
        CtfAuthCommand::Logout => logout(cache),
    }
}

async fn login(cache: &Cache) -> anyhow::Result<()> {
    println!("Enter your CTF API token (from https://ctf.hackthebox.com/settings):");
    let token = rpassword::read_password()?;
    let token = token.trim().to_string();

    if token.is_empty() {
        anyhow::bail!("Token cannot be empty");
    }

    let client = HtbClient::with_base_url(token.clone(), CTF_BASE_URL.to_string());
    let user = client.ctf().profile().await?;

    crate::config::save_ctf_token(&token)?;
    cache.clear();
    println!("Authenticated as {}", user.name);
    Ok(())
}

async fn status(format: OutputFormat) -> anyhow::Result<()> {
    let token = crate::config::read_ctf_token()?;
    let client = HtbClient::with_base_url(token, CTF_BASE_URL.to_string());
    let user = client.ctf().profile().await?;

    let fields = vec![
        ("Username", user.name.clone()),
        ("ID", user.id.to_string()),
        ("Email", user.email.clone().unwrap_or_default()),
        ("Timezone", user.timezone.clone().unwrap_or_default()),
        (
            "Has Team",
            if user.has_any_team { "Yes" } else { "No" }.into(),
        ),
    ];

    crate::output::print_detail(&user, format, &fields);
    Ok(())
}

fn logout(cache: &Cache) -> anyhow::Result<()> {
    crate::config::remove_ctf_token()?;
    cache.clear();
    println!("CTF token removed.");
    Ok(())
}

async fn events(all: bool, format: OutputFormat, cache: &Arc<Cache>) -> anyhow::Result<()> {
    let client = ctf_client(cache)?;
    let mut events = client.ctf().events().await?;

    if !all {
        events.retain(|e| {
            e.status
                .as_deref()
                .is_some_and(|s| s == "Ongoing" || s == "Upcoming")
        });
    }

    crate::output::print_list(&events, format);
    Ok(())
}

async fn info(slug: &str, format: OutputFormat, cache: &Arc<Cache>) -> anyhow::Result<()> {
    let client = ctf_client(cache)?;
    let detail = client.ctf().event_details(slug).await?;

    let fields = vec![
        ("ID", detail.id.to_string()),
        ("Name", detail.name.clone()),
        ("Status", detail.status.clone().unwrap_or_default()),
        ("Format", detail.format.clone().unwrap_or_default()),
        ("Type", detail.event_type.clone().unwrap_or_default()),
        ("Location", detail.location.clone().unwrap_or_default()),
        (
            "Start",
            detail.start_date.clone().unwrap_or_default(),
        ),
        ("End", detail.end_date.clone().unwrap_or_default()),
        (
            "Players",
            detail
                .players_joined
                .map(|p| p.to_string())
                .unwrap_or_default(),
        ),
        (
            "Teams",
            detail
                .teams_joined
                .map(|t| t.to_string())
                .unwrap_or_default(),
        ),
        (
            "Challenges",
            detail
                .challenges
                .map(|c| c.to_string())
                .unwrap_or_default(),
        ),
        (
            "Max Team Size",
            detail
                .max_team_size
                .map(|m| m.to_string())
                .unwrap_or_default(),
        ),
        (
            "Prize Pool",
            detail.prize_pool.clone().unwrap_or_else(|| "-".into()),
        ),
    ];

    crate::output::print_detail(&detail, format, &fields);
    Ok(())
}
