use std::{fs, path::PathBuf};

use crate::{config::Config};

#[derive(Clone, Debug)]
pub struct Module {
    pub file_path: PathBuf,
    pub contents: String,
    pub injected: bool,
}

impl Module {
    pub fn new(file_path: &PathBuf, config: &Config) -> Self {
        assert!(
            file_path.is_file(),
            "file_path {:?} is not a file",
            file_path
        );
        let folder = file_path.parent().unwrap();
        let mut contents = match fs::read_to_string(file_path) {
            Ok(contents) => contents,
            Err(e) => panic!("Error reading file: {}", e),
        };

        // Rewrite includes to be relative to the root folder
        let dotted_folder = path_to_dotted(&folder.to_path_buf());
        if !dotted_folder.is_empty() {
            let reg_include = &config.filetype.regex;
            for capture in reg_include.captures_iter(&contents.clone()) {
                let cap = capture.get(1).unwrap();
                let range = cap.range();
                let to_replace = &format!("{dotted_folder}.{:}", cap.as_str());
                contents.replace_range(range, to_replace);
            }
        }

        let module = Module {
            file_path: file_path.clone(),
            contents,
            injected: false,
        };
        module
    }

    pub fn has_module(modules: &Vec<Module>, file_path: &PathBuf) -> bool {
        modules
            .iter()
            .find(|m| {
                &m.file_path == file_path
            })
            .is_some()
    }
}

fn path_to_dotted(path: &PathBuf) -> String {
    let mut parts = path.iter().map(|p| p.to_string_lossy()).collect::<Vec<_>>();
    if parts.is_empty() {
        return "".to_string();
    }
    parts.remove(0);
    
    parts.join(".")
}

pub fn dotted_to_path(dots: &str, config: &Config) -> PathBuf {
    let mut path = config.base_folder.clone();
    for part in dots.split('.') {
        path.push(part);
    }
    path.set_extension(&config.filetype.extension);
    path
}
