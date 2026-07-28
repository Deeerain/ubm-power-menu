use std::path::{Path, PathBuf};

use gtk4::Application;
use gtk4::glib::ExitCode;
use gtk4::prelude::*;

use log::{error, info, warn};

use crate::config::Config;

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

    let app_config: Config;

    match config::load(PathBuf::from(config_path)) {
        Ok(conf) => app_config = conf,
        Err(e) => {
            warn!(
                "Failded to load config file: {:?}. Using default config file",
                e
            );
            app_config = Config::default();
            match config::save(PathBuf::from(config_path), &app_config) {
                Ok(()) => info!("Config file saved"),
                Err(()) => error!("Failed to save config file"),
            }
        }
    }

    info!("Config file loaded");

    info!("Init application");
    let app = Application::builder().application_id(APP_ID).build();

    info!("Init ui");
    app.connect_activate(move |app| view::build_ui(app, &app_config, css_filename));

    info!("Run application");
    app.run()
}
