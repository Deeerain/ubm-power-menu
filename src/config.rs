use gtk4::Orientation;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Config {
    pub title: String,
    
    // Margin
    pub margin_top: i32,
    pub margin_bottom: i32,
    pub margin_left: i32,
    pub margin_right: i32,

    pub orientation: String,

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
                self.orientation = config.orientation;
                self.title = config.filename;

                self.margin_top = config.margin_top;
                self.margin_bottom = config.margin_bottom;
                self.margin_left = config.margin_left;
                self.margin_right = config.margin_right;
            }
        }
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
            filename: String::from("./config.json"),
            margin_top: 5,
            margin_bottom: 5,
            margin_left: 5,
            margin_right: 5,
        }
    }

    pub fn get_orientation(&self) -> Option<Orientation> {

        match self.orientation.as_str() {
            "vertical" => Some(Orientation::Vertical),
            "horizontal" => Some(Orientation::Horizontal),
            _ => Option::None
        }
    }
}