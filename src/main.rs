use std::path::Path;

use gtk4::Application;
use gtk4::glib::ExitCode;
use gtk4::prelude::*;

use log::{info, debug, error};

mod actions;
mod config;
mod utils;
mod view;

const APP_ID: &str = "com.deerains.dummy-power-menu";
fn main() -> ExitCode {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Debug)
        .init();


    let config_path = "./config.json";
    let css_filename = Path::new("./style.css");

    let mut config = config::Config::new(config_path);

    info!("Loading config...");
    match std::fs::exists(config_path) {
        Ok(true) => {
            debug!("Config file exist. Loading");
            config.load();
        }
        Ok(false) => {
            debug!("Config file not exist. Save default");
            config.save();
        }
        Err(e) => {
            error!("Failed to check config file: {}", e);
        }
    }

    info!("Config file loaded");

    info!("Init application");
    let app = Application::builder().application_id(APP_ID).build();

    info!("Init ui");
    app.connect_activate(move |app| view::build_ui(app, &config, css_filename));

    info!("Run application");
    app.run()
}
