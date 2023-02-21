use std::{fs::File, io::BufReader, path::Path};

use newsboat_youtube::{self, check_duplicate};

pub fn execute(
    url: &str,
    api_key: &String,
    filename: &str,
    verbose: bool,
) -> Result<String, String> {
    let handle = newsboat_youtube::parse_handle(url)?;
    let (channel_id, channel_name) = match newsboat_youtube::fetch_yt_api(&handle, api_key) {
        Ok(v) => v,
        Err(err) => return Err(err.to_string()),
    };

    let feed = format!("https://www.youtube.com/feeds/videos.xml?channel_id={channel_id} \"youtube\" \"{channel_name}\"\n");

    if !Path::new(filename).exists() {
        File::create(filename).unwrap();
    }

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let is_duplicate = check_duplicate(reader, &channel_id);

    if is_duplicate {
        if verbose {
            return Ok(format!(
                "{channel_name} ({channel_id}) is already in the feed list. Skipping..."
            ));
        }
        return Ok("".to_string());
    }

    if verbose {
        return match newsboat_youtube::append_to_file(filename, feed.as_str()) {
            Ok(_) => {
                if verbose {
                    Ok(format!(
                        "{channel_name} ({channel_id}) feed has been successfully added to newsboat urls"
                    ))
                } else {
                    Ok("".to_string())
                }
            }
            Err(err) => Err(err.to_string()),
        };
    }
    Ok("".to_string())
}
