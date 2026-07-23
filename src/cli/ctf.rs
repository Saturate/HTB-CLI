use std::sync::Arc;

use clap::Subcommand;

use crate::api::HtbClient;
use crate::cache::Cache;
use crate::output::OutputFormat;

const CTF_BASE_URL: &str = "https://ctf.hackthebox.com";

#[derive(Subcommand)]
pub enum CtfCommand {
    /// Manage CTF authentication
    Auth {
        #[command(subcommand)]
        command: CtfAuthCommand,
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
    }
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
