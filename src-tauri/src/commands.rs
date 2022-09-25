use std::fs;

use crate::upload::upload_file_to_nest;

#[tauri::command]
pub fn get_api_key(app_handle: tauri::AppHandle) -> String {
    let config = crate::config::get_config(&app_handle);

    config.api_key
}

#[tauri::command]
pub fn set_api_key(app_handle: tauri::AppHandle, api_key: String) {
    let mut config = crate::config::get_config(&app_handle);

    config.api_key = api_key;

    crate::config::set_config(&app_handle, config);
}
#[tauri::command]
pub fn set_setup(app_handle: tauri::AppHandle) {
    let mut config = crate::config::get_config(&app_handle);

    config.setup = true;

    crate::config::set_config(&app_handle, config);

    // To help the user, create the screenshots folder by default
    fs::create_dir_all(
        dirs::home_dir()
            .expect("Could not get home dir")
            .to_str()
            .expect("Not a valid path?")
            .to_owned()
            + "/screenshots",
    )
    .expect("Failed to create screenshots folder");
}

#[tauri::command]
pub fn test_upload(app_handle: tauri::AppHandle) -> bool {
    let file = app_handle
        .path_resolver()
        .resolve_resource("images/test.png")
        .expect("Could not find test file");

    upload_file_to_nest(&file, &app_handle).is_ok()
}
