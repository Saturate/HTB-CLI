use clap::Subcommand;

use crate::api::HtbClient;
use crate::output::{self, OutputFormat};

#[derive(Subcommand)]
pub enum PwnboxCommand {
    /// Show PwnBox usage quota
    Usage,
    /// Show active PwnBox status
    Status,
}

pub async fn handle(
    client: &HtbClient,
    cmd: PwnboxCommand,
    format: OutputFormat,
) -> anyhow::Result<()> {
    match cmd {
        PwnboxCommand::Usage => {
            let usage: PwnboxUsage = client.get("/api/v4/pwnbox/usage").await?;
            let fields = vec![
                ("Remaining", format!("{} min", usage.remaining)),
                ("Used", format!("{} min", usage.used)),
                ("Allowed", format!("{} min", usage.allowed)),
                ("Sessions", usage.sessions.to_string()),
            ];
            output::print_detail(&usage, format, &fields);
        }
        PwnboxCommand::Status => {
            let result: Result<PwnboxStatus, _> = client.get("/api/v4/pwnbox/status").await;
            match result {
                Ok(status) if status.hostname.is_some() => {
                    let fields = vec![
                        ("Hostname", status.hostname.clone().unwrap_or_default()),
                        ("IP", status.ip.clone().unwrap_or_else(|| "-".into())),
                        ("Region", status.region.clone().unwrap_or_default()),
                        ("Lab", status.lab.clone().unwrap_or_default()),
                        ("Expires", status.expires_at.clone().unwrap_or_default()),
                    ];
                    output::print_detail(&status, format, &fields);
                }
                _ => {
                    output::print_message("No active PwnBox instance.");
                }
            }
        }
    }
    Ok(())
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct PwnboxUsage {
    #[serde(default)]
    remaining: u32,
    #[serde(default)]
    used: u32,
    #[serde(default)]
    allowed: u32,
    #[serde(default)]
    sessions: u32,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct PwnboxStatus {
    #[serde(default)]
    hostname: Option<String>,
    #[serde(default)]
    ip: Option<String>,
    #[serde(default)]
    region: Option<String>,
    #[serde(default)]
    lab: Option<String>,
    #[serde(default)]
    expires_at: Option<String>,
}
