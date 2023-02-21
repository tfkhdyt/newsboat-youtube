use serde_json::Value;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

pub fn parse_handle(url: &str) -> Result<String, String> {
    if !url.contains('@') {
        return Err("Invalid url, should contains @".to_string());
    }

    let token: Vec<&str> = url.split('@').collect();
    let handle: String = match token[1].parse() {
        Ok(result) => result,
        Err(_) => return Err("Failed to parse token to string".to_string()),
    };

    let cleaned_handle = remove_symbols(handle.as_str());

    if cleaned_handle.len() < 3 {
        return Err("Handle is invalid".to_string());
    }

    Ok(cleaned_handle)
}

fn remove_symbols(s: &str) -> String {
    s.chars().filter(|c| c.is_alphanumeric()).collect()
}

#[tokio::main]
pub async fn fetch_yt_api(
    yt_handle: &String,
    api_key: &String,
) -> Result<(String, String), Box<dyn std::error::Error>> {
    let resp = reqwest::get(format!("https://youtube.googleapis.com/youtube/v3/search?part=id,snippet&q={yt_handle}&type=channel&key={api_key}"))
        .await?.json::<Value>().await?;

    let channel_id = &resp["items"][0]["id"]["channelId"]
        .as_str()
        .ok_or("Failed to get channel id")?;
    let channel_name = &resp["items"][0]["snippet"]["title"]
        .as_str()
        .ok_or("Failed to get channel name")?;

    Ok((channel_id.to_string(), channel_name.to_string()))
}

pub fn append_to_file(file_name: &str, text: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_name)?;
    file.write_all(text.as_bytes())?;
    Ok(())
}

pub fn check_duplicate(reader: BufReader<File>, channel_id: &String) -> bool {
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        if line.contains(channel_id) {
            return true;
        }
    }
    false
}
