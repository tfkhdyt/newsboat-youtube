pub fn parse_handle(url: String) -> Result<String, String> {
    if !url.contains('@') {
        return Err("Invalid url, should contains @".to_string());
    }

    let token: Vec<&str> = url.split('@').collect();
    let handle: String = match token[1].parse() {
        Ok(result) => result,
        Err(_) => return Err("Failed to parse token to string".to_string()),
    };

    return Ok(remove_symbols(handle.as_str()));
}

fn remove_symbols(s: &str) -> String {
    s.chars().filter(|c| c.is_alphanumeric()).collect()
}
