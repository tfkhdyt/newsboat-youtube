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

        /// Print all process verbosely
        #[arg(short = 'v', long)]
        verbose: bool,

        /// API key to fetch channel id from YouTube Data API v3. You can also set it via
        /// environment variable "NBYT_API_KEY"
        #[arg(short = 'k', long)]
        api_key: Option<String>,
    },
}
