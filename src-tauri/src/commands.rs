use crate::upload::upload_file_to_nest;

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
#[tauri::command]
pub fn test_upload(app_handle: tauri::AppHandle) {
    let file = app_handle
        .path_resolver()
        .resolve_resource("images/test.png")
        .expect("Could not find test file");

    upload_file_to_nest(&file, &app_handle);
}
