use serde_json::Value;

#[tokio::main]
pub async fn fetch_yt_api(
    yt_handle: &String,
) -> Result<(String, String), Box<dyn std::error::Error>> {
    let api_key = "AIzaSyC_6P7NiL2zXoO9VQKgoANBcpZqhRTjFl4";
    let resp = reqwest::get(format!("https://youtube.googleapis.com/youtube/v3/search?part=id,snippet&q={yt_handle}&type=channel&key={api_key}"))
        .await?.json::<Value>().await?;

    let channel_id = &resp["items"][0]["id"]["channelId"].as_str().unwrap();
    let channel_name = &resp["items"][0]["snippet"]["title"].as_str().unwrap();

    Ok((channel_id.to_string(), channel_name.to_string()))
}
