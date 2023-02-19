use std::env;

pub struct Config {
    api_key: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            api_key: env::var("API_KEY").expect("API_KEY must be set"),
        }
    }

    pub fn get_api_key(&self) -> String {
        self.api_key.clone()
    }
}
