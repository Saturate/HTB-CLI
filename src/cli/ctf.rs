use clap::Subcommand;

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
