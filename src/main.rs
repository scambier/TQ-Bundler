use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use regex::Regex;
use std::{
    fs,
    path::{Path, PathBuf},
    sync::mpsc::channel,
    time::Duration,
};
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
                    notify::DebouncedEvent::Write(path) => {
                        if !path.ends_with("compiled.fnl") {
                            compile();
                        }
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

    // The entry point MUST be named "main.fnl"
    // TODO: allow any entry point (CLI param)
    let mut modules: Vec<Module> = vec![Module::new(&base_path.to_path_buf(), "main".to_string())];
    let mut requires: Vec<PathBuf> = vec![];

    // Reference all the modules
    let mut to_add: Vec<Module> = vec![];
    loop {
        modules.append(&mut to_add);
        for module in modules.to_vec().iter_mut() {
            for (cap, pos) in re_require
                .captures_iter(&module.contents.clone())
                .zip(re_require.find_iter(&module.contents.clone()))
            {
                let name = cap.get(1).unwrap().as_str().to_string();
                let path = Module::get_module_path(&module.path, &name);
                if !Module::has_module(&modules, &path) {
                    // Module does not already exist, load it
                    to_add.push(Module::from_path(path.clone()));

                    // De-duplicate requires
                    if requires.contains(&path) {
                        module.contents.replace_range(pos.range(), "");
                    } else {
                        requires.push(path.clone());
                    }
                }
                println!("{:?}", &cap);
            }
        }
        if to_add.len() == 0 {
            break;
        }
    }

    // Loop until the entry point no longer has requires
    let mut copy = modules.to_vec();
    let entry_point = copy.first_mut().unwrap();
    loop {
        let cloned_contents = entry_point.contents.clone();
        match (
            re_require.captures(&cloned_contents),
            re_require.find(&cloned_contents),
        ) {
            (Some(cap), Some(pos)) => {
                let mod_name = cap.get(1).unwrap().as_str().to_string();
                let path = Module::get_module_path(&entry_point.path, &mod_name);
                let module = modules.iter().find(|m| m.path == path).unwrap();
                entry_point
                    .contents
                    .replace_range(pos.range(), &module.contents);
            }
            _ => {
                break;
            }
        }

        // for (cap, pos) in re_require
        //     .captures_iter(&entry_point.contents.clone())
        //     .zip(re_require.find_iter(&entry_point.contents.clone()))
        // {
        //     let mod_name = &cap.get(1).unwrap().as_str().to_string();
        //     let path = Module::get_module_path(&entry_point.path.clone(), &mod_name);
        //     let module = modules.iter().find(|m| m.path == path).unwrap();
        //     entry_point
        //         .contents
        //         .replace_range(pos.range(), &module.contents);
        // }
        if !re_require.is_match(&entry_point.contents) {
            // Break once we recursively replaced all requires in the entry point
            break;
        }
    }

    for module in modules {
        println!("{:?}", module.path);
    }

    fs::write(base_path.join("compiled.fnl"), &entry_point.contents);

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
