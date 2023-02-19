use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
/// Manage your Newsboat YouTube RSS Feed link easily
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Convert YouTube channel link to valid RSS link and append it to your newsboat urls file
    Add {
        /// YouTube Channel URLs (Example: https://youtube.com/@tfkhdyt)
        urls: Vec<String>,
    },
}
