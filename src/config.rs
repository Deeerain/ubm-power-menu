use std::{path::PathBuf};

use gtk4::Orientation;
use log::warn;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ButtonsConfig {
    pub height: i32,
    pub width: i32,
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
            margin_top: MARGINS,
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
}

impl Config {
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
            application: AppConfig::default(),
            buttons: ButtonsConfig::default(),
        }
    }
}

pub fn load(path: PathBuf) -> Result<Config, ()> {
    match std::fs::read_to_string(path) {
        Ok(raw) => {
            match serde_json::from_str::<Config>(&raw){
                Ok(config) => Ok(config),
                Err(e) => {
                    warn!("Failed to load config file: {}", e);
                    Ok(Config::default())
                },
            }
        },
        Err(_) => Ok(Config::default())
    }
}

pub fn save(path: PathBuf, config: &Config) -> Result<(), ()> {
    match serde_json::to_string_pretty(config) {
        Ok(json) => {
            match std::fs::write(path, json) {
                Ok(_) => Ok(()),
                Err(_) => Err(()),
            }
        },
        Err(_) => Err(())
    }
}