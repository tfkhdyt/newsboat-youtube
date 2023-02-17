mod args;
mod subcommands;

use args::{Cli, Commands};
use clap::Parser;
use newsboat_youtube::fetch_yt_api;
use std::process;
use subcommands::add;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Add { urls }) => {
            for url in urls {
                let handle = add::parse_handle(url.to_string()).unwrap_or_else(|err| {
                    eprintln!("Error: {err}");
                    process::exit(1);
                });
                match fetch_yt_api(&handle) {
                    Ok(resp) => {
                        println!("Channel id: {}", resp.0);
                        println!("Channel name: {}", resp.1);
                    }
                    Err(err) => {
                        eprintln!("Error: {err}");
                    }
                }
                println!("Handle: {handle}");
            }
        }
        None => {}
    }
}
