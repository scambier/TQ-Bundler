mod config;
mod module;

use chrono::Local;
use clap::{App, Arg};
use config::*;
use module::*;
use notify::{DebouncedEvent::*, RecommendedWatcher, RecursiveMode, Watcher};
use regex::Regex;
use std::{
    fs,
    path::PathBuf,
    process::{exit, Child, Command},
    sync::{mpsc::channel, Mutex},
    time::Duration,
};

fn log(str: String) {
    println!("{:} - {:}", Local::now().format("%M:%m:%S"), str);
}

fn watch(config: &Config) -> notify::Result<()> {
    let (sender, receiver) = channel();

    let mut watcher: RecommendedWatcher = Watcher::new(sender, Duration::from_millis(500)).unwrap();

    watcher
        .watch(&config.base_folder, RecursiveMode::Recursive)
        .unwrap();

    loop {
        match receiver.recv() {
            Ok(event) => {
                match event {
                    // Trigger rebuild on file write|delete
                    NoticeWrite(path) | NoticeRemove(path) => {
                        if !path.ends_with(&config.output_file) {
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

fn compile(config: &Config) -> bool {
    let re_require = Regex::new(r"\(include :(.+)\)").unwrap();

    // Check the entry point
    let entry = Module::new(&config.base_folder, &config.entry_point);
    assert!(entry.is_ok(), "Could not find file {}", &config.entry_point);
    let entry = entry.unwrap();

    let mut modules: Vec<Module> = vec![entry];
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
                    match Module::from_path(path.clone()) {
                        Ok(module) => {
                            to_add.push(module);
                        }
                        Err(_) => {
                            log(format!(":( Could not find module {:?}", &path));
                            return false;
                        }
                    }

                    // De-duplicate requires
                    if requires.contains(&path) {
                        module.contents.replace_range(pos.range(), "");
                    } else {
                        requires.push(path.clone());
                    }
                }
                // println!("{:?}", &cap);
            }
        }
        if to_add.len() == 0 {
            break;
        }
    }

    // Loop until the main file no longer has requires
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

                // Inject code into the main file
                let module_contents = &format!(
                    ";; {:}\n\n{:}\n;; /{:}\n",
                    &mod_name, &module.contents, &mod_name
                );
                entry_point
                    .contents
                    .replace_range(pos.range(), module_contents);
            }
            _ => {
                break;
            }
        }

        if !re_require.is_match(&entry_point.contents) {
            // Break once we recursively replaced all requires in the entry point
            break;
        }
    }

    let names = modules
        .iter()
        .map(|m| m.path.file_name().unwrap().to_str().unwrap())
        .collect::<Vec<_>>()
        .join(", ");
    log(format!(
        "Compiled {:} files into {:}: {:}",
        modules.len(),
        &config.output_file,
        names
    ));

    let success = fs::write(
        config.base_folder.join(&config.output_file),
        &entry_point.contents,
    );
    match success {
        Ok(_) => {}
        Err(e) => {
            println!("Could not write output file:");
            println!("{:?}", e);
        }
    };
    true
}

fn main() {
    let matches = App::new("TIC-80 Bundler")
        .version("1.0.0")
        .arg(
            Arg::with_name("FILE")
                .help("The entry point of your TIC-80 game")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .short("o")
                .long("output")
                .help("The entry point of your TIC-80 game")
                .takes_value(true)
                .default_value("build.fnl")
                .required(false),
        )
        .arg(
            Arg::with_name("TIC")
                .long("--tic")
                .help("Path to the TIC-80 executable. If specified, will launch TIC-80 in watch mode, with your game loaded.")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("WATCH")
                .short("w")
                .long("watch")
                .help("Watch for changes and rebuild automatically"),
        )
        .get_matches();

    let config = Config::new(&matches);

    // First compilation
    let compiled = compile(&config);

    // If compilation failed and we need to launch TIC, abort
    if !compiled && config.tic_path.is_some() {
        println!("Compilation failed - Could not launch TIC-80");
        return;
    }

    // Start TIC-80
    let tic_path = config.tic_path.clone();
    let tic_process_mtx: Mutex<Option<Child>> = Mutex::new(None);
    match tic_path {
        Some(tic) => {
            let output_path = config
                .base_folder
                .join(&config.output_file)
                .to_str()
                .unwrap()
                .to_string();
            tic_process_mtx.lock().unwrap().replace(
                Command::new(tic)
                    .args(&["-code-watch", &output_path])
                    .spawn()
                    .expect("Failed to launch TIC-80"),
            );
        }
        None => {}
    }

    ctrlc::set_handler(move || {
        let child = tic_process_mtx.lock().unwrap().take();
        // Kill TUC-80 if it is launched
        if let Some(mut child) = child {
            let _ = child.kill();
        }
        exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    // Start the watcher
    if config.watch {
        if let Err(e) = watch(&config) {
            println!("error: {:?}", e)
        }
    }
}
