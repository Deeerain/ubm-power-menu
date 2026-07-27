use std::process::Command;

use async_channel::Sender;
use gtk4::gdk::Key;
use gtk4::glib::Propagation;
use gtk4::{
    Application, ApplicationWindow, Box, CssProvider, gdk::Display, glib, prelude::*,
    style_context_add_provider_for_display,
};
use gtk4::{Button, EventControllerKey};

use crate::actions;
use crate::config;
use crate::utils;

pub fn build_ui(app: &Application, config: &config::Config) {
    let css_provider = CssProvider::new();
    css_provider.load_from_data(
        "button {
            font-size: 20pt;
            font-weight: bold;
        }",
    );

    if let Some(display) = Display::default() {
        style_context_add_provider_for_display(
            &display,
            &css_provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    let (sender, receiver) = async_channel::unbounded::<actions::PowerCommand>();
    let actions = action_definitions();

    let window = ApplicationWindow::builder()
        .application(app)
        .title(&config.application.title)
        .build();

    setup_layer_shell(&window);
    setup_controller(&window, &sender);

    println!("Orientation {:?}", config.get_orientation());
    let box_container = Box::new(
        config
            .get_orientation()
            .expect("Failed to set orientation; Orientatio is None"),
        12,
    );
    box_container.set_margin_top(config.application.margin_top);
    box_container.set_margin_bottom(config.application.margin_bottom);
    box_container.set_margin_start(config.application.margin_left);
    box_container.set_margin_end(config.application.margin_right);
    box_container.set_halign(gtk4::Align::Center);
    box_container.set_valign(gtk4::Align::Center);

    for btn in build_buttons(
        &sender,
        &actions,
        config.buttons.width,
        config.buttons.height,
    ) {
        box_container.append(&btn);
    }

    window.set_child(Some(&box_container));
    window.set_resizable(false);
    window.present();

    let window_clone = window.clone();
    let actions_for_loop = actions;
    glib::MainContext::default().spawn_local(async move {
        while let Ok(command) = receiver.recv().await {
            if let Some(action) = actions_for_loop
                .iter()
                .find(|entry| entry.command == command)
            {
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

fn action_definitions() -> Vec<actions::PowerAction> {
    let exit_spec = if utils::is_hyprland_running() {
        Some(actions::CommandSpec::new("hyprctl", &["dispatch", "exit"]))
    } else if utils::is_sway_running() {
        Some(actions::CommandSpec::new("swaymsg", &["exit"]))
    } else if utils::is_gnome_running() {
        Some(actions::CommandSpec::new(
            "gnome-session-quit",
            &["--power-off"],
        ))
    } else {
        Some(actions::CommandSpec::new(
            "loginctl",
            &["terminate-session", "self"],
        ))
    };

    vec![
        actions::PowerAction {
            command: actions::PowerCommand::Shutdown,
            label: "󰐥",
            spec: Some(actions::CommandSpec::new("systemctl", &["poweroff"])),
        },
        actions::PowerAction {
            command: actions::PowerCommand::Reboot,
            label: "󰑓",
            spec: Some(actions::CommandSpec::new("systemctl", &["reboot"])),
        },
        actions::PowerAction {
            command: actions::PowerCommand::Suspend,
            label: "󰤄",
            spec: Some(actions::CommandSpec::new("systemctl", &["suspend"])),
        },
        actions::PowerAction {
            command: actions::PowerCommand::Exit,
            label: "󰈆",
            spec: exit_spec,
        },
        actions::PowerAction {
            command: actions::PowerCommand::CloseMenu,
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

fn setup_controller(window: &ApplicationWindow, sender: &Sender<actions::PowerCommand>) {
    let key_controller = EventControllerKey::new();
    let tx = sender.clone();
    key_controller.connect_key_pressed(move |_, keyval, _, _| {
        let command = match keyval {
            Key::Escape => Some(actions::PowerCommand::CloseMenu),
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

fn build_buttons(
    sender: &Sender<actions::PowerCommand>,
    actions: &[actions::PowerAction],
    button_width: i32,
    button_height: i32,
) -> Vec<Button> {
    let mut result = Vec::<Button>::new();

    for action in actions {
        let tx = sender.clone();
        let button = Button::builder()
            .label(action.label)
            .width_request(button_width)
            .height_request(button_height)
            .build();
        let command = action.command;
        button.connect_clicked(move |_| {
            let _ = tx.send_blocking(command);
        });
        result.push(button);
    }

    result
}
