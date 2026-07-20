use clap::Subcommand;

use crate::api::HtbClient;
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

pub async fn handle(cmd: AuthCommand, format: OutputFormat) -> anyhow::Result<()> {
    match cmd {
        AuthCommand::Login => login().await,
        AuthCommand::Status => status(format).await,
        AuthCommand::Logout => logout(),
    }
}

async fn login() -> anyhow::Result<()> {
    println!("Enter your HTB API token (from https://app.hackthebox.com/profile/settings):");
    let token = rpassword::read_password()?;
    let token = token.trim().to_string();

    if token.is_empty() {
        anyhow::bail!("Token cannot be empty");
    }

    let client = HtbClient::new(token.clone());
    let user = client.user().current().await?;

    crate::config::save_token(&token)?;
    println!("Authenticated as {} ({})", user.name, if user.is_vip { "VIP" } else { "Free" });
    Ok(())
}

async fn status(format: OutputFormat) -> anyhow::Result<()> {
    let token = crate::config::read_token()?;
    let client = HtbClient::new(token);
    let user = client.user().current().await?;

    let fields = vec![
        ("Username", user.name.clone()),
        ("ID", user.id.to_string()),
        ("Subscription", user.subscription_type.clone().unwrap_or_else(|| "Free".into())),
        ("VIP", if user.is_vip { "Yes" } else { "No" }.into()),
        ("Verified", if user.verified { "Yes" } else { "No" }.into()),
    ];

    crate::output::print_detail(&user, format, &fields);
    Ok(())
}

fn logout() -> anyhow::Result<()> {
    crate::config::remove_token()?;
    println!("Token removed.");
    Ok(())
}
