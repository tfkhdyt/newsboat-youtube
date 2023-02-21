mod args;
mod config;
mod subcommands;

use std::process;

use args::{Cli, Commands};
use clap::Parser;
use config::Config;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use subcommands::add;

fn main() {
    dotenv::dotenv().ok();
    let config = Config::new();

    let cli = Cli::parse();
    let filename = "yt_url.txt";

    match &cli.command {
        Some(Commands::Add {
            urls,
            verbose,
            api_key,
        }) => urls.par_iter().for_each(|url| {
            let api_key_to_use = match api_key {
                Some(api_key) => api_key.clone(),
                None => config.get_api_key(),
            };
            let result = match add::execute(url, &api_key_to_use, filename, *verbose) {
                Ok(v) => v,
                Err(err) => {
                    eprintln!("Error: {}", err);
                    process::exit(1);
                }
            };
            println!("{}", result);
        }),

        None => {}
    }
}
