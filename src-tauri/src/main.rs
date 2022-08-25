#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{sync::mpsc::channel, time::Duration};

use notify::{watcher, RecursiveMode, Watcher};
use tauri::{
    api::notification::Notification, CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu,
};

fn main() {
    let mut app = tauri::Builder::default()
        .system_tray(get_system_tray())
        .on_system_tray_event(handle_tray_click)
        .invoke_handler(tauri::generate_handler![test])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    app.set_activation_policy(tauri::ActivationPolicy::Accessory);
    watch_file_system();
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

#[tauri::command]
fn test(app_handle: tauri::AppHandle) {
    Notification::new(&app_handle.config().tauri.bundle.identifier)
        .title("Hello")
        .body("World")
        .show()
        .expect("error while showing notification");
}

fn watch_file_system() {
    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_secs(10)).unwrap();

    watcher
        .watch(
            dirs::home_dir()
                .expect("Could not get home dir")
                .to_str()
                .expect("Not a valid path?")
                .to_owned()
                + "/screenshots",
            RecursiveMode::NonRecursive,
        )
        .unwrap();

    loop {
        match rx.recv() {
            Ok(event) => println!("{:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
