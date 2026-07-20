use clap::Subcommand;

#[derive(Subcommand)]
pub enum SeasonCommand {
    /// List all seasons
    List,
    /// Show machines in a season
    Machines {
        /// Season ID
        season_id: u32,
    },
    /// Show season leaderboard
    Leaderboard {
        /// Season ID
        season_id: u32,
    },
    /// Show your rank in the current season
    Rank,
}
