use gtk4::{Orientation};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ButtonsConfig {
    pub height: i32,
    pub width: i32
}

impl Default for ButtonsConfig {
    fn default() -> Self {
        Self {
            width: 100,
            height: 100,    
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub title: String,

    // Margin
    pub margin_top: i32,
    pub margin_bottom: i32,
    pub margin_left: i32,
    pub margin_right: i32,

    pub orientation: String,
}

impl Default for AppConfig {

    fn default() -> Self {
        const MARGINS: i32 = 15;
        Self {
            title: "Power menu".to_string(),
            orientation: "horizontal".to_string(), // "vertical"
            margin_top:  MARGINS,
            margin_bottom: MARGINS,
            margin_left: MARGINS,
            margin_right: MARGINS,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub application: AppConfig,
    pub buttons: ButtonsConfig,

    #[serde(skip)]
    filename: String,
}

impl Config {
    pub fn new(filename: &str) -> Self {
        let mut conf = Config::default();
        conf.filename = String::from(filename);
        conf
    }

    pub fn load(&mut self) {
        if let Ok(config_data) = std::fs::read_to_string(&self.filename) {
            if let Ok(config) = serde_json::from_str::<Config>(&config_data) {
                self.application = config.application;
            }
        }
    }

    pub fn save(&self) {
        if let Ok(config_data) = serde_json::to_string_pretty(&self) {
            let _ = std::fs::write(&self.filename, config_data);
        }
    }

    pub fn get_orientation(&self) -> Option<Orientation> {
        match self.application.orientation.as_str() {
            "vertical" => Some(Orientation::Vertical),
            "horizontal" => Some(Orientation::Horizontal),
            _ => Option::None,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            filename: "./config.json".to_string(),
            application: AppConfig::default(),
            buttons: ButtonsConfig::default(),
        }   
    }
}