mod config;
mod module;

use chrono::Local;
use clap::{App, Arg, ArgMatches, SubCommand};
use config::*;
use module::*;
use notify::{DebouncedEvent::*, RecommendedWatcher, RecursiveMode, Watcher};
use std::{
    fs,
    path::PathBuf,
    process::{exit, Child, Command},
    sync::{mpsc::channel, Mutex},
    time::Duration,
};

fn main() {
    let matches = App::new("TIC-80 Bundler")
        .version("1.0.0")
        .subcommand(
            SubCommand::with_name("run")
            .about("Bundle and launch your game")
            .arg(
                Arg::with_name("GAME")
                    .help("The TIC game file in which the bundled code will be injected")
                    .required(true)
                    .index(1),
            )
            .arg(
                Arg::with_name("CODE")
                    .help("The \"main\" code file that will be injected inside the game")
                    .required(true)
                    .index(2)
            )
            .arg(
                Arg::with_name("OUTPUT")
                    .short("o")
                    .long("output")
                    .help("The entry point of your TIC-80 game")
                    .takes_value(true)
                    .required(false),
            )
            .arg(
                Arg::with_name("TIC")
                    .value_name("path")
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
        )
        .subcommand(
            SubCommand::with_name("init").about("Initialize a TIC-80 project")
            .arg(Arg::with_name("LANG").help(r#""fnl", "wren""#))
        )
        .get_matches();

    // Create a config file from the CLI arguments
    if let Some(runargs) = matches.subcommand_matches("run") {
        run(&runargs);
    }
}

fn log(str: String) {
    println!("{:} - {:}", Local::now().format("%M:%m:%S"), str);
}

fn watch(config: &Config) -> notify::Result<()> {
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
                            && path.ends_with(&config.filetype.extension)
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

fn compile(config: &Config) -> bool {
    let re_include = &config.filetype.regex;

    // Check the entry point
    let main_file = Module::new(
        &config.base_folder,
        &config.entry_point,
        &config.filetype.extension,
    );
    assert!(
        main_file.is_ok(),
        "Could not find file {}",
        &config.entry_point
    );
    let main_file = main_file.unwrap();

    // List of files to include, starting with the entry file
    let mut modules: Vec<Module> = vec![main_file];
    // Modules to add once the loop is over
    let mut to_add: Vec<Module> = vec![];
    // List of included file paths
    let mut includes: Vec<PathBuf> = vec![];

    // Index all the modules
    loop {
        modules.append(&mut to_add);

        for module in modules.to_vec().iter_mut() {
            for (cap, pos) in re_include
                .captures_iter(&module.contents.clone())
                .zip(re_include.find_iter(&module.contents.clone()))
            {
                let name = cap.get(1).unwrap().as_str().to_string();
                let path = Module::get_module_path(&module.path, &name, &config.filetype.extension);
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

                    // De-duplicate includes
                    if includes.contains(&path) {
                        module.contents.replace_range(pos.range(), "");
                    } else {
                        includes.push(path.clone());
                    }
                }
                // println!("{:?}", &cap);
            }
        }
        // Stop the indexing once we no longer have any module to add,
        if to_add.len() == 0 {
            break;
        }
    }

    // Make a copy of the modules vec
    // to get a mutable copy of the entry file
    let mut modules_copy = modules.to_vec();
    let main_file = modules_copy.first_mut().unwrap();

    // Loop until all includes in the main file
    // are recursively replaced
    loop {
        let cloned_contents = main_file.contents.clone();
        match (
            re_include.captures(&cloned_contents),
            re_include.find(&cloned_contents),
        ) {
            (Some(cap), Some(pos)) => {
                let module_name = cap.get(1).unwrap().as_str().to_string();
                let path = Module::get_module_path(
                    &main_file.path,
                    &module_name,
                    &config.filetype.extension,
                );
                let module = modules.iter().find(|m| m.path == path).unwrap();

                // Inject code into the main file
                let module_contents = &format!(
                    "{:} [included {:}]\n\n{:}\n{:} [/included {:}]\n",
                    &config.filetype.comment,
                    &module_name,
                    &module.contents,
                    &config.filetype.comment,
                    &module_name
                );
                // Inject the code
                main_file
                    .contents
                    .replace_range(pos.range(), module_contents);
            }
            _ => {
                // If we haven't captured any regex,
                // that means that all includes are resolved
                break;
            }
        }
    }

    // Log the (succesful or not) result
    let success = fs::write(
        config.base_folder.join(&config.output_file),
        &main_file.contents,
    );
    match success {
        Ok(_) => {
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
        }
        Err(e) => {
            println!("Could not write output file:");
            println!("{:?}", e);
        }
    };
    true
}

fn run(matches: &ArgMatches) {
    let config = Config::new(&matches);

    // Initial compilation, if we don't want to watch the files
    let compiled = compile(&config);

    // If compilation failed AND we need to launch TIC, abort.
    // If subsequent compilations fail while TIC is already running,
    // we'll just log an error message and continue watching.
    if !compiled && config.tic_path.is_some() {
        println!("Compilation failed - Could not launch TIC-80");
        return;
    }

    // Start TIC-80
    let tic_path = config.tic_path.clone();
    let tic_process_mtx: Mutex<Option<Child>> = Mutex::new(None);
    if let Some(tic_path) = tic_path {
        let output_path = config
            .base_folder
            .join(&config.output_file)
            .to_str()
            .unwrap()
            .to_string();

        let child = Command::new(tic_path)
            .args(&[&config.game, "-code-watch", &output_path])
            .spawn()
            .expect("Failed to launch TIC-80");
        tic_process_mtx.lock().unwrap().replace(child);

        // Handle CTRL+C interruptions to exit gracefully
        let _handler = ctrlc::set_handler(move || {
            let child = tic_process_mtx.lock().unwrap().take();
            // Kill TIC-80 if it is launched
            if let Some(mut child) = child {
                let _ = child.kill();
            }
            exit(0);
        });
        // .expect("Error setting Ctrl-C handler");
    }

    // Start the watcher
    if config.watch {
        if let Err(e) = watch(&config) {
            println!("error: {:?}", e)
        }
    }
}
