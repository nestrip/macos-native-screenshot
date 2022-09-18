use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct UserConfig {
    pub api_key: String,
    pub setup: bool,
}

pub fn get_config(app: &tauri::AppHandle) -> UserConfig {
    let path = app.path_resolver().app_dir().unwrap().join("config.toml");

    let config: UserConfig = confy::load_path(path).expect("Could not load config");

    config
}

pub fn set_config(app: &tauri::AppHandle, config: UserConfig) {
    let path = app.path_resolver().app_dir().unwrap().join("config.toml");

    confy::store_path(path, config).expect("Could not save config");
}
