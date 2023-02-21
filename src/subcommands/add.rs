use std::{
    fs::File,
    io::{self, BufReader, Write},
    path::Path,
};

use newsboat_youtube::{self, check_duplicate};

pub fn execute(
    url: &str,
    api_key: &String,
    filename: &str,
    no_confirmation: bool,
) -> Result<String, String> {
    let handle = newsboat_youtube::parse_handle(url)?;
    let (channel_id, channel_name) = match newsboat_youtube::fetch_yt_api(&handle, api_key) {
        Ok(v) => v,
        Err(err) => return Err(err.to_string()),
    };

    let feed = format!("https://www.youtube.com/feeds/videos.xml?channel_id={channel_id} \"youtube\" \"{channel_name}\"\n");

    println!("Handle        : @{handle}");
    println!("Channel ID    : {channel_id}");
    println!("Channel Name  : {channel_name}");

    if !Path::new(filename).exists() {
        File::create(filename).unwrap();
    }

    let mut is_confirmed = false;

    if !no_confirmation {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let is_duplicate = check_duplicate(reader, channel_id);
        let mut input = String::new();

        if is_duplicate {
            println!("{channel_name} is already added!");
            print!("Do you want to add this feed again? (Y/n): ");
        } else {
            print!("Do you want to add this feed? (Y/n): ");
        }

        io::stdout().flush().expect("Could not flush stdout");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        is_confirmed = input.to_lowercase().trim() == "y" || input.trim() == "";
    }

    if is_confirmed || no_confirmation {
        match newsboat_youtube::append_to_file(filename, feed.as_str()) {
            Ok(_) => Ok(format!(
                "{channel_name} feed has been successfully added to newsboat urls"
            )),
            Err(err) => Err(err.to_string()),
        }
    } else {
        Ok("".to_string())
    }
}
