use std::process::Command;

use async_channel::Sender;
use gtk4::gdk::Key;
use gtk4::glib::{ExitCode, Propagation};
use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Box, Button, EventControllerKey, Orientation, glib,
};

mod cofnig;

const APP_ID: &str = "com.deerains.dummy-power-menu";

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum PowerCommand {
    Shutdown,
    Reboot,
    Suspend,
    Exit,
    CloseMenu,
}

#[derive(Clone, Copy)]
struct CommandSpec {
    program: &'static str,
    args: &'static [&'static str],
}

#[derive(Clone, Copy)]
struct PowerAction {
    command: PowerCommand,
    label: &'static str,
    spec: Option<CommandSpec>,
}

impl CommandSpec {
    const fn new(program: &'static str, args: &'static [&'static str]) -> Self {
        Self { program, args }
    }
}

fn main() -> ExitCode {
    let config_path = "./config.json";

    let mut config = cofnig::Config::new(config_path);

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
    app.connect_activate(move |app| build_ui(app, &config));
    app.run()
}

fn build_ui(app: &Application, config: &cofnig::Config) {
    let (sender, receiver) = async_channel::unbounded::<PowerCommand>();
    let actions = action_definitions();

    let window = ApplicationWindow::builder()
        .application(app)
        .title(&config.title)
        .build();

    setup_layer_shell(&window);
    setup_controller(&window, &sender);

    println!("Orientation {:?}", config.get_orientation());
    let box_container = Box::new(config.get_orientation().expect("Failed to set orientation; Orientatio is None"), 12);
    box_container.set_margin_top(config.margin_top);
    box_container.set_margin_bottom(config.margin_bottom);
    box_container.set_margin_start(config.margin_left);
    box_container.set_margin_end(config.margin_right);
    box_container.set_halign(gtk4::Align::Center);
    box_container.set_valign(gtk4::Align::Center);

    for btn in build_buttons(&sender, &actions) {
        box_container.append(&btn);
    }

    window.set_child(Some(&box_container));
    window.set_resizable(false);
    window.present();

    let window_clone = window.clone();
    let actions_for_loop = actions;
    glib::MainContext::default().spawn_local(async move {
        while let Ok(command) = receiver.recv().await {
            if let Some(action) = actions_for_loop.iter().find(|entry| entry.command == command) {
                if let Some(spec) = action.spec {
                    match Command::new(spec.program).args(spec.args).spawn() {
                        Ok(_) => {}
                        Err(err) => {
                            eprintln!("Failed to run {:?}: {err}", action.command);
                        }
                    }
                }
            }

            window_clone.close();
            break;
        }
    });
}

fn action_definitions() -> Vec<PowerAction> {
    let exit_spec = if is_hyprland_running() {
        Some(CommandSpec::new("hyprctl", &["dispatch", "exit"]))
    } else if is_sway_running() {
        Some(CommandSpec::new("swaymsg", &["exit"]))
    } else if is_gnome_running() {
        Some(CommandSpec::new("gnome-session-quit", &["--power-off"]))
    } else {
        Some(CommandSpec::new("loginctl", &["terminate-session", "self"]))
    };

    vec![
        PowerAction {
            command: PowerCommand::Shutdown,
            label: "󰐥",
            spec: Some(CommandSpec::new("systemctl", &["poweroff"])),
        },
        PowerAction {
            command: PowerCommand::Reboot,
            label: "󰑓",
            spec: Some(CommandSpec::new("systemctl", &["reboot"])),
        },
        PowerAction {
            command: PowerCommand::Suspend,
            label: "󰤄",
            spec: Some(CommandSpec::new("systemctl", &["suspend"])),
        },
        PowerAction {
            command: PowerCommand::Exit,
            label: "󰈆",
            spec: exit_spec,
        },
        PowerAction {
            command: PowerCommand::CloseMenu,
            label: "",
            spec: None,
        },
    ]

    
}

fn setup_layer_shell(window: &ApplicationWindow) {
    use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};
    window.init_layer_shell();
    window.set_layer(Layer::Overlay);
    window.set_keyboard_mode(KeyboardMode::Exclusive);
    window.set_anchor(Edge::Left, false);
    window.set_anchor(Edge::Right, false);
    window.set_anchor(Edge::Top, false);
    window.set_anchor(Edge::Bottom, false);
}

fn setup_controller(window: &ApplicationWindow, sender: &Sender<PowerCommand>) {
    let key_controller = EventControllerKey::new();
    let tx = sender.clone();
    key_controller.connect_key_pressed(move |_, keyval, _, _| {
        let command = match keyval {
            Key::Escape => Some(PowerCommand::CloseMenu),
            _ => None,
        };

        if let Some(cmd) = command {
            let _ = tx.send_blocking(cmd);
            Propagation::Proceed
        } else {
            Propagation::Stop
        }
    });
    window.add_controller(key_controller);
}

fn build_buttons(sender: &Sender<PowerCommand>, actions: &[PowerAction]) -> Vec<Button> {
    let mut result = Vec::<Button>::new();

    for action in actions {
        let tx = sender.clone();
        let button = Button::builder().label(action.label).build();
        let command = action.command;
        button.connect_clicked(move |_| {
            let _ = tx.send_blocking(command);
        });
        result.push(button);
    }

    result
}

fn is_hyprland_running() -> bool {
    std::env::var("HYPRLAND_INSTANCE_SIGNATURE").is_ok()
}

fn is_sway_running() -> bool {
    std::env::var("SWAYSOCK").is_ok()
}

fn is_gnome_running() -> bool {
    std::env::var("XDG_CURRENT_DESKTOP")
        .map(|desktop| desktop.to_lowercase().contains("gnome"))
        .unwrap_or(false)
}