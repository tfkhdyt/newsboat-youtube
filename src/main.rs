mod args;

use args::{Cli, Commands};
use clap::Parser;
use newsboat_youtube::{append_to_file, fetch_yt_api, parse_handle};
use std::process;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Add { urls }) => {
            for url in urls {
                let handle = parse_handle(url.to_string()).unwrap_or_else(|err| {
                    eprintln!("Error: {err}");
                    process::exit(1);
                });
                let (channel_id, channel_name) = fetch_yt_api(&handle).unwrap_or_else(|err| {
                    eprintln!("Error: {err}");
                    process::exit(1);
                });

                let feed = format!("https://www.youtube.com/feeds/videos.xml?channel_id={channel_id} 'youtube' '{channel_name}'\n");

                println!("Handle: {handle}");
                println!("Channel ID: {channel_id}");
                println!("Channel Name: {channel_name}");
                // println!("Feed: {feed}");

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
        None => {}
    }
}
