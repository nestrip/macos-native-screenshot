#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod config;
mod files;
mod listeners;
mod upload;

use std::thread;
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu};

fn main() {
    let mut app = tauri::Builder::default()
        .system_tray(get_system_tray())
        .on_system_tray_event(handle_tray_click)
        .invoke_handler(tauri::generate_handler![
            commands::get_api_key,
            commands::set_api_key
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    app.set_activation_policy(tauri::ActivationPolicy::Accessory);

    let app_handle = app.handle();

    thread::spawn(move || {
        listeners::watch_file_system(&app_handle);
    });

    app.run(|_app_handle, event| match event {
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        _ => {}
    });
}

fn get_system_tray() -> SystemTray {
    let title = CustomMenuItem::new("title", "nest.rip screenshot tool").disabled();

    let options = CustomMenuItem::new("options".to_string(), "Show Options");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");

    let tray_menu = SystemTrayMenu::new()
        .add_item(title)
        .add_native_item(tauri::SystemTrayMenuItem::Separator)
        .add_item(options)
        .add_item(quit);

    SystemTray::new().with_menu(tray_menu)
}

fn handle_tray_click(app: &tauri::AppHandle, event: tauri::SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "options" => {
                tauri::WindowBuilder::new(app, "local", tauri::WindowUrl::App("index.html".into()))
                    .always_on_top(true)
                    .title("Options Window")
                    .build()
                    .expect("error while creating local window");
            }
            "quit" => {
                std::process::exit(0);
            }
            _ => {}
        },
        _ => {}
    }
}
