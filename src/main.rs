use std::path::Path;

use gtk4::Application;
use gtk4::glib::ExitCode;
use gtk4::prelude::*;

mod actions;
mod config;
mod utils;
mod view;

const APP_ID: &str = "com.deerains.dummy-power-menu";

fn main() -> ExitCode {
    let config_path = "./config.json";
    let css_filename = Path::new("./style.css");

    let mut config = config::Config::new(config_path);

    match std::fs::exists(config_path) {
        Ok(true) => {
            config.load();
        }
        Ok(false) => {
            config.save();
        }
        Err(e) => {
            eprintln!("Faield to check cofnig filename: {e}");
        }
    }

    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(move |app| view::build_ui(app, &config, css_filename));
    app.run()
}
