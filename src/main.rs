mod args;

use args::{Cli, Commands};
use clap::Parser;
use newsboat_youtube::{append_to_file, fetch_yt_api, parse_handle};
use std::{
    io::{self, Write},
    process,
};

fn main() {
    dotenv::dotenv().ok();

    let api_key = std::env::var("API_KEY").expect("API_KEY must be set");
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Add { urls }) => {
            for url in urls {
                let handle = parse_handle(url.to_string()).unwrap_or_else(|err| {
                    eprintln!("Error: {err}");
                    process::exit(1);
                });
                let (channel_id, channel_name) =
                    fetch_yt_api(&handle, &api_key).unwrap_or_else(|err| {
                        eprintln!("Error: {err}");
                        process::exit(1);
                    });

                let feed = format!("https://www.youtube.com/feeds/videos.xml?channel_id={channel_id} 'youtube' '{channel_name}'\n");

                println!("Handle        : @{handle}");
                println!("Channel ID    : {channel_id}");
                println!("Channel Name  : {channel_name}\n");

                let mut is_confirmed = String::new();

                print!("Do you want to add this feed? (Y/n): ");
                io::stdout().flush().ok().expect("Could not flush stdout");
                io::stdin()
                    .read_line(&mut is_confirmed)
                    .expect("Failed to read line");

                if is_confirmed.to_lowercase().trim() == "y" || is_confirmed.trim() == "" {
                    match append_to_file("yt_url.txt", feed.as_str()) {
                        Ok(_) => {
                            println!(
                                "{channel_name} feed has been successfully added to newsboat urls"
                            );
                        }
                        Err(err) => {
                            eprintln!("Error: {err}");
                            process::exit(1);
                        }
                    }
                }
            }
        }
        None => {}
    }
}
