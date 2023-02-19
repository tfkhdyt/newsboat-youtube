use std::{
    fs::File,
    io::{self, BufRead, BufReader, Write},
    path::Path,
};

use newsboat_youtube;

pub fn execute(url: &String, api_key: &String, filename: &str) -> Result<String, String> {
    let handle = newsboat_youtube::parse_handle(url)?;
    let (channel_id, channel_name) = match newsboat_youtube::fetch_yt_api(&handle, api_key) {
        Ok(v) => v,
        Err(err) => return Err(err.to_string()),
    };

    let feed = format!("https://www.youtube.com/feeds/videos.xml?channel_id={channel_id} \"youtube\" \"{channel_name}\"\n");

    println!("Handle        : @{handle}");
    println!("Channel ID    : {channel_id}");
    println!("Channel Name  : {channel_name}\n");

    if !Path::new(filename).exists() {
        File::create(filename).unwrap();
    }

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut is_duplicate = false;

    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        if line.contains(&channel_id) {
            println!("{channel_name} is already added!");
            is_duplicate = true;
            break;
        }
    }

    let mut input = String::new();
    if is_duplicate {
        print!("Do you want to add this feed again? (Y/n): ");
    } else {
        print!("Do you want to add this feed? (Y/n): ");
    }
    io::stdout().flush().ok().expect("Could not flush stdout");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let is_confirmed = input.to_lowercase().trim() == "y" || input.trim() == "";

    if is_confirmed {
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
