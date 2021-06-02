use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use regex::Regex;
use std::{fs, path::Path, sync::mpsc::channel, time::Duration};
mod module;
use module::*;

static BASE_PATH: &str = "C:/Users/cambi/AppData/Roaming/com.nesbox.tic/TIC-80/roguelike";

fn watch() -> notify::Result<()> {
    let (sender, receiver) = channel();

    let mut watcher: RecommendedWatcher = Watcher::new(sender, Duration::from_millis(500)).unwrap();

    watcher.watch(BASE_PATH, RecursiveMode::Recursive).unwrap();

    loop {
        match receiver.recv() {
            Ok(event) => {
                println!("{:?}", event);
                match event {
                    // notify::DebouncedEvent::NoticeWrite(_) => todo!(),
                    // notify::DebouncedEvent::NoticeRemove(_) => todo!(),
                    // notify::DebouncedEvent::Create(_) => todo!(),
                    notify::DebouncedEvent::Write(_) => {
                        compile();
                    }
                    // notify::DebouncedEvent::Chmod(_) => todo!(),
                    // notify::DebouncedEvent::Remove(_) => todo!(),
                    // notify::DebouncedEvent::Rename(_, _) => todo!(),
                    // notify::DebouncedEvent::Rescan => todo!(),
                    // notify::DebouncedEvent::Error(_, _) => todo!(),
                    _ => {}
                }
            }
            Err(e) => {
                println!("Watch error: {:?}", e);
            }
        }
    }
}

fn compile() {
    let re_require = Regex::new(r"\(require :(.+)\)").unwrap();
    let base_path = Path::new(BASE_PATH);

    let mut modules: Vec<Module> = vec![Module::new(&base_path.to_path_buf(), "main".to_string())];
    let mut to_add: Vec<Module> = vec![];

    loop {
        modules.append(&mut to_add);
        for module in &modules {
            for cap in re_require.captures_iter(&module.contents) {
                let name = cap.get(1).unwrap().as_str().to_string();
                let path = Module::get_module_path(&module.full_path, &name);
                if !Module::has_module(&modules, &path) {
                    // Module does not already exist, load it
                    to_add.push(Module::from_path(path))
                }
                println!("{:?}", &cap);
            }
        }

        if to_add.len() == 0 {
            break;
        }
    }

    for module in modules {
        println!("{:?}", module.full_path);
    }

    // for cap in re_require.captures_iter(&file_main) {
    //     println!("{:?}", &cap);
    // }

    // for loc in re_require.find_iter(&file_main) {
    //     println!("{:?}-{:?}: {:?}", &loc.start(), &loc.end(), &loc.as_str());
    // }
}

fn main() {
    // Compile a first time
    compile();
    if let Err(e) = watch() {
        println!("error: {:?}", e)
    }
}
