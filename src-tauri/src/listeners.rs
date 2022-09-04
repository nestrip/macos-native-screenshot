use notify::{raw_watcher, RawEvent, RecursiveMode, Watcher};
use std::sync::mpsc::channel;

use crate::{files, upload};

pub fn watch_file_system() {
    let mut last_image = String::new();
    let (tx, rx) = channel();
    let mut watcher = raw_watcher(tx).unwrap();

    watcher
        .watch(
            dirs::home_dir()
                .expect("Could not get home dir")
                .to_str()
                .expect("Not a valid path?")
                .to_owned()
                + "/screenshots",
            RecursiveMode::NonRecursive,
        )
        .unwrap();

    loop {
        match rx.recv() {
            Ok(RawEvent {
                path: Some(path),
                op: Ok(op),
                cookie: _,
            }) => {
                if !(op.contains(notify::Op::RENAME)
                    && !path.file_name().unwrap().to_string_lossy().starts_with(".")
                    && files::is_image(&path)
                    && last_image != path.to_str().unwrap())
                {
                    continue;
                }

                last_image = path.to_str().unwrap().to_owned();

                // copy the image to the clipboard, just incase the request fails
                files::copy_image_to_clipboard(&path);

                // upload the image to nest
                upload::upload_file_to_nest(&path);

                // delete the file from the file system to prevent any unneeded files
                files::delete_file(&path);
            }
            Ok(event) => println!("broken event: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
