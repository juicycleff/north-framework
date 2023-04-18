use std::path::Path;
use notify::{ RecursiveMode, Watcher };

pub fn watch_file_path(path: Box<Path>) -> Result<(), crate::Error> {
    // Automatically select the best implementation for your platform.
    let mut watcher = notify::recommended_watcher(|res| {
        match res {
            Ok(event) => println!("event: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }).unwrap();

    watcher.watch(&*path, RecursiveMode::Recursive).unwrap();

    Ok(())
}