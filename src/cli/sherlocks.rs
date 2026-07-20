use clap::Subcommand;

use crate::api::HtbClient;
use crate::output::{self, OutputFormat};

#[derive(Subcommand)]
pub enum SherlockCommand {
    /// List Sherlocks
    List {
        #[arg(long, help = "Filter by category")]
        category: Option<String>,
        #[arg(long, help = "Page number")]
        page: Option<u32>,
    },
    /// List Sherlock categories
    Categories,
    /// Show Sherlock details
    Info {
        /// Sherlock slug
        slug: String,
    },
    /// Download Sherlock files
    Download {
        /// Sherlock slug
        slug: String,
    },
    /// Submit a task flag
    Submit {
        /// Sherlock ID
        id: u64,
        /// Task ID
        task_id: u64,
        /// The flag
        flag: String,
    },
}

pub async fn handle(
    client: &HtbClient,
    cmd: SherlockCommand,
    format: OutputFormat,
) -> anyhow::Result<()> {
    match cmd {
        SherlockCommand::List { category, page } => {
            let result = client.sherlocks().list(page.unwrap_or(1), 100).await?;
            let mut sherlocks = result.data;

            if let Some(ref cat_filter) = category {
                sherlocks.retain(|s| {
                    s.category_name
                        .as_ref()
                        .is_some_and(|cn| cn.eq_ignore_ascii_case(cat_filter))
                });
            }

            output::print_list(&sherlocks, format);
            output::print_pagination(
                result.meta.current_page,
                result.meta.last_page,
                result.meta.total,
            );
        }

        SherlockCommand::Categories => {
            let categories = client.sherlocks().categories().await?;
            output::print_list(&categories, format);
        }

        SherlockCommand::Info { slug } => {
            let sherlock = client.sherlocks().info(&slug).await?;
            let fields = vec![
                ("ID", sherlock.id.to_string()),
                ("Name", sherlock.name.clone()),
                (
                    "Difficulty",
                    sherlock.difficulty.clone().unwrap_or_default(),
                ),
                (
                    "Category",
                    sherlock.category_name.clone().unwrap_or_default(),
                ),
                ("Solves", sherlock.solves.to_string()),
                (
                    "Rating",
                    sherlock
                        .rating
                        .map(|r| format!("{r:.1}"))
                        .unwrap_or_default(),
                ),
                (
                    "Progress",
                    sherlock
                        .progress
                        .map(|p| format!("{p}%"))
                        .unwrap_or_else(|| "-".into()),
                ),
                ("State", sherlock.state.clone().unwrap_or_default()),
            ];
            output::print_detail(&sherlock, format, &fields);
        }

        SherlockCommand::Download { slug } => {
            let sherlock = client.sherlocks().info(&slug).await?;
            let url = client.sherlocks().download_link(sherlock.id).await?;
            let bytes = client.challenges().download_file(&url).await?;
            let filename = format!("{slug}.zip");
            std::fs::write(&filename, &bytes)?;
            output::print_message(&format!("Downloaded {} ({} bytes)", filename, bytes.len()));
        }

        SherlockCommand::Submit { id, task_id, flag } => {
            let resp = client.sherlocks().submit_flag(id, task_id, &flag).await?;
            output::print_message(&resp.message);
        }
    }
    Ok(())
}
