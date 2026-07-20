use clap::Subcommand;

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
    /// List tasks for a Sherlock
    Tasks {
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
