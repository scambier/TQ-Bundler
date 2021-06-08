use crate::{compile, config::Config};
use notify::{DebouncedEvent::*, RecommendedWatcher, RecursiveMode, Watcher};
use std::{sync::mpsc::channel, time::Duration};

pub fn watch(config: &Config) -> notify::Result<()> {
    let (sender, receiver) = channel();

    let mut watcher: RecommendedWatcher = Watcher::new(sender, Duration::from_millis(50)).unwrap();

    watcher
        .watch(&config.base_folder, RecursiveMode::Recursive)
        .unwrap(); // Panic if the watcher can't watch

    loop {
        match receiver.recv() {
            Ok(event) => {
                match event {
                    // Trigger rebuild on file write|delete
                    Create(path) | Write(path) | Remove(path) => {
                        if path.is_file()
                            && path.to_string_lossy().ends_with(&config.filetype.extension)
                            && !path.ends_with(&config.output_file)
                        {
                            compile(&config);
                        }
                    }
                    _ => {}
                }
            }
            Err(e) => {
                println!("Watch error: {:?}", e);
            }
        }
    }
}
