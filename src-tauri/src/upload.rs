use std::path::Path;

use arboard::Clipboard;
use reqwest::blocking::{
    multipart::{Form, Part},
    Client,
};
use serde::Deserialize;

pub fn upload_file_to_nest(file: &Path) {
    let replaced_path = file.as_os_str().to_string_lossy().replace("/.", "/"); //Macos removes the . from the file name
    let path = Path::new(&replaced_path);

    println!("Uploading file to nest {}", path.display());

    let data = Client::new()
        .post("https://nest.rip/api/files/upload")
        .header("Authorization", "Pringles_aJgFWlzmlGLSv6kBtitqzWGNwv0lqCpc")
        .multipart(Form::new().part("files", Part::file(path).unwrap()))
        .send()
        .unwrap();

    let response = data.json::<UploadResponse>().unwrap();
    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_text(response.fileURL).unwrap();
}

#[allow(dead_code, non_snake_case)]
#[derive(Deserialize)]
struct UploadResponse {
    cdnFileName: String,
    deletionURL: String,
    fileName: String,
    fileURL: String,
}
