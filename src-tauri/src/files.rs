use arboard::{Clipboard, ImageData};
use image::io::Reader as ImageReader;
use std::{borrow::Cow, path::Path};

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
