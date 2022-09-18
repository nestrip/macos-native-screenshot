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
            commands::set_api_key,
            commands::test_upload,
            commands::set_setup,
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    let app_handle = app.handle();

    let config = crate::config::get_config(&app_handle);

    if config.setup {
        app.set_activation_policy(tauri::ActivationPolicy::Accessory);

        thread::spawn(move || {
            listeners::watch_file_system(&app_handle);
        });
    } else {
        tauri::WindowBuilder::new(&app, "local", tauri::WindowUrl::App("setup.html".into()))
            .title("Setup nest.rip uploader")
            .inner_size(800f64, 400f64)
            .focus()
            .build()
            .expect("Could not start window");
    }

    app.run(move |_app_handle, event| match event {
        tauri::RunEvent::ExitRequested { api, .. } => {
            if config.setup {
                api.prevent_exit();
            }
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
                let window = tauri::WindowBuilder::new(
                    app,
                    "local",
                    tauri::WindowUrl::App("index.html".into()),
                )
                .always_on_top(true)
                .title("nest.rip - Options")
                .build()
                .expect("error while creating local window");

                window.on_window_event(|event| match event {
                    tauri::WindowEvent::CloseRequested { api, .. } => {
                        api.prevent_close();
                    }
                    _ => {}
                });
            }
            "quit" => {
                std::process::exit(0);
            }
            _ => {}
        },
        _ => {}
    }
}
