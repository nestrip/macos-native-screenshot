use arboard::{Clipboard, ImageData};
use image::io::Reader as ImageReader;
use rodio::{Decoder, OutputStream, Source};
use std::{borrow::Cow, fs::File, io::BufReader, path::Path};

pub fn copy_image_to_clipboard(path: &Path) {
    let file = ImageReader::open(path).expect("Could not open image");

    let image = file.decode().expect("Could not decode image");

    let mut clipboard = Clipboard::new().unwrap();

    let img_date = ImageData {
        width: usize::try_from(image.width()).expect("Could not convert width"),
        height: usize::try_from(image.height()).expect("Could not convert width"),
        bytes: Cow::Borrowed(image.as_bytes()),
    };
    clipboard.set_image(img_date).unwrap();
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
