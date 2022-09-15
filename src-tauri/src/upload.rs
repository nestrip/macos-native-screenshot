use std::path::Path;

use arboard::Clipboard;
use reqwest::blocking::{
    multipart::{Form, Part},
    Client,
};
use serde::Deserialize;
use tauri::api::notification::Notification;

use crate::files;

#[allow(dead_code, non_snake_case)]
#[derive(Deserialize)]
struct UploadResponse {
    cdnFileName: String,
    deletionURL: String,
    fileName: String,
    fileURL: String,
}

#[allow(dead_code, non_snake_case)]
#[derive(Deserialize)]
struct ErrorResponse {
    message: String,
}

pub fn upload_file_to_nest(file: &Path, app_handle: &tauri::AppHandle) {
    let replaced_path = file.as_os_str().to_string_lossy().replace("/.", "/"); //Macos removes the . from the file name
    let path = Path::new(&replaced_path);

    println!("Uploading file to nest {}", path.display());

    // Load the config to get the users api key
    let config = crate::config::get_config(&app_handle);

    let data = Client::new()
        .post("https://nest.rip/api/files/upload")
        .header("Authorization", config.api_key)
        .multipart(Form::new().part("files", Part::file(path).unwrap()))
        .send()
        .unwrap();

    // Request failed...displaying meswsage to user
    if data.status() != 200 {
        files::play_audio_file(app_handle, "sounds/error.wav");
        let response = data.json::<ErrorResponse>().unwrap();
        display_error_message(app_handle, response);
        return;
    }

    display_successfull_notification(app_handle, path);
    files::play_audio_file(app_handle, "sounds/upload.wav");

    let response = data
        .json::<UploadResponse>()
        .expect("Could not parse upload response");
    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_text(response.fileURL).unwrap();
}

fn display_successfull_notification(app_handle: &tauri::AppHandle, path: &Path) {
    Notification::new(app_handle.config().tauri.bundle.identifier.clone())
        .title("Uploaded to nest")
        .body(format!(
            "Uploaded {} to nest, and copied url to clipboard",
            path.file_name().unwrap().to_str().unwrap(),
        ))
        .show()
        .expect("error while showing notification");
}

fn display_error_message(app_handle: &tauri::AppHandle, response: ErrorResponse) {
    Notification::new(app_handle.config().tauri.bundle.identifier.clone())
        .title("Could not upload to nest")
        .body(response.message)
        .show()
        .expect("error while showing notification");
}
