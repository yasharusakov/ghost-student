use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub enum Platform {
    #[serde(rename = "google-meet")]
    GoogleMeet,
    #[serde(rename = "zoom")]
    Zoom,
}

#[derive(Deserialize, Debug)]
pub struct TgBot {
    pub token: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Meeting {
    pub platform: Platform,
    pub name: String,
    pub day: u32,
    pub start_time: String,
    pub end_time: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub meetings: Vec<Meeting>,
    pub tg_bot: Option<TgBot>,
}

impl Config {
    pub fn load(path: &str) -> Result<Config, String> {
        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => return Err(format!("Failed to read config file: {}", e)),
        };

        let config: Config = match serde_json::from_str(&content) {
            Ok(cfg) => cfg,
            Err(e) => return Err(format!("Failed to parse config file: {}", e)),
        };

        Ok(config)
    }
}
