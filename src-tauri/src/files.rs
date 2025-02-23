use arboard::{Clipboard, ImageData};
use image::io::Reader as ImageReader;
use rodio::{Decoder, OutputStream, Source};
use std::{borrow::Cow, fs::File, io::BufReader, path::Path};

pub fn copy_image_to_clipboard(path: &Path) {
    let file = match ImageReader::open(path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Could not open image: {}", e);
            return;
        }
    };

    let image = match file.decode() {
        Ok(image) => image,
        Err(e) => {
            eprintln!("Could not decode image: {}", e);
            return;
        }
    };

    let mut clipboard = match Clipboard::new() {
        Ok(clipboard) => clipboard,
        Err(e) => {
            eprintln!("Could not create clipboard: {}", e);
            return;
        }
    };

    let img_data = ImageData {
        width: match usize::try_from(image.width()) {
            Ok(width) => width,
            Err(e) => {
                eprintln!("Could not convert width: {}", e);
                return;
            }
        },
        height: match usize::try_from(image.height()) {
            Ok(height) => height,
            Err(e) => {
                eprintln!("Could not convert height: {}", e);
                return;
            }
        },
        bytes: Cow::Borrowed(image.as_bytes()),
    };

    if let Err(e) = clipboard.set_image(img_data) {
        eprintln!("Could not set image to clipboard: {}", e);
    }
}

pub fn is_image(path: &Path) -> bool {
    let ext = path.extension().unwrap().to_str().unwrap();
    match ext {
        "png" | "jpg" | "jpeg" | "gif" | "bmp" | "webp" => true,
        _ => false,
    }
}

pub fn delete_file(file: &Path) {
    std::fs::remove_file(file).expect("Could not delete file");
}

pub fn play_audio_file(app_handle: &tauri::AppHandle, file: &str) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(
        File::open(
            app_handle
                .path_resolver()
                .resolve_resource(file)
                .expect("Could not find sound file")
                .as_path(),
        )
        .unwrap(),
    );

    let source = Decoder::new(file).unwrap();
    stream_handle.play_raw(source.convert_samples()).unwrap();

    std::thread::sleep(std::time::Duration::from_secs(1));
}
