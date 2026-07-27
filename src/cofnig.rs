use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Config {
    pub orientation: String,
    pub title: String,

    #[serde(skip)]
    filename: String,
}

impl Config {
    pub fn new(filename: &str) -> Self {
        let mut conf = Config::default();
        conf.filename = String::from(filename);
        conf
    }

    pub fn load(&self) -> Self {
        if let Ok(config_data) = std::fs::read_to_string(&self.filename) {
            if let Ok(config) = serde_json::from_str::<Config>(&config_data) {
                return config;
            }
        }

        Self::default()
    }

    pub fn save(&self) {
        if let Ok(config_data) = serde_json::to_string_pretty(&self) {
            let _ = std::fs::write(&self.filename, config_data);
        }
    }

    pub fn default() -> Self {
        Self {
            orientation: String::from("vertical"),
            title: String::from("Power Menu"),
            filename: String::from("./config.json")
        }
    }
}