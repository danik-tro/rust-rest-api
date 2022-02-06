use serde::Deserialize;
use std::fs;


#[derive(Deserialize)]
struct AppConfig {
    url: String,
    port: u16
}


#[derive(Deserialize)]
pub struct Config {
    app: AppConfig
}


impl Config {
    pub fn from_file(path: &'static str) -> Self {
        let config = fs::read_to_string(path).unwrap();
        serde_json::from_str(&config).unwrap()
    }

    pub fn get_app_url(&self) -> String {
        format!("{0}:{1}", self.app.url, self.app.port)
    }
}
