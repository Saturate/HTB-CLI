use clap::Subcommand;

#[derive(Subcommand)]
pub enum UserCommand {
    /// Show your profile
    Me,
    /// Show another user's profile
    Info {
        /// Username or user ID
        user: String,
    },
}
