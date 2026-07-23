use clap::Subcommand;

use crate::api::HtbClient;
use crate::output::{self, OutputFormat};

#[derive(Subcommand)]
#[command(
    after_help = "Examples:\n  htb challenges list --category Web     Web challenges\n  htb challenges categories              All categories\n  htb challenges info Poly               Challenge details\n  htb challenges start Poly              Start container instance\n  htb challenges submit 112 'HTB{f}'     Submit a flag\n  htb challenges download Poly           Download challenge files"
)]
pub enum ChallengeCommand {
    /// List challenges
    List {
        #[arg(long, help = "Filter by category")]
        category: Option<String>,
        #[arg(long, help = "Filter by state (active, retired_free)")]
        state: Option<String>,
        #[arg(long, help = "Page number")]
        page: Option<u32>,
    },
    /// List challenge categories
    Categories,
    /// Show challenge details
    Info {
        /// Challenge slug
        slug: String,
    },
    /// Download challenge files
    Download {
        /// Challenge slug
        slug: String,
    },
    /// Start a challenge instance
    Start {
        /// Challenge slug
        slug: String,
    },
    /// Stop a challenge instance
    Stop {
        /// Challenge slug
        slug: String,
    },
    /// Submit a flag
    Submit {
        /// Challenge ID
        id: u64,
        /// The flag
        flag: String,
    },
}

pub async fn handle(
    client: &HtbClient,
    cmd: ChallengeCommand,
    format: OutputFormat,
) -> anyhow::Result<()> {
    match cmd {
        ChallengeCommand::List {
            category,
            state,
            page,
        } => {
            let result = client.challenges().list(page.unwrap_or(1), 100).await?;
            let mut challenges = result.data;

            if let Some(ref cat_filter) = category {
                challenges.retain(|c| {
                    c.category_name
                        .as_ref()
                        .is_some_and(|cn| cn.eq_ignore_ascii_case(cat_filter))
                });
            }

            if let Some(ref state_filter) = state {
                challenges.retain(|c| {
                    c.state
                        .as_ref()
                        .is_some_and(|s| s.eq_ignore_ascii_case(state_filter))
                });
            }

            output::print_list(&challenges, format);
            output::print_pagination(
                result.meta.current_page,
                result.meta.last_page,
                result.meta.total,
                format,
            );
        }

        ChallengeCommand::Categories => {
            let categories = client.challenges().categories().await?;
            output::print_list(&categories, format);
        }

        ChallengeCommand::Info { slug } => {
            let detail = client.challenges().info(&slug).await?;
            let fields = vec![
                ("ID", detail.id.to_string()),
                ("Name", detail.name.clone()),
                ("Difficulty", detail.difficulty.clone().unwrap_or_default()),
                ("Category", detail.category_name.clone().unwrap_or_default()),
                (
                    "Points",
                    detail.points.clone().unwrap_or_else(|| "0".into()),
                ),
                (
                    "XP",
                    detail
                        .experience_points
                        .map(|xp| xp.to_string())
                        .unwrap_or_else(|| "0".into()),
                ),
                (
                    "Rating",
                    detail.stars.map(|s| format!("{s:.1}")).unwrap_or_default(),
                ),
                ("Solves", detail.solves.to_string()),
                ("State", detail.state.clone().unwrap_or_default()),
                (
                    "First Blood",
                    detail
                        .first_blood_user
                        .clone()
                        .unwrap_or_else(|| "-".into()),
                ),
                (
                    "Blood Time",
                    detail
                        .first_blood_time
                        .clone()
                        .unwrap_or_else(|| "-".into()),
                ),
                ("Creator", detail.creator_name.clone().unwrap_or_default()),
                (
                    "Description",
                    detail.description.clone().unwrap_or_default(),
                ),
                ("Play Methods", detail.play_methods.join(", ")),
                (
                    "Solved",
                    if detail.auth_user_solve { "Yes" } else { "No" }.into(),
                ),
            ];
            output::print_detail(&detail, format, &fields);
        }

        ChallengeCommand::Download { slug } => {
            let detail = client.challenges().info(&slug).await?;
            let url = client.challenges().download_link(detail.id).await?;
            let bytes = client.challenges().download_file(&url).await?;
            let raw_name = detail.file_name.unwrap_or_else(|| format!("{slug}.zip"));
            let filename = crate::sanitize_filename(&raw_name, &format!("{slug}.zip"));
            std::fs::write(&filename, &bytes)?;
            output::print_message(&format!("Downloaded {} ({} bytes)", filename, bytes.len()));
        }

        ChallengeCommand::Start { slug } => {
            let detail = client.challenges().info(&slug).await?;
            let resp = client.challenges().start(detail.id).await?;
            output::print_message(&resp.message);
        }

        ChallengeCommand::Stop { slug } => {
            let detail = client.challenges().info(&slug).await?;
            let resp = client.challenges().stop(detail.id).await?;
            output::print_message(&resp.message);
        }

        ChallengeCommand::Submit { id, flag } => {
            let resp = client.challenges().submit_flag(id, &flag).await?;
            output::print_message(&resp.message);
        }
    }
    Ok(())
}
