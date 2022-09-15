#[tauri::command]
pub fn get_api_key(app_handle: tauri::AppHandle) -> String {
    let config = crate::config::get_config(&app_handle);

    println!("API KEY: {}", config.api_key);

    config.api_key
}

#[tauri::command]
pub fn set_api_key(app_handle: tauri::AppHandle, api_key: String) {
    let mut config = crate::config::get_config(&app_handle);

    config.api_key = api_key;

    println!("API KEY: {}", config.api_key);

    crate::config::set_config(&app_handle, config);
}
