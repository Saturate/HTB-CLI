use std::sync::Arc;

use clap::Subcommand;

use crate::api::HtbClient;
use crate::cache::Cache;
use crate::output::OutputFormat;

#[derive(Subcommand)]
pub enum AuthCommand {
    /// Save your HTB API token
    Login,
    /// Show current authentication status
    Status,
    /// Remove stored token
    Logout,
}

pub async fn handle(
    cmd: AuthCommand,
    format: OutputFormat,
    cache: &Arc<Cache>,
) -> anyhow::Result<()> {
    match cmd {
        AuthCommand::Login => login(cache).await,
        AuthCommand::Status => status(format).await,
        AuthCommand::Logout => logout(cache),
    }
}

async fn login(cache: &Cache) -> anyhow::Result<()> {
    println!("Enter your HTB API token (from https://app.hackthebox.com/profile/settings):");
    let token = rpassword::read_password()?;
    let token = token.trim().to_string();

    if token.is_empty() {
        anyhow::bail!("Token cannot be empty");
    }

    let client = HtbClient::new(token.clone());
    let user = client.user().current().await?;

    crate::config::save_token(&token)?;
    cache.clear();
    println!(
        "Authenticated as {} ({})",
        user.name,
        if user.is_vip { "VIP" } else { "Free" }
    );
    Ok(())
}

async fn status(format: OutputFormat) -> anyhow::Result<()> {
    let token = crate::config::read_token()?;
    let client = HtbClient::new(token);
    let user = client.user().current().await?;

    let fields = vec![
        ("Username", user.name.clone()),
        ("ID", user.id.to_string()),
        (
            "Subscription",
            user.subscription_type
                .clone()
                .unwrap_or_else(|| "Free".into()),
        ),
        ("VIP", if user.is_vip { "Yes" } else { "No" }.into()),
        ("Verified", if user.verified { "Yes" } else { "No" }.into()),
    ];

    crate::output::print_detail(&user, format, &fields);
    Ok(())
}

fn logout(cache: &Cache) -> anyhow::Result<()> {
    crate::config::remove_token()?;
    cache.clear();
    println!("Token removed.");
    Ok(())
}
