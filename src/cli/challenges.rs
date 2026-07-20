use clap::Subcommand;

#[derive(Subcommand)]
pub enum ChallengeCommand {
    /// List challenges
    List {
        #[arg(long, help = "Filter by category")]
        category: Option<String>,
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
