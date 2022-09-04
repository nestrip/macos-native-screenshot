use notify::{raw_watcher, RawEvent, RecursiveMode, Watcher};
use std::sync::mpsc::channel;

use crate::files;

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
                if op.contains(notify::Op::WRITE)
                    && files::is_image(&path)
                    && !op.contains(notify::Op::RENAME)
                    && last_image != path.to_str().unwrap()
                {
                    last_image = path.to_str().unwrap().to_owned();
                    files::copy_image_to_clipboard(&path);
                }
            }
            Ok(event) => println!("broken event: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
