use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};

#[derive(Debug, Deserialize, Serialize, Default, Validate)]
pub struct UserConfig {
    #[validate(length(min = 1, message = "API key cannot be empty"))]
    pub api_key: String,
    pub setup: bool,
}

pub fn get_config(app: &tauri::AppHandle) -> UserConfig {
    let path = app.path_resolver().app_dir().unwrap().join("config.toml");

    let config: UserConfig = confy::load_path(path).expect("Could not load config");

    if let Err(e) = config.validate() {
        eprintln!("Invalid configuration: {:?}", e);
        panic!("Invalid configuration");
    }

    config
}

pub fn set_config(app: &tauri::AppHandle, config: UserConfig) {
    if let Err(e) = config.validate() {
        eprintln!("Invalid configuration: {:?}", e);
        panic!("Invalid configuration");
    }

    let path = app.path_resolver().app_dir().unwrap().join("config.toml");

    confy::store_path(path, config).expect("Could not save config");
}
