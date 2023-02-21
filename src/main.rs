mod args;
mod config;
mod subcommands;

use std::process;

use args::{Cli, Commands};
use clap::Parser;
use config::Config;
use subcommands::add;

fn main() {
    dotenv::dotenv().ok();
    let config = Config::new();
    let api_key = config.get_api_key();

    let cli = Cli::parse();
    let filename = "yt_url.txt";

    match &cli.command {
        Some(Commands::Add {
            urls,
            no_confirmation,
        }) => urls.iter().for_each(|url| {
            let result = match add::execute(url, &api_key, filename, *no_confirmation) {
                Ok(v) => v,
                Err(err) => {
                    eprintln!("Error: {}", err);
                    process::exit(1);
                }
            };

            println!("{}\n", result);
        }),
        None => {}
    }
}
