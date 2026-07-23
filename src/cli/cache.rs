use clap::Subcommand;

use crate::cache::Cache;
use crate::output;

#[derive(Subcommand)]
pub enum CacheCommand {
    /// Remove all cached responses
    Clear,
}

pub fn handle(cmd: CacheCommand, cache: &Cache) {
    match cmd {
        CacheCommand::Clear => {
            cache.clear();
            output::print_message("Cache cleared.");
        }
    }
}
