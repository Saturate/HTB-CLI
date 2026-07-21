pub mod api;
pub mod cache;
pub mod cli;
pub mod config;
pub mod error;
pub mod mcp;
pub mod models;
pub mod output;

use std::path::Path;

pub fn sanitize_filename(raw: &str, fallback: &str) -> String {
    Path::new(raw)
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| fallback.to_string())
}
